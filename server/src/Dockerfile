FROM rust:latest

WORKDIR /app

# Copy Cargo files from project root
COPY Cargo.toml Cargo.lock .env ./

# Copy database
# TODO: separate container for db
COPY database ./database

# Copy Rust source code
COPY src ./src

# Build
RUN cargo build --release

EXPOSE 7878

# Auto-run backend
CMD ["./target/release/server"]