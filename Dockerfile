# syntax=docker/dockerfile:1

# ---- Builder: compiles the server binary (musl, statically linked) and the WASM/JS/CSS site bundle ----
FROM rust:1-bookworm AS builder

# musl-tools provides musl-gcc, needed to statically link the C dependencies (e.g. zstd-sys,
# pulled in by actix-web's compression middleware) against musl instead of glibc.
RUN apt-get update && apt-get install -y --no-install-recommends musl-tools \
    && rm -rf /var/lib/apt/lists/*

# The base image's default toolchain is named after its exact version (e.g. "1.98.0-..."),
# not "stable" — but rust-toolchain.toml pins channel = "stable". Without this, rustup
# resolves that override to a *different*, freshly-installed "stable" toolchain later (once
# rust-toolchain.toml is copied in) that never got the targets added below, and the build
# fails with "can't find crate for `core`/`std`" for both targets.
RUN rustup toolchain install stable && rustup default stable
RUN rustup target add wasm32-unknown-unknown x86_64-unknown-linux-musl

# CC_<target> is only for compiling the C sources of dependencies like zstd-sys against musl.
# Deliberately NOT overriding CARGO_TARGET_..._LINKER: doing so forces the *final* link through
# Debian's musl-gcc CRT objects instead of rustc's own self-contained musl linker, producing a
# corrupt binary that segfaults (SIGSEGV, SEGV_MAPERR at NULL) before a single syscall — verified
# with strace. Leaving the linker on rustc's default self-contained musl support fixes it.
ENV CC_x86_64_unknown_linux_musl=musl-gcc
# crt-static is the musl-target default, but pinning it explicitly avoids relying on that not
# changing. Scoped to this one target only so it doesn't reach the wasm32 frontend build.
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-C target-feature=+crt-static"

# Cache the cargo registry/git index across image rebuilds, so re-running this ~2min compile
# only happens when the cache is explicitly pruned, not on every unrelated source change.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo install cargo-leptos --locked

WORKDIR /app
COPY . .

# cargo-leptos itself always expects the server binary at the host-triple path
# (target/release/<name>) regardless of --bin-cargo-args, so it can't drive a cross-compiled
# musl build directly. Instead: let it build only the wasm/JS/CSS site bundle (target triple
# doesn't matter there), and build the server binary ourselves with a plain `cargo build`
# targeting musl for a fully static binary that runs in `scratch` below.
#
# Cache mounts are scratch space, not part of the final layer — copy the build output out of
# the cached target/ dir into normal image paths before the mount is torn down, so the
# `scratch` stage below can COPY --from them.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    cargo leptos build --release --frontend-only \
    && cargo build --release --package blog_start --bin blog_start \
       --no-default-features --features ssr --target x86_64-unknown-linux-musl \
    && cp target/x86_64-unknown-linux-musl/release/blog_start /blog_start \
    && cp -r target/site /site

# ---- Runtime: no OS, no shell, no package manager — just the static binary and site assets ----
FROM scratch AS runner

COPY --from=builder /blog_start /blog_start
COPY --from=builder /site /site

ENV RUST_LOG="info"
ENV APP_ENVIRONMENT="production"
ENV LEPTOS_OUTPUT_NAME="blog_start"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="/site"
ENV LEPTOS_SITE_PKG_DIR="pkg"

EXPOSE 8080

CMD ["/blog_start"]
