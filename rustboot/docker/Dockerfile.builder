# Multi-stage Dockerfile for building Rustboot applications
# This builder image compiles Rust applications with optimal caching

# Stage 1: Build dependencies (cached layer)
FROM rust:1.82-slim-bookworm AS chef
WORKDIR /build

# Install cargo-chef for dependency caching
RUN cargo install cargo-chef

# Stage 2: Prepare recipe for dependency caching
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3: Build dependencies
FROM chef AS dependencies
COPY --from=planner /build/recipe.json recipe.json

# Build dependencies only - this layer will be cached unless dependencies change
RUN cargo chef cook --release --recipe-path recipe.json

# Stage 4: Build the application
FROM rust:1.82-slim-bookworm AS builder
WORKDIR /build

# Install required build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy cached dependencies from previous stage
COPY --from=dependencies /build/target target
COPY --from=dependencies /usr/local/cargo /usr/local/cargo

# Copy source code
COPY . .

# Build the application in release mode
# Set the binary name via build arg (default: app)
ARG BINARY_NAME=rustboot-app
ENV BINARY_NAME=${BINARY_NAME}

# Build the specific binary or entire workspace
RUN cargo build --release

# The binary will be at /build/target/release/${BINARY_NAME}
# For workspace members, binaries are in /build/target/release/

# Stage 5: Create minimal runtime image
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd -r appuser && useradd -r -g appuser appuser

# Set working directory
WORKDIR /app

# Copy binary from builder
ARG BINARY_NAME=rustboot-app
COPY --from=builder /build/target/release/${BINARY_NAME} /app/app

# Set ownership
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose default port (can be overridden)
EXPOSE 8080

# Health check (adjust based on your app's health endpoint)
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Run the binary
CMD ["/app/app"]
