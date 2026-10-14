# Building Stage
ARG RUST_VERSION=1.95
FROM rust:${RUST_VERSION}-slim-trixie AS build
WORKDIR /app
COPY LICENSE LICENSE

RUN apt-get -y update && apt-get install -y --no-install-recommends pkg-config libssl-dev

RUN --mount=type=bind,source=doc,target=doc \
    --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=crates,target=crates \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    <<EOF
set -e
cargo build --locked --release
cp target/release/bifrost /bifrost
EOF


# Final Stage
FROM debian:trixie-slim AS final

COPY --from=build /bifrost /app/bifrost

RUN apt-get -y update && apt-get install -y --no-install-recommends libssl3 ca-certificates

WORKDIR /app

CMD ["/app/bifrost"]
