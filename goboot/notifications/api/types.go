// Package api contains the public interfaces and types for the notifications module.
package api

import (
	"context"
	"time"
)

// Channel represents a notification channel.
type Channel string

const (
	// ChannelEmail for email notifications.
	ChannelEmail Channel = "email"
	// ChannelSMS for SMS notifications.
	ChannelSMS Channel = "sms"
	// ChannelPush for push notifications.
	ChannelPush Channel = "push"
	// ChannelWebhook for webhook notifications.
	ChannelWebhook Channel = "webhook"
	// ChannelSlack for Slack notifications.
	ChannelSlack Channel = "slack"
)

// Priority represents notification priority.
type Priority int

const (
	// PriorityLow for low priority.
	PriorityLow Priority = iota
	// PriorityNormal for normal priority.
	PriorityNormal
	// PriorityHigh for high priority.
	PriorityHigh
	// PriorityUrgent for urgent priority.
	PriorityUrgent
)

// Notification represents a notification.
type Notification struct {
	ID          string
	Channel     Channel
	Recipient   string
	Subject     string
	Body        string
	Data        map[string]any
	Priority    Priority
	ScheduledAt *time.Time
	CreatedAt   time.Time
}

// NewNotification creates a new Notification.
func NewNotification(channel Channel, recipient, subject, body string) *Notification {
	return &Notification{
		Channel:   channel,
		Recipient: recipient,
		Subject:   subject,
		Body:      body,
		Data:      make(map[string]any),
		Priority:  PriorityNormal,
		CreatedAt: time.Now(),
	}
}

// WithData adds data to the notification.
func (n *Notification) WithData(key string, value any) *Notification {
	n.Data[key] = value
	return n
}

// WithPriority sets the priority.
func (n *Notification) WithPriority(p Priority) *Notification {
	n.Priority = p
	return n
}

// ScheduleAt schedules the notification.
func (n *Notification) ScheduleAt(t time.Time) *Notification {
	n.ScheduledAt = &t
	return n
}

// SendResult represents the result of sending a notification.
type SendResult struct {
	NotificationID string
	Channel        Channel
	Success        bool
	Error          error
	SentAt         time.Time
	ExternalID     string // ID from external service
}

// Sender is the interface for notification senders.
type Sender interface {
	// Channel returns the supported channel.
	Channel() Channel

	// Send sends a notification.
	Send(ctx context.Context, notification *Notification) (*SendResult, error)
}

// NotificationService is the interface for notification services.
type NotificationService interface {
	// Send sends a notification.
	Send(ctx context.Context, notification *Notification) (*SendResult, error)

	// SendBatch sends multiple notifications.
	SendBatch(ctx context.Context, notifications []*Notification) ([]*SendResult, error)

	// RegisterSender registers a sender for a channel.
	RegisterSender(sender Sender)
}

// Template represents a notification template.
type Template struct {
	ID       string
	Name     string
	Channel  Channel
	Subject  string
	Body     string
	Vars     []string
}

// TemplateEngine is the interface for template rendering.
type TemplateEngine interface {
	// Render renders a template with data.
	Render(template *Template, data map[string]any) (subject, body string, err error)
}

// Email represents an email notification.
type Email struct {
	To          []string
	CC          []string
	BCC         []string
	From        string
	Subject     string
	Body        string
	HTMLBody    string
	Attachments []Attachment
}

// Attachment represents an email attachment.
type Attachment struct {
	Filename    string
	ContentType string
	Data        []byte
}
