// Package core contains the implementation details for the notifications module.
package core

import (
	"context"
	"fmt"
	"strings"
	"sync"
	"time"

	"dev.engineeringlabs/goboot/notifications/api"
)

// DefaultNotificationService is the default notification service.
type DefaultNotificationService struct {
	senders map[api.Channel]api.Sender
	mu      sync.RWMutex
}

// NewNotificationService creates a new DefaultNotificationService.
func NewNotificationService() *DefaultNotificationService {
	return &DefaultNotificationService{
		senders: make(map[api.Channel]api.Sender),
	}
}

// RegisterSender registers a sender.
func (s *DefaultNotificationService) RegisterSender(sender api.Sender) {
	s.mu.Lock()
	s.senders[sender.Channel()] = sender
	s.mu.Unlock()
}

// Send sends a notification.
func (s *DefaultNotificationService) Send(ctx context.Context, notification *api.Notification) (*api.SendResult, error) {
	s.mu.RLock()
	sender, ok := s.senders[notification.Channel]
	s.mu.RUnlock()

	if !ok {
		return nil, fmt.Errorf("no sender registered for channel: %s", notification.Channel)
	}

	return sender.Send(ctx, notification)
}

// SendBatch sends multiple notifications.
func (s *DefaultNotificationService) SendBatch(ctx context.Context, notifications []*api.Notification) ([]*api.SendResult, error) {
	results := make([]*api.SendResult, len(notifications))
	var wg sync.WaitGroup
	var mu sync.Mutex

	for i, n := range notifications {
		wg.Add(1)
		go func(idx int, notification *api.Notification) {
			defer wg.Done()
			result, err := s.Send(ctx, notification)
			mu.Lock()
			if err != nil {
				results[idx] = &api.SendResult{
					NotificationID: notification.ID,
					Channel:        notification.Channel,
					Success:        false,
					Error:          err,
				}
			} else {
				results[idx] = result
			}
			mu.Unlock()
		}(i, n)
	}

	wg.Wait()
	return results, nil
}

// LogSender is a sender that logs notifications (for testing).
type LogSender struct {
	channel api.Channel
	logs    []*api.Notification
	mu      sync.Mutex
}

// NewLogSender creates a new LogSender.
func NewLogSender(channel api.Channel) *LogSender {
	return &LogSender{
		channel: channel,
		logs:    make([]*api.Notification, 0),
	}
}

// Channel returns the channel.
func (s *LogSender) Channel() api.Channel {
	return s.channel
}

// Send logs the notification.
func (s *LogSender) Send(ctx context.Context, notification *api.Notification) (*api.SendResult, error) {
	s.mu.Lock()
	s.logs = append(s.logs, notification)
	s.mu.Unlock()

	return &api.SendResult{
		NotificationID: notification.ID,
		Channel:        s.channel,
		Success:        true,
		SentAt:         time.Now(),
	}, nil
}

// Logs returns all logged notifications.
func (s *LogSender) Logs() []*api.Notification {
	s.mu.Lock()
	defer s.mu.Unlock()
	result := make([]*api.Notification, len(s.logs))
	copy(result, s.logs)
	return result
}

// Clear clears the logs.
func (s *LogSender) Clear() {
	s.mu.Lock()
	s.logs = make([]*api.Notification, 0)
	s.mu.Unlock()
}

// WebhookSender sends notifications to webhooks.
type WebhookSender struct {
	httpClient HTTPClient
}

// HTTPClient is the interface for HTTP clients.
type HTTPClient interface {
	Post(ctx context.Context, url string, body []byte) error
}

// NewWebhookSender creates a new WebhookSender.
func NewWebhookSender(client HTTPClient) *WebhookSender {
	return &WebhookSender{httpClient: client}
}

// Channel returns the channel.
func (s *WebhookSender) Channel() api.Channel {
	return api.ChannelWebhook
}

// Send sends a webhook notification.
func (s *WebhookSender) Send(ctx context.Context, notification *api.Notification) (*api.SendResult, error) {
	// The recipient is the webhook URL
	body := fmt.Sprintf(`{"subject":"%s","body":"%s"}`, notification.Subject, notification.Body)
	
	err := s.httpClient.Post(ctx, notification.Recipient, []byte(body))
	if err != nil {
		return &api.SendResult{
			NotificationID: notification.ID,
			Channel:        api.ChannelWebhook,
			Success:        false,
			Error:          err,
		}, err
	}

	return &api.SendResult{
		NotificationID: notification.ID,
		Channel:        api.ChannelWebhook,
		Success:        true,
		SentAt:         time.Now(),
	}, nil
}

// SimpleTemplateEngine is a simple template engine.
type SimpleTemplateEngine struct{}

// NewTemplateEngine creates a new SimpleTemplateEngine.
func NewTemplateEngine() *SimpleTemplateEngine {
	return &SimpleTemplateEngine{}
}

// Render renders a template.
func (e *SimpleTemplateEngine) Render(template *api.Template, data map[string]any) (string, string, error) {
	subject := template.Subject
	body := template.Body

	for key, value := range data {
		placeholder := "{{" + key + "}}"
		subject = strings.ReplaceAll(subject, placeholder, fmt.Sprint(value))
		body = strings.ReplaceAll(body, placeholder, fmt.Sprint(value))
	}

	return subject, body, nil
}

// NotificationBuilder is a fluent builder for notifications.
type NotificationBuilder struct {
	notification *api.Notification
}

// NewEmailNotification creates an email notification builder.
func NewEmailNotification(to, subject, body string) *NotificationBuilder {
	return &NotificationBuilder{
		notification: api.NewNotification(api.ChannelEmail, to, subject, body),
	}
}

// NewSMSNotification creates an SMS notification builder.
func NewSMSNotification(phone, message string) *NotificationBuilder {
	return &NotificationBuilder{
		notification: api.NewNotification(api.ChannelSMS, phone, "", message),
	}
}

// NewWebhookNotification creates a webhook notification builder.
func NewWebhookNotification(url, subject, body string) *NotificationBuilder {
	return &NotificationBuilder{
		notification: api.NewNotification(api.ChannelWebhook, url, subject, body),
	}
}

// WithData adds data.
func (b *NotificationBuilder) WithData(key string, value any) *NotificationBuilder {
	b.notification.WithData(key, value)
	return b
}

// WithPriority sets the priority.
func (b *NotificationBuilder) WithPriority(p api.Priority) *NotificationBuilder {
	b.notification.WithPriority(p)
	return b
}

// Build builds the notification.
func (b *NotificationBuilder) Build() *api.Notification {
	return b.notification
}
