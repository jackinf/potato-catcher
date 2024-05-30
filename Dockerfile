# Start from a lightweight base image
FROM rust:1.76 as builder

# Install wasm-bindgen-cli
RUN cargo install wasm-bindgen-cli

# Set the Current Working Directory inside the container
WORKDIR /app

# Copy the source code into the container
COPY . .

# Build the WASM target
RUN rustup target add wasm32-unknown-unknown
RUN cargo build --target wasm32-unknown-unknown

# Run wasm-bindgen to generate bindings
RUN wasm-bindgen target/wasm32-unknown-unknown/debug/potato-catcher.wasm --out-dir static --web

# Use a smaller base image for the final stage
FROM golang:1.22-alpine

# Set the Current Working Directory inside the container
WORKDIR /app

# Copy the Go module files
COPY go.mod .

# Copy the generated static files and Go source code from the builder stage
COPY --from=builder /app/static ./static
COPY --from=builder /app/assets ./static
COPY --from=builder /app/main.go .

# Build the Go app
RUN go build -o main .

# Expose port 8080 to the outside world
EXPOSE 8080

# Command to run the executable
CMD ["./main"]
