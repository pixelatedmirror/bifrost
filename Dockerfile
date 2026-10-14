# Building Stage
ARG RUST_VERSION=1.95
FROM rust:${RUST_VERSION}-slim-trixie AS build
WORKDIR /app
COPY LICENSE LICENSE

ENV PATH="/.cargo/bin:$PATH"

RUN apt-get -y update && apt-get install -y --no-install-recommends pkg-config libssl-dev curl
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli --root "/.cargo" -y --force
RUN rustup target add wasm32-unknown-unknown

RUN --mount=type=bind,source=doc,target=doc \
    --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=crates,target=crates \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    <<EOF
set -e
cargo build --locked --release
cp target/release/bifrost /bifrost
(cd crates/bifrost-frontend && dx bundle --platform web --release)
cp -r target/dx/frontend/release/web/public /frontend
mkdir -p /frontend/assets
cp -r crates/bifrost-frontend/assets/* /frontend/assets/
EOF


# Final Stage
FROM debian:trixie-slim AS final

COPY --from=build /bifrost /app/bifrost
COPY --from=build /frontend /app/frontend

RUN apt-get -y update && apt-get install -y --no-install-recommends libssl3 ca-certificates

WORKDIR /app

CMD ["/app/bifrost"]
