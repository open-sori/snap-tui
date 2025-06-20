FROM docker.io/alpine:3.21.3 AS builder

WORKDIR /snap-tui

RUN apk add --update bash curl ca-certificates alpine-sdk alsa-lib-dev pkgconfig openssl-dev rustup && \
    rustup-init -y && \
    bash && \
    curl --silent --location --fail --remote-name-all --request GET "https://github.com/open-sori/snap-tui/archive/refs/tags/v1.0.0.tar.gz" --output /snap-tui/snap-tui.tar.gz && \
    tar --extract --gzip --file /snap-tui/snap-tui.tar.gz --directory /snap-tui && \
    cd /snap-tui/snap-tui-1.0.0 && \
    source "$HOME/.cargo/env" && \
    cargo build --release

FROM scratch

# Ref from https://github.com/opencontainers/image-spec/blob/main/annotations.md
LABEL org.opencontainers.image.created="2025-06-19"
LABEL org.opencontainers.image.authors="thibault@open-sori.dev"
LABEL org.opencontainers.image.url="https://github.com/orgs/open-sori/packages/container/package/snap-tui"
LABEL org.opencontainers.image.documentation="https://github.com/open-sori/snap-tui"
LABEL org.opencontainers.image.source="https://github.com/open-sori/snap-tui"
LABEL org.opencontainers.image.version="v1.0.0"
LABEL org.opencontainers.image.revision="v1.0.0"
LABEL org.opencontainers.image.vendor="open-sori"
LABEL org.opencontainers.image.licenses="GPL-3.0-or-later"
LABEL org.opencontainers.image.ref.name="snap-tui"
LABEL org.opencontainers.image.title="snap-tui"
LABEL org.opencontainers.image.description="snap-tui binary docker image"
LABEL org.opencontainers.image.base.name="ghcr.io/open-sori/snap-tui:v1.0.0"

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

COPY --from=builder /librespot/target/release/snap-tui /bin/snap-tui

ENTRYPOINT ["snap-tui"]