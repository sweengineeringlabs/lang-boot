# Rustboot Monitoring Stack

This directory contains a complete monitoring stack for Rustboot applications using Prometheus, Grafana, Loki, and Alertmanager.

## Quick Start

```bash
# Start the monitoring stack
docker-compose up -d

# Access the services
# Grafana: http://localhost:3000 (admin/rustboot)
# Prometheus: http://localhost:9090
# Alertmanager: http://localhost:9093
# Loki: http://localhost:3100
```

## Components

### Prometheus
- **Port**: 9090
- **Purpose**: Metrics collection, alerting rules
- **Config**: `prometheus/prometheus.yml`
- **Alert Rules**: `prometheus/rules/alerts.yml`

### Grafana
- **Port**: 3000
- **Credentials**: admin/rustboot
- **Purpose**: Visualization, dashboards
- **Dashboards**: `grafana/dashboards/`
- **Datasources**: `grafana/datasources.yml`

### Alertmanager
- **Port**: 9093
- **Purpose**: Alert routing and notifications
- **Config**: `alertmanager/alertmanager.yml`

### Loki
- **Port**: 3100
- **Purpose**: Log aggregation
- **Config**: `loki/loki-config.yml`

### Promtail
- **Purpose**: Log shipping to Loki
- **Config**: `loki/promtail-config.yml`

## Included Dashboards

### Rustboot Application Overview
- Request rate and latency
- Error rates
- Circuit breaker status
- Rate limiting metrics
- Cache hit rates
- Database connection pool status
- Memory and CPU usage

## Alert Rules

### Availability Alerts
- `ServiceDown` - Service is unreachable
- `HighErrorRate` - Error rate above 5%
- `CriticalErrorRate` - Error rate above 10%

### Performance Alerts
- `HighLatency` - P95 latency above 1 second
- `VeryHighLatency` - P99 latency above 5 seconds
- `HighRequestRate` - Over 1000 req/sec

### Resource Alerts
- `HighMemoryUsage` - Memory above 2GB
- `HighCPUUsage` - CPU above 80%
- `LowDiskSpace` - Disk below 10%

### Resilience Alerts
- `CircuitBreakerOpen` - Circuit breaker tripped
- `CircuitBreakerStuck` - Circuit breaker open 10+ min
- `HighRateLimitRejections` - High rejection rate

### Database Alerts
- `DatabasePoolExhausted` - No available connections
- `HighDatabaseLatency` - P95 above 500ms

### Cache Alerts
- `LowCacheHitRate` - Hit rate below 50%

### Health Alerts
- `HealthCheckFailing` - Health check unhealthy
- `HealthCheckDegraded` - Health check degraded

## Instrumenting Your Application

Add metrics to your Rustboot application:

```rust
use prometheus::{Counter, Histogram, Registry};

// Create metrics
lazy_static! {
    static ref HTTP_REQUESTS: Counter = Counter::new(
        "http_requests_total",
        "Total HTTP requests"
    ).unwrap();

    static ref HTTP_LATENCY: Histogram = Histogram::with_opts(
        HistogramOpts::new(
            "http_request_duration_seconds",
            "HTTP request latency"
        )
    ).unwrap();
}

// Expose metrics endpoint
async fn metrics() -> String {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    encoder.encode_to_string(&metric_families).unwrap()
}
```

## Customization

### Adding New Alert Rules

Edit `prometheus/rules/alerts.yml`:

```yaml
- alert: CustomAlert
  expr: your_metric > threshold
  for: 5m
  labels:
    severity: warning
  annotations:
    summary: "Custom alert fired"
```

### Adding Notification Channels

Edit `alertmanager/alertmanager.yml` to add:

- Slack notifications
- Email notifications
- PagerDuty integration
- Webhook endpoints

### Adding New Dashboards

Place JSON dashboard files in `grafana/dashboards/` and they will be automatically provisioned.

## Production Recommendations

1. **Secure Grafana**: Change default password
2. **Enable HTTPS**: Use reverse proxy with TLS
3. **Configure retention**: Set appropriate data retention
4. **Scale Prometheus**: Use federation for large deployments
5. **Backup data**: Regularly backup Prometheus and Grafana data
6. **Use remote storage**: Consider Thanos or Cortex for long-term storage

## Troubleshooting

### Prometheus not scraping
- Check target status at http://localhost:9090/targets
- Verify network connectivity between containers
- Check Prometheus logs: `docker-compose logs prometheus`

### Grafana dashboard not loading
- Verify datasource connectivity
- Check Grafana logs: `docker-compose logs grafana`
- Ensure Prometheus is running

### Alerts not firing
- Check Prometheus rules at http://localhost:9090/rules
- Verify alert expression in Prometheus UI
- Check Alertmanager at http://localhost:9093

### Logs not appearing in Loki
- Check Promtail logs: `docker-compose logs promtail`
- Verify log file paths in promtail config
- Check Loki at http://localhost:3100/ready
