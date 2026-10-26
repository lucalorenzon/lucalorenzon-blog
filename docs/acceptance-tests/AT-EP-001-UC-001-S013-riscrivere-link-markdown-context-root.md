# AT-EP-001-UC-001-S013: Riscrivere secondo `CONTEXT_ROOT` i link interni scritti dall'autore nel markdown

Epic: EP-001 | UC: UC-001 | Story: EP-001-UC-001-S013

> **Righe spostate il 2026-10-26** da
> [AT-S006](AT-EP-001-UC-001-S006-servire-sito-context-root-configurabile.md),
> dove erano già approvate, senza riscriverle.

---

## Link e immagini scritti dall'autore nel markdown: riscritti

> Source: Story AC-1, AC-2. Valore dell'attributo nell'HTML del corpo dell'ARTICLE-PAGE.

| markdown nel corpo | `CONTEXT_ROOT` | attributo generato? | ref |
|---|---|---|---|
| `[altro](/articles/altro)` | `/lucalorenzon-blog/` | `href="/lucalorenzon-blog/articles/altro"` | AC-1 |
| `![x](/images/x.webp)` | `/lucalorenzon-blog/` | `src="/lucalorenzon-blog/images/x.webp"` | AC-1 |
| `[altro](/articles/altro)` | `/` | `href="/articles/altro"` | AC-2 |

## Link e immagini scritti dall'autore nel markdown: non riscritti

> Source: Story AC-3, AC-4 (si riscrivono solo i percorsi dalla radice; limite accettato: HTML grezzo)

| markdown nel corpo | `CONTEXT_ROOT` | attributo generato? | ref |
|---|---|---|---|
| `[ext](https://example.com/p)` | `/lucalorenzon-blog/` | `href="https://example.com/p"` | AC-3 |
| `[cdn](//cdn.example.com/x)` | `/lucalorenzon-blog/` | `href="//cdn.example.com/x"` | AC-3 |
| `[sez](#sezione)` | `/lucalorenzon-blog/` | `href="#sezione"` | AC-3 |
| `[mail](mailto:a@example.com)` | `/lucalorenzon-blog/` | `href="mailto:a@example.com"` | AC-3 |
| `[rel](altro)` | `/lucalorenzon-blog/` | `href="altro"` | AC-3 |
| `<a href="/articles/altro">raw</a>` (HTML grezzo) | `/lucalorenzon-blog/` | `href="/articles/altro"` | AC-4 |

---

## Open Issues

- **Link relativi nel markdown** (`[rel](altro)`): restano invariati per la
  regola concordata, ma il browser li risolve rispetto all'URL della pagina
  (`/lucalorenzon-blog/articles/` per un articolo). Se sia accettabile, o
  vada vietato o segnalato (validazione dei contenuti
  [[EP-001-UC-001-S007]] o controllo dell'output [[EP-001-UC-001-S009]]), è
  da decidere fuori da questa storia.
