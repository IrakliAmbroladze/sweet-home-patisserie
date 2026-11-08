FROM rust:1-bookworm AS builder
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-leptos --locked
WORKDIR /app
COPY . .
RUN cargo leptos build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/sweet-home-patisserie /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/

ENV LEPTOS_ENV="PROD"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080
CMD ["/app/sweet-home-patisserie"]
