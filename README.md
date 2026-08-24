# lucalorenzon-blog

Personal blog for Luca Lorenzon ("Lvk@73r"), built with the [Leptos](https://github.com/leptos-rs/leptos) Rust web framework (v0.8, stable toolchain) and server-rendered via [actix-web](https://actix.rs/). Only the interactive parts of the page ship JavaScript, via Leptos's [islands](https://leptos-rs.github.io/leptos/islands.html) architecture — everything else is plain server-rendered HTML. Styling is [Tailwind CSS v4](https://tailwindcss.com/), compiled by `cargo-leptos`.

This project started from the `leptos-rs/start` template; the structure below reflects how it has since diverged.

## Requirements

- Rust **stable** (pinned via `rust-toolchain.toml`)
- The `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- [`cargo-leptos`](https://github.com/leptos-rs/cargo-leptos): `cargo install cargo-leptos`
- `dart-sass` is **not** required — `cargo-leptos` downloads its own standalone `tailwindcss` CLI to compile `input.css` (Tailwind v4, CSS-first config)

## Running the project

```sh
cargo leptos watch
```

Serves the app at `http://localhost:3000` with hot reload.

## Production build

```sh
cargo leptos build --release
```

This produces:

1. The server binary at `target/release/blog_start`
2. The static site assets (JS/WASM/CSS) at `target/site`

To run the built server standalone, copy both alongside each other and set:

```sh
export LEPTOS_OUTPUT_NAME="blog_start"
export LEPTOS_SITE_ROOT="site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="127.0.0.1:3000"
```

See the `Dockerfile` for a complete example (multi-stage build, static binary, `scratch` runtime image).

## End-to-end tests

```sh
cargo leptos end-to-end
```

Runs the Playwright suite in `end2end/` (see `end2end-cmd`/`end2end-dir` in `Cargo.toml`). To run a single spec directly:

```sh
cd end2end && npx playwright test tests/example.spec.ts
```

## Test coverage

Requires the `llvm-tools-preview` rustup component and [`cargo-llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov):

```sh
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov --locked
```

Both wrap the same stable, official mechanism documented in the [rustc book](https://doc.rust-lang.org/rustc/instrument-coverage.html) (`-C instrument-coverage` + LLVM's `llvm-profdata`/`llvm-cov`) — `cargo-llvm-cov` just automates the merge/discovery steps that tool would otherwise require running by hand.

```sh
cargo llvm-cov --lib --features ssr
```

For a browsable HTML report:

```sh
cargo llvm-cov --lib --features ssr --html --open
```

`--features ssr` is required: the test code (and `ContentSource` adapters) only compile under that feature.

## Project structure

- `src/app.rs` — the `App` root component: `<Html>`/`<Meta>`/`<Stylesheet>`/`<Body>` setup and routes
- `src/layout.rs` — the `Layout` slot-based wrapper every article page is built through
- `src/components/` — one subdirectory per UI unit (headers, footers, logos, menu)
- `src/domain/` — plain domain types and ports (e.g. `Article`, `ContentSource`), independent of the web framework
- `assets/` — static files synced verbatim into the site output and served from `/assets`
- `docs/` — epics, use cases, stories, and ADRs tracking why this project is shaped the way it is

See `CLAUDE.md` for the full architecture notes (compilation targets, islands, routing, styling).
