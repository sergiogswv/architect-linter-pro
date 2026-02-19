# Architect Linter GitLab CI Runner
# Multi-stage build for a smaller final image

FROM rust:1.75-slim as builder

WORKDIR /build

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    git \
    && rm -rf /var/lib/apt/lists/*

# Copy Cargo files first for better caching
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build release binary
RUN cargo build --release

# Final stage - minimal image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    git \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /build/target/release/architect-linter-pro /usr/local/bin/architect-linter-pro

# Make it executable
RUN chmod +x /usr/local/bin/architect-linter-pro

# Set up working directory
WORKDIR /workspace

# Default command
ENTRYPOINT ["architect-linter-pro"]
CMD ["--help"]
