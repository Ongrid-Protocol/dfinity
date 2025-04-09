# Use the official Rust image as a base
FROM rust:1.75-slim as builder

# Install necessary build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release

# Use a smaller base image for the final stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/bootstrap_node_backend /usr/local/bin/

# Set environment variables
ENV RUST_LOG=info
ENV DFX_NETWORK=local
ENV DFX_HOST=192.168.100.172:49517

# Expose the port
EXPOSE 49517

# Run the application
CMD ["bootstrap_node_backend"] 