FROM rust:1-trixie AS chef
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-chef cargo-leptos --locked
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --no-default-features --features ssr --recipe-path recipe.json
RUN cargo chef cook --profile wasm-release --target wasm32-unknown-unknown \
    --no-default-features --features hydrate --target-dir target/front --recipe-path recipe.json
# only now copy the source
COPY . .
RUN cargo leptos build --release

FROM debian:trixie-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/sweet-home-patisserie /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/

ENV LEPTOS_ENV="PROD"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080
CMD ["/app/sweet-home-patisserie"]
