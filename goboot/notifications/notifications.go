// Package notifications provides notification utilities for the goboot framework.
//
// This module provides:
//   - API layer: Channel, Notification, Sender interfaces
//   - Core layer: NotificationService, LogSender, WebhookSender
//
// Example:
//
//	import "dev.engineeringlabs/goboot/notifications"
//
//	service := notifications.NewNotificationService()
//
//	// Register senders
//	service.RegisterSender(notifications.NewLogSender(notifications.ChannelEmail))
//
//	// Send notification
//	notification := notifications.NewNotification(
//	    notifications.ChannelEmail,
//	    "user@example.com",
//	    "Welcome!",
//	    "Welcome to our service!",
//	)
//	result, _ := service.Send(ctx, notification)
//
//	// Using builder
//	email := notifications.NewEmailNotification("user@example.com", "Hello", "Body").
//	    WithPriority(notifications.PriorityHigh).
//	    Build()
package notifications

import (
	"dev.engineeringlabs/goboot/notifications/api"
	"dev.engineeringlabs/goboot/notifications/core"
)

// Re-export API types
type (
	// Channel represents a notification channel.
	Channel = api.Channel
	// Priority represents notification priority.
	Priority = api.Priority
	// Notification represents a notification.
	Notification = api.Notification
	// SendResult represents the result of sending.
	SendResult = api.SendResult
	// Sender is the interface for senders.
	Sender = api.Sender
	// NotificationService is the interface for notification services.
	NotificationService = api.NotificationService
	// Template represents a notification template.
	Template = api.Template
	// TemplateEngine is the interface for template rendering.
	TemplateEngine = api.TemplateEngine
	// Email represents an email notification.
	Email = api.Email
	// Attachment represents an email attachment.
	Attachment = api.Attachment
)

// Re-export API constants
const (
	ChannelEmail   = api.ChannelEmail
	ChannelSMS     = api.ChannelSMS
	ChannelPush    = api.ChannelPush
	ChannelWebhook = api.ChannelWebhook
	ChannelSlack   = api.ChannelSlack

	PriorityLow    = api.PriorityLow
	PriorityNormal = api.PriorityNormal
	PriorityHigh   = api.PriorityHigh
	PriorityUrgent = api.PriorityUrgent
)

// Re-export API functions
var NewNotification = api.NewNotification

// Re-export Core types
type (
	// DefaultNotificationService is the default notification service.
	DefaultNotificationService = core.DefaultNotificationService
	// LogSender logs notifications (for testing).
	LogSender = core.LogSender
	// WebhookSender sends to webhooks.
	WebhookSender = core.WebhookSender
	// SimpleTemplateEngine is a simple template engine.
	SimpleTemplateEngine = core.SimpleTemplateEngine
	// NotificationBuilder is a fluent builder.
	NotificationBuilder = core.NotificationBuilder
)

// Re-export Core functions
var (
	NewNotificationService  = core.NewNotificationService
	NewLogSender            = core.NewLogSender
	NewWebhookSender        = core.NewWebhookSender
	NewTemplateEngine       = core.NewTemplateEngine
	NewEmailNotification    = core.NewEmailNotification
	NewSMSNotification      = core.NewSMSNotification
	NewWebhookNotification  = core.NewWebhookNotification
)
