#!/bin/sh
# Health check script for Rustboot applications
# This can be used as a custom health check in Docker containers

set -e

# Configuration
HEALTH_ENDPOINT="${HEALTH_ENDPOINT:-http://localhost:8080/health}"
TIMEOUT="${HEALTH_TIMEOUT:-3}"
MAX_RETRIES="${HEALTH_MAX_RETRIES:-3}"

# Function to check health
check_health() {
    if command -v curl > /dev/null 2>&1; then
        curl -f --max-time "$TIMEOUT" "$HEALTH_ENDPOINT" > /dev/null 2>&1
    elif command -v wget > /dev/null 2>&1; then
        wget --timeout="$TIMEOUT" --tries=1 --spider "$HEALTH_ENDPOINT" > /dev/null 2>&1
    else
        echo "Error: Neither curl nor wget found"
        return 1
    fi
}

# Retry logic
attempt=1
while [ $attempt -le $MAX_RETRIES ]; do
    if check_health; then
        echo "Health check passed"
        exit 0
    fi

    if [ $attempt -lt $MAX_RETRIES ]; then
        echo "Health check failed (attempt $attempt/$MAX_RETRIES), retrying..."
        sleep 1
    fi

    attempt=$((attempt + 1))
done

echo "Health check failed after $MAX_RETRIES attempts"
exit 1
