# -------------------------
# STAGE 1: Builder
# -------------------------
FROM rust:alpine AS builder

# Install build dependencies required for compiling Rust on Alpine (musl libc)
RUN apk add --no-cache musl-dev

# Set up working directory
WORKDIR /usr/src/vyzorix_cli

# We copy the manifest files first to cache dependencies
COPY Cargo.toml ./
# Create dummy source to build dependencies and cache them
RUN mkdir src && \
    echo 'fn main() { println!("Dummy!"); }' > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Now copy the actual source code
COPY src ./src

# Touch main.rs to force cargo to recompile the project (avoiding the cached dummy)
RUN touch src/main.rs

# Build the ultra-optimized release binary
RUN cargo build --release


# -------------------------
# STAGE 2: Runtime (Minimal)
# -------------------------
FROM alpine:latest

# Security: Create a non-root system user and group
# This ensures the container runs without elevated privileges, safe for any VM or Kubernetes
RUN addgroup -S vyzorix && adduser -S vyzorix -G vyzorix

# Create a working workspace directory and grant ownership to the non-root user
WORKDIR /workspace
RUN chown vyzorix:vyzorix /workspace

# Copy the compiled binary from the builder stage
# It's statically linked due to Alpine's musl, so it relies on no external dependencies
COPY --from=builder /usr/src/vyzorix_cli/target/release/vyzorix /usr/local/bin/vyzorix

# Drop root privileges by switching to our new user
USER vyzorix

# Set default environment variables (can be overridden at runtime)
ENV VYZORIX_DEV_MODE=false
ENV RUST_LOG=info

# Expose Vyzorix as the entrypoint
ENTRYPOINT ["vyzorix"]
# Default command when no arguments are passed
CMD ["--help"]
