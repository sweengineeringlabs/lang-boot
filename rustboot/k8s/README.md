# Kubernetes Deployment for Rustboot Applications

This directory contains Kubernetes manifests for deploying Rustboot applications using Kustomize for environment-specific configurations.

## Directory Structure

```
k8s/
├── base/                       # Base Kubernetes manifests
│   ├── deployment.yaml         # Base deployment configuration
│   ├── service.yaml           # Service definitions
│   ├── configmap.yaml         # Configuration management
│   └── kustomization.yaml     # Base kustomization
├── overlays/
│   ├── development/           # Development environment
│   │   ├── kustomization.yaml
│   │   └── ingress.yaml
│   └── production/            # Production environment
│       ├── kustomization.yaml
│       ├── hpa.yaml           # Horizontal Pod Autoscaler
│       ├── pdb.yaml           # Pod Disruption Budget
│       ├── networkpolicy.yaml # Network policies
│       ├── ingress.yaml       # Production ingress
│       └── servicemonitor.yaml # Prometheus monitoring
└── README.md                  # This file
```

## Prerequisites

- Kubernetes cluster (v1.24+)
- kubectl CLI tool
- Kustomize (v4.5+) or kubectl with built-in kustomize support
- Container registry for your images
- Optional: Prometheus Operator for monitoring (production)
- Optional: cert-manager for TLS certificates

## Features

### Base Configuration

The base configuration includes:

- **Deployment**:
  - 2 replicas by default
  - Health probes (liveness, readiness, startup)
  - Resource limits and requests
  - Security context (non-root, read-only filesystem)
  - Pod anti-affinity for better distribution

- **Service**:
  - ClusterIP service for HTTP (port 80 -> 8080)
  - Metrics endpoint (port 9090)
  - Headless service for direct pod access

- **ConfigMap**:
  - Environment-specific configuration
  - Application settings
  - Feature flags

### Development Overlay

Optimized for local development and testing:

- Single replica
- Relaxed resource limits (64Mi-256Mi memory, 50m-250m CPU)
- Debug logging enabled
- Faster health probe intervals
- CORS enabled for localhost
- Local ingress configuration
- Rate limiting disabled

### Production Overlay

Production-ready configuration with:

- **High Availability**:
  - 3 replicas minimum
  - Pod Disruption Budget (minimum 2 pods available)
  - Pod anti-affinity rules

- **Auto-scaling**:
  - HPA: 3-10 replicas
  - CPU-based scaling (70% threshold)
  - Memory-based scaling (80% threshold)
  - Smart scale-down policies

- **Security**:
  - Network policies (ingress/egress restrictions)
  - TLS/HTTPS enforced
  - Security headers configured
  - Non-root containers
  - Read-only root filesystem

- **Resource Management**:
  - 256Mi-1Gi memory
  - 200m-1000m CPU
  - Optimized for production workloads

- **Monitoring**:
  - ServiceMonitor for Prometheus
  - Pre-configured alerts
  - Metrics collection every 30s

## Quick Start

### 1. Build and Push Your Image

```bash
# Build your Rustboot application
cargo build --release

# Build Docker image
docker build -t your-registry.io/rustboot-app:v1.0.0 .

# Push to registry
docker push your-registry.io/rustboot-app:v1.0.0
```

### 2. Configure Your Environment

Update the image reference in the overlay kustomization files:

**For production** (`k8s/overlays/production/kustomization.yaml`):
```yaml
images:
- name: rustboot-app
  newName: your-registry.io/rustboot-app
  newTag: v1.0.0
```

### 3. Deploy to Development

```bash
# Preview the manifests
kubectl kustomize k8s/overlays/development

# Apply to cluster
kubectl apply -k k8s/overlays/development

# Check deployment status
kubectl get pods -n development
kubectl logs -f deployment/dev-rustboot-app -n development
```

### 4. Deploy to Production

```bash
# Preview the manifests
kubectl kustomize k8s/overlays/production

# Apply to cluster
kubectl apply -k k8s/overlays/production

# Check deployment status
kubectl get pods -n production
kubectl get hpa -n production
kubectl get pdb -n production
```

## Configuration

### Environment Variables

Configure your application through the ConfigMap. Common variables:

```yaml
APP_ENV: "production"           # Application environment
LOG_LEVEL: "info"              # Logging level
DATABASE_POOL_MAX_CONNECTIONS: "20"
RATE_LIMIT_ENABLED: "true"
METRICS_ENABLED: "true"
```

### Secrets Management

For sensitive data, create Kubernetes secrets:

```bash
# Create database secret
kubectl create secret generic rustboot-secrets \
  --from-literal=DATABASE_URL='postgres://user:pass@host/db' \
  --from-literal=API_KEY='your-api-key' \
  -n production

# Or use external secret management (recommended)
# - AWS Secrets Manager
# - HashiCorp Vault
# - Sealed Secrets
# - External Secrets Operator
```

Then reference in deployment:
```yaml
envFrom:
- secretRef:
    name: rustboot-secrets
```

### Resource Tuning

Adjust resources based on your workload:

**Development** (small footprint):
```yaml
resources:
  requests:
    memory: "64Mi"
    cpu: "50m"
  limits:
    memory: "256Mi"
    cpu: "250m"
```

**Production** (performance):
```yaml
resources:
  requests:
    memory: "256Mi"
    cpu: "200m"
  limits:
    memory: "1Gi"
    cpu: "1000m"
```

## Health Checks

The deployment uses three types of probes:

### Liveness Probe
Checks if the application is alive. If it fails, Kubernetes restarts the pod.

```yaml
livenessProbe:
  httpGet:
    path: /health/live
    port: 8080
  initialDelaySeconds: 30
  periodSeconds: 10
```

### Readiness Probe
Checks if the application is ready to serve traffic. If it fails, the pod is removed from service endpoints.

```yaml
readinessProbe:
  httpGet:
    path: /health/ready
    port: 8080
  initialDelaySeconds: 10
  periodSeconds: 5
```

### Startup Probe
For slow-starting applications. Other probes are disabled until this succeeds.

```yaml
startupProbe:
  httpGet:
    path: /health/startup
    port: 8080
  failureThreshold: 12
  periodSeconds: 5
```

**Required**: Your Rustboot application must implement these health endpoints. Use the `rustboot-health` crate:

```rust
use rustboot_health::HealthCheck;

// Implement health check endpoints at:
// GET /health/live
// GET /health/ready
// GET /health/startup
```

## Networking

### Ingress Configuration

**Development**:
- HTTP access
- Local hostname: `rustboot-dev.local`
- No TLS required

**Production**:
- HTTPS enforced
- Custom domain: `api.yourdomain.com`
- TLS certificates via cert-manager
- Rate limiting enabled
- CORS configured
- Security headers

Update the ingress host in `k8s/overlays/production/ingress.yaml`:
```yaml
spec:
  tls:
  - hosts:
    - api.yourdomain.com
```

### Network Policies

Production includes strict network policies:

**Ingress**:
- Allow from ingress controller (port 8080)
- Allow from Prometheus (port 9090)
- Allow from same namespace

**Egress**:
- DNS (port 53)
- PostgreSQL (port 5432)
- Redis (port 6379)
- Kafka (port 9092)
- RabbitMQ (port 5672)
- HTTPS external services (port 443)

Customize based on your dependencies in `k8s/overlays/production/networkpolicy.yaml`.

## Scaling

### Manual Scaling

```bash
# Scale deployment
kubectl scale deployment prod-rustboot-app --replicas=5 -n production
```

### Horizontal Pod Autoscaler (HPA)

Production includes HPA configuration:

```yaml
minReplicas: 3
maxReplicas: 10
metrics:
- type: Resource
  resource:
    name: cpu
    target:
      type: Utilization
      averageUtilization: 70
```

Monitor HPA:
```bash
kubectl get hpa -n production
kubectl describe hpa prod-rustboot-app -n production
```

### Custom Metrics

To scale based on custom metrics (e.g., HTTP requests/sec):

1. Install metrics-server and custom metrics adapter
2. Configure application to expose custom metrics
3. Update HPA configuration in `k8s/overlays/production/hpa.yaml`

## Monitoring

### Prometheus Integration

Production includes ServiceMonitor for automatic Prometheus discovery:

```yaml
apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: rustboot-app
spec:
  endpoints:
  - port: metrics
    path: /metrics
    interval: 30s
```

**Required**: Your application must expose Prometheus metrics at `/metrics` on port 9090.

Use the `rustboot-observability` crate:

```rust
use rustboot_observability::metrics;

// Metrics are automatically exposed at /metrics
```

### Pre-configured Alerts

The production overlay includes alerts for:

- High error rate (>5%)
- High response time (>1s at p95)
- Pod not ready
- High CPU usage (>80%)
- High memory usage (>90%)
- Frequent pod restarts
- Deployment replica mismatch

View alerts in Prometheus/Alertmanager:
```bash
kubectl port-forward -n monitoring svc/prometheus 9090:9090
```

## Troubleshooting

### View Logs

```bash
# All pods in namespace
kubectl logs -l app=rustboot-app -n production

# Specific pod
kubectl logs pod/prod-rustboot-app-xxx -n production

# Follow logs
kubectl logs -f deployment/prod-rustboot-app -n production

# Previous container (if pod restarted)
kubectl logs pod/prod-rustboot-app-xxx -n production --previous
```

### Check Pod Status

```bash
# List pods
kubectl get pods -n production

# Detailed pod info
kubectl describe pod prod-rustboot-app-xxx -n production

# Check events
kubectl get events -n production --sort-by='.lastTimestamp'
```

### Debug Network Issues

```bash
# Check service endpoints
kubectl get endpoints -n production

# Test service connectivity
kubectl run -it --rm debug --image=busybox --restart=Never -n production -- \
  wget -O- http://prod-rustboot-app.production.svc.cluster.local/health

# Check network policies
kubectl describe networkpolicy -n production
```

### Resource Issues

```bash
# Check resource usage
kubectl top pods -n production
kubectl top nodes

# Describe HPA
kubectl describe hpa prod-rustboot-app -n production

# Check pod disruption budget
kubectl get pdb -n production
```

### Access Application

```bash
# Port forward to local machine
kubectl port-forward svc/prod-rustboot-app 8080:80 -n production

# Access via curl
curl http://localhost:8080/health
```

## Best Practices

### 1. Image Management
- Use semantic versioning for image tags
- Never use `latest` tag in production
- Scan images for vulnerabilities
- Use multi-stage builds for smaller images

### 2. Configuration
- Use ConfigMaps for non-sensitive data
- Use Secrets for sensitive data
- Never commit secrets to Git
- Use external secret management in production

### 3. Security
- Run as non-root user
- Use read-only root filesystem
- Drop all capabilities
- Enable network policies
- Use PodSecurityPolicies/PodSecurityStandards
- Regular security scanning

### 4. Monitoring
- Implement all health check endpoints
- Expose Prometheus metrics
- Set up meaningful alerts
- Monitor resource usage
- Track application-specific metrics

### 5. High Availability
- Use multiple replicas (minimum 3 in production)
- Configure pod disruption budgets
- Use pod anti-affinity
- Deploy across multiple zones/nodes
- Test failover scenarios

### 6. Resource Management
- Set appropriate resource requests and limits
- Monitor actual usage and adjust
- Use HPA for automatic scaling
- Consider cluster autoscaler
- Plan for peak loads

### 7. Deployment Strategy
- Use rolling updates
- Set appropriate `maxSurge` and `maxUnavailable`
- Test in development first
- Use blue-green or canary for critical services
- Have rollback plan ready

## Advanced Topics

### GitOps Integration

Use ArgoCD or Flux for GitOps workflow:

```yaml
# ArgoCD Application
apiVersion: argoproj.io/v1alpha1
kind: Application
metadata:
  name: rustboot-app
spec:
  source:
    repoURL: https://github.com/yourorg/rustboot
    path: k8s/overlays/production
    targetRevision: main
  destination:
    server: https://kubernetes.default.svc
    namespace: production
  syncPolicy:
    automated:
      prune: true
      selfHeal: true
```

### Service Mesh Integration

For service mesh (Istio, Linkerd):

1. Add sidecar injection label to namespace
2. Configure VirtualServices and DestinationRules
3. Implement mTLS between services
4. Use mesh metrics instead of direct Prometheus scraping

### Multi-Region Deployment

For multi-region setups:

1. Create region-specific overlays
2. Use external-dns for DNS management
3. Configure global load balancing
4. Implement database replication
5. Consider data sovereignty requirements

## Maintenance

### Update Deployment

```bash
# Update image
cd k8s/overlays/production
kustomize edit set image rustboot-app=your-registry.io/rustboot-app:v1.1.0

# Apply changes
kubectl apply -k .

# Monitor rollout
kubectl rollout status deployment/prod-rustboot-app -n production
```

### Rollback

```bash
# Rollback to previous version
kubectl rollout undo deployment/prod-rustboot-app -n production

# Rollback to specific revision
kubectl rollout undo deployment/prod-rustboot-app --to-revision=2 -n production

# View rollout history
kubectl rollout history deployment/prod-rustboot-app -n production
```

### Cleanup

```bash
# Delete development deployment
kubectl delete -k k8s/overlays/development

# Delete production deployment
kubectl delete -k k8s/overlays/production

# Delete namespace (careful!)
kubectl delete namespace production
```

## Support and Contributing

For issues or questions:
- Check the Rustboot documentation
- Review Kubernetes documentation
- Open an issue on GitHub
- Contact the DevOps team

## License

Same as the Rustboot framework.
