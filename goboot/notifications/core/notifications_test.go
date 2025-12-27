package core

import (
	"context"
	"testing"
	"time"

	"dev.engineeringlabs/goboot/notifications/api"
)

func TestDefaultNotificationService_Send(t *testing.T) {
	service := NewNotificationService()
	logSender := NewLogSender(api.ChannelEmail)
	service.RegisterSender(logSender)

	ctx := context.Background()
	notification := api.NewNotification(api.ChannelEmail, "test@example.com", "Hello", "World")

	result, err := service.Send(ctx, notification)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	if !result.Success {
		t.Error("Send should succeed")
	}
	if result.Channel != api.ChannelEmail {
		t.Error("Wrong channel in result")
	}

	logs := logSender.Logs()
	if len(logs) != 1 {
		t.Errorf("Expected 1 log, got %d", len(logs))
	}
}

func TestDefaultNotificationService_SendNoSender(t *testing.T) {
	service := NewNotificationService()
	ctx := context.Background()
	notification := api.NewNotification(api.ChannelSMS, "+1234567890", "", "Hello")

	_, err := service.Send(ctx, notification)
	if err == nil {
		t.Error("Expected error for missing sender")
	}
}

func TestDefaultNotificationService_SendBatch(t *testing.T) {
	service := NewNotificationService()
	service.RegisterSender(NewLogSender(api.ChannelEmail))

	ctx := context.Background()
	notifications := []*api.Notification{
		api.NewNotification(api.ChannelEmail, "a@example.com", "A", "Body A"),
		api.NewNotification(api.ChannelEmail, "b@example.com", "B", "Body B"),
		api.NewNotification(api.ChannelEmail, "c@example.com", "C", "Body C"),
	}

	results, err := service.SendBatch(ctx, notifications)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	if len(results) != 3 {
		t.Errorf("Expected 3 results, got %d", len(results))
	}

	successCount := 0
	for _, r := range results {
		if r.Success {
			successCount++
		}
	}
	if successCount != 3 {
		t.Errorf("Expected 3 successes, got %d", successCount)
	}
}

func TestLogSender(t *testing.T) {
	sender := NewLogSender(api.ChannelEmail)

	if sender.Channel() != api.ChannelEmail {
		t.Error("Wrong channel")
	}

	ctx := context.Background()
	notification := api.NewNotification(api.ChannelEmail, "test@example.com", "Test", "Body")

	result, err := sender.Send(ctx, notification)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	if !result.Success {
		t.Error("Should succeed")
	}

	logs := sender.Logs()
	if len(logs) != 1 {
		t.Error("Should have 1 log")
	}

	sender.Clear()
	if len(sender.Logs()) != 0 {
		t.Error("Should be empty after clear")
	}
}

func TestNotification_WithData(t *testing.T) {
	notification := api.NewNotification(api.ChannelEmail, "test@example.com", "Test", "Body").
		WithData("key1", "value1").
		WithData("key2", 42)

	if notification.Data["key1"] != "value1" {
		t.Error("Missing key1")
	}
	if notification.Data["key2"] != 42 {
		t.Error("Missing key2")
	}
}

func TestNotification_WithPriority(t *testing.T) {
	notification := api.NewNotification(api.ChannelEmail, "test@example.com", "Test", "Body").
		WithPriority(api.PriorityHigh)

	if notification.Priority != api.PriorityHigh {
		t.Error("Wrong priority")
	}
}

func TestNotification_ScheduleAt(t *testing.T) {
	future := time.Now().Add(time.Hour)
	notification := api.NewNotification(api.ChannelEmail, "test@example.com", "Test", "Body").
		ScheduleAt(future)

	if notification.ScheduledAt == nil {
		t.Error("Should be scheduled")
	}
	if !notification.ScheduledAt.Equal(future) {
		t.Error("Wrong scheduled time")
	}
}

func TestSimpleTemplateEngine(t *testing.T) {
	engine := NewTemplateEngine()

	template := &api.Template{
		Subject: "Hello {{name}}",
		Body:    "Welcome {{name}}, your code is {{code}}.",
	}

	data := map[string]any{
		"name": "John",
		"code": "ABC123",
	}

	subject, body, err := engine.Render(template, data)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}

	expectedSubject := "Hello John"
	if subject != expectedSubject {
		t.Errorf("Expected '%s', got '%s'", expectedSubject, subject)
	}

	expectedBody := "Welcome John, your code is ABC123."
	if body != expectedBody {
		t.Errorf("Expected '%s', got '%s'", expectedBody, body)
	}
}

func TestNotificationBuilders(t *testing.T) {
	t.Run("EmailNotification", func(t *testing.T) {
		email := NewEmailNotification("test@example.com", "Subject", "Body").
			WithPriority(api.PriorityHigh).
			Build()

		if email.Channel != api.ChannelEmail {
			t.Error("Wrong channel")
		}
		if email.Recipient != "test@example.com" {
			t.Error("Wrong recipient")
		}
		if email.Priority != api.PriorityHigh {
			t.Error("Wrong priority")
		}
	})

	t.Run("SMSNotification", func(t *testing.T) {
		sms := NewSMSNotification("+1234567890", "Hello").Build()

		if sms.Channel != api.ChannelSMS {
			t.Error("Wrong channel")
		}
		if sms.Recipient != "+1234567890" {
			t.Error("Wrong recipient")
		}
	})

	t.Run("WebhookNotification", func(t *testing.T) {
		webhook := NewWebhookNotification("https://example.com/hook", "Event", "Payload").Build()

		if webhook.Channel != api.ChannelWebhook {
			t.Error("Wrong channel")
		}
		if webhook.Recipient != "https://example.com/hook" {
			t.Error("Wrong recipient")
		}
	})
}
