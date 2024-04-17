# Builder stage
FROM rust:latest as builder

# Create app directory
WORKDIR /usr/src/peersity-api

# Copy the source code
COPY . .

# Build the application
RUN cargo install --path .

# Final stage, using Ubuntu for a newer GLIBC version
FROM ubuntu:latest

# Install dependencies required by the application and OpenSSL
RUN apt-get update && apt-get install -y openssl libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /usr/local/cargo/bin/peersity-api /usr/local/bin/peersity-api

# Set environment variables
ENV RUST_LOG=info

# Expose the port the API runs on
EXPOSE 8080

# Command to run the application
CMD ["peersity-api"]
