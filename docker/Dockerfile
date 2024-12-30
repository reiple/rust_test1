# Use an official Rust image as the base image
FROM rust:1.83-alpine AS builder

# Install build dependencies
# RUN apk add --no-cache musl-dev openssl-dev build-base

# Set the working directory
WORKDIR /usr/src/app

# Copy Cargo.toml and Cargo.lock to the container
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY . ./

# Build dependencies only
# RUN cargo build --release && rm -rf src
RUN cargo build --release

# Use a minimal base image for the final stage
FROM alpine:latest

# Install necessary runtime dependencies
# RUN apk add --no-cache libssl1.1

# Set the working directory
WORKDIR /usr/src/app

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/test1 ./

# Expose the necessary port (optional, if your app uses a specific port)
# EXPOSE 8080

# Set the binary as the entry point
ENTRYPOINT ["./test1"]
