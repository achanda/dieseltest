FROM rust:1.87 as builder

WORKDIR /usr/src/app
COPY . .

# Install diesel_cli with version specification and --locked flag
RUN cargo install diesel_cli@2.1.1 --no-default-features --features postgres --locked

# Build the application
RUN cargo build --release

# Use the same Rust image for runtime to avoid glibc compatibility issues
FROM rust:1.87-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq5 \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/rust_app /usr/local/bin/
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/
COPY --from=builder /usr/src/app/migrations /usr/local/bin/migrations

WORKDIR /usr/local/bin

# Run diesel migrations and then start the app
CMD ["bash", "-c", "diesel migration run && rust_app"]