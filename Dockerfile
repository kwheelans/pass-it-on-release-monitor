FROM lukemathwalker/cargo-chef:latest as chef

FROM chef AS planner
WORKDIR /recipe
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /pass-it-on-release-monitor

# Build dependencies
COPY --from=planner /recipe/recipe.json recipe.json
RUN cargo chef cook --release --features vendored-tls --recipe-path recipe.json

# Build application
COPY . .
RUN cargo build --release --frozen --features vendored-tls --bin pass-it-on-release-monitor

# Final image
FROM debian:12-slim

RUN mkdir /pass-it-on-release-monitor
WORKDIR /pass-it-on-release-monitor

ENV PATH=/pass-it-on-release-monitor:$PATH \
VERBOSITY=Info

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /pass-it-on-release-monitor/target/release/pass-it-on-release-monitor /pass-it-on-release-monitor
VOLUME /config

CMD ["pass-it-on-release-monitor","--config", "/config/monitor.toml"]
