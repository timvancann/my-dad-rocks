# Get started with a build env with Rust nightly
FROM rust:1.76.0-buster as builder

ARG SQLX_OFFLINE=true
RUN mkdir -p /app
WORKDIR /app
COPY . .

RUN apt-get update && apt-get install --reinstall libasound2-dev -y 

RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-leptos
RUN cargo leptos build --release -vv

FROM rust:1.76.0-buster as runner

# -- NB: update binary name from "leptos_start" to match your app name in Cargo.toml --
# Copy the server binary to the /app directory
COPY --from=builder /app/target/release/my-dad-rocks /app/

# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/target/site /app/site
# Copy Cargo.toml if it’s needed at runtime
COPY --from=builder /app/Cargo.toml /app/
COPY ./assets /app/assets

WORKDIR /app

# Set any required env variables and
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080

# -- NB: update binary name from "leptos_start" to match your app name in Cargo.toml --
# Run the server
CMD ["/app/my-dad-rocks"]

