# lucalorenzon-blog

A personal blog that showcases the author's professional experience through
articles on a range of topics.

Status: being rebuilt from scratch. The first goal is a minimal live site
(a single "under construction" home page); everything else grows from there
in small increments.

## Stack

Rust + [Leptos](https://github.com/leptos-rs/leptos), styled with
[Tailwind CSS](https://tailwindcss.com/), generated as a static site and
hosted on GitHub Pages.

## Build and test locally

Requirements: Rust (stable, installed automatically from
`rust-toolchain.toml`), Node.js 24, Python 3, Linux x64.

```sh
scripts/build-site.sh            # builds the site into target/site
cargo test                       # unit tests
cd end2end && npm ci && npx playwright install chromium
npx playwright test              # opens the built site in a browser
```

Every push to `main` runs the same checks on GitHub, publishes the site
to GitHub Pages and then runs the browser test again on the public
address.

## Documentation

| Document | Path | Description |
|---|---|---|
| Plan | [docs/plan.md](docs/plan.md) | What is being built now, next and later, and the decisions still open |
| Backlog | [docs/backlog/](docs/backlog/) | Things noticed but deliberately not done yet, with the reason |
| Session minutes | [docs/sessions/](docs/sessions/) | One record per working session: decisions and why |
| Working rules | [CLAUDE.md](CLAUDE.md) | How this project is developed |

## License

[MIT](LICENSE)
