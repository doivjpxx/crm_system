# Use the official Rust image as the base image
FROM rust:1.80.1-slim-bullseye as builder

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev

# Build the dependencies separately to take advantage of Docker layer caching
RUN cargo build --release || true

# Copy the source code to the container
COPY src ./src

RUN cargo install sqlx-cli --no-default-features --features postgres

RUN cargo sqlx prepare

# Build the application
RUN cargo build --release

# Create a new stage for the final image
FROM debian:bullseye

# Install system dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev \
    libpq-dev

# Set the working directory inside the container
WORKDIR /app

# Copy the built binary from the previous stage
COPY --from=builder /app/target/release/crm_system .

# Expose the port that the application listens on
EXPOSE 3000

# Set the command to run the application
CMD ["./crm_system"]