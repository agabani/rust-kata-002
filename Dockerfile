# 1: Build
FROM rust:1.49.0 as builder
WORKDIR /usr/src

# 1a: Prepare for static linking

# 1b: Download and compile Rust dependencies (and store as a separate Docker layer)
RUN USER=root cargo new --bin app
WORKDIR /usr/src/app
COPY Cargo.lock Cargo.toml ./
RUN cargo build --release && \
    rm -rf src/

# 1c: Build the exe using the actual source code
COPY src/ ./src/
RUN cargo build --release

# 2: Copy the exe and extra files to an empty Docker image
FROM gcr.io/distroless/cc
COPY .env .
COPY --from=builder /usr/src/app/target/release/rust-kata-002 .
CMD ["./rust-kata-002"]
