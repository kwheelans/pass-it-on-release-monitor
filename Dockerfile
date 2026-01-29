FROM lukemathwalker/cargo-chef:latest AS chef

FROM chef AS planner
WORKDIR /recipe
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /pass-it-on-release-monitor

# Build dependencies
COPY --from=planner /recipe/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .
RUN cargo build --release --frozen --bin pass-it-on-release-monitor
RUN ./target/release/pass-it-on-release-monitor --download-pico-css

# Download CSS
FROM ghcr.io/kwheelans/container-utils:0.1 AS css
WORKDIR /app
RUN container-utils pico-css-download

# Final image
FROM debian:13-slim

RUN mkdir /pass-it-on-release-monitor /data
WORKDIR /pass-it-on-release-monitor

ENV PATH=/pass-it-on-release-monitor:$PATH \
VERBOSITY=Info

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /pass-it-on-release-monitor/target/release/pass-it-on-release-monitor /pass-it-on-release-monitor
COPY --from=css /app/css /pass-it-on-release-monitor/css
VOLUME /config
VOLUME /data

CMD ["pass-it-on-release-monitor","--config", "/config/monitor.toml"]
