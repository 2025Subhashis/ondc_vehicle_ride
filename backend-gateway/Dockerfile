# --- Build Stage ---
FROM rust:1.80-slim-bookworm AS builder

# Install necessary build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

# Build the application in release mode
RUN cargo build --release

# --- Run Stage ---
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
# Copy the binary from the build stage
COPY --from=builder /app/target/release/backend-gateway /app/backend-gateway

EXPOSE 8080

CMD ["./backend-gateway"]
