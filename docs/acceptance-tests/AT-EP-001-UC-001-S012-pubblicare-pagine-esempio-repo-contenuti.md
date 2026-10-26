# AT-EP-001-UC-001-S012: Pubblicare pagine di esempio dal repo contenuti sotto `CONTEXT_ROOT`

Epic: EP-001 | UC: UC-001 | Story: EP-001-UC-001-S012

> **Righe spostate il 2026-10-26** da
> [AT-S006](AT-EP-001-UC-001-S006-servire-sito-context-root-configurabile.md),
> dove erano già approvate, senza riscriverle. **Da completare in
> `/acceptance-tests`:** lettura del repo privato in CI con la credenziale,
> homepage con l'elenco al posto della pagina "in costruzione".

Valori di riferimento: slug di esempio `hello-world`, immagine propria di
esempio `cover.webp` (da sostituire con gli slug degli articoli "Lorem
ipsum" quando esistono).

---

## Percorsi generati: derivati dai contenuti

> Source: Story AC-2

| elemento | con `CONTEXT_ROOT` = `/lucalorenzon-blog/`? | con `CONTEXT_ROOT` = `/`? | ref |
|---|---|---|---|
| link all'articolo in LISTING-PAGE (`<a>` `href`) | `/lucalorenzon-blog/articles/hello-world` | `/articles/hello-world` | AC-2 |
| immagine propria dell'articolo (`<img>` `src`) | `/lucalorenzon-blog/images/cover.webp` | `/images/cover.webp` | AC-2 |
| immagine di fallback (`<img>` `src`) | `/lucalorenzon-blog/images/article-image-not-found.svg` | `/images/article-image-not-found.svg` | AC-2 |

---

## Sito servito

> Source: Story AC-3. Output servito da un host statico che fa corrispondere la `CONTEXT_ROOT` alla radice dell'output (come GitHub Pages), con risoluzione degli URL senza estensione (`/articles/hello-world` → `articles/hello-world.html`).

| `CONTEXT_ROOT` | URL richiesto | stato HTTP? | richieste con stato `404` durante il caricamento della pagina? | ref |
|---|---|---|---|---|
| `/lucalorenzon-blog/` | `/lucalorenzon-blog/articles/hello-world` | `200` (ARTICLE-PAGE) | `0` | AC-3 |
| `/lucalorenzon-blog/` | `/lucalorenzon-blog/articles/page/1` | `200` (LISTING-PAGE) | `0` | AC-3 |
| `/` | `/articles/hello-world` | `200` (ARTICLE-PAGE) | `0` | AC-3 |
| `/` | `/articles/page/1` | `200` (LISTING-PAGE) | `0` | AC-3 |

---

## Sviluppo locale

> Source: Story AC-5

| `CONTEXT_ROOT` in `cargo leptos watch` | URL richiesto | stato HTTP? | richieste con stato `404` durante il caricamento? | ref |
|---|---|---|---|---|
| `/` (dichiarata nella configurazione di sviluppo) | `http://localhost:3000/articles/hello-world` | `200` (ARTICLE-PAGE) | `0` | AC-5 |

---

## Open Issues

- **Host statico** per le righe "Sito servito": manuale finché non c'è
  l'infrastruttura Playwright di [[EP-001-S010]], o finché il design di
  S006 non adatta il server di sviluppo.
- **Righe da aggiungere** (vedi nota in cima).
