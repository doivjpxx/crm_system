# Use the official Rust image as the base image
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock .env ./

RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build the dependencies separately to take advantage of Docker layer caching
RUN cargo build --release

# Copy the source code to the container
COPY src src

# Build the application
RUN touch src/main.rs
RUN cargo build --release

# Create a new stage for the final image
FROM debian:bookworm-slim

# Install system dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev

# Set the working directory inside the container
WORKDIR /app

# Copy the built binary from the previous stage
COPY --from=builder /app/target/release/crm_system .

# Expose the port that the application listens on
EXPOSE 3000

# Set the command to run the application
CMD ["./crm_system"]