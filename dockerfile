# ----------- Build the Rust application -----------
FROM rust:1.81 AS builder
WORKDIR /app

# Copy Cargo manifests first
COPY Cargo.toml Cargo.lock ./

# Copy source files
COPY src ./src

# Build in release mode
RUN cargo build --release

# ----------- Create a lightweight runtime image -----------
FROM debian:bookworm-slim AS runner
WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    # ssl dep
    libssl-dev \
    # certifcate for https requests
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from builder
COPY --from=builder /app/target/release/eth_fetcher /app/eth_fetcher

# Copy the .env file
COPY .env /app/.env

# Expose port from API_PORT in .env file
EXPOSE 3000

CMD ["/app/eth_fetcher"]
