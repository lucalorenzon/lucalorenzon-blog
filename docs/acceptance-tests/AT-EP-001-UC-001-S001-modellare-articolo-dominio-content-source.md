# AT-EP-001-UC-001-S001: Modellare l'articolo e la porta ContentSource

Epic: EP-001 | UC: UC-001 | Story: EP-001-UC-001-S001

---

## Component: Article — costruzione da frontespizio (happy path)

> Source: Story AC-1

| data | slug | tag | titolo | abstract | immagine di sintesi | costruzione riuscita? | Article.abstract? | Article.immagine? | ref |
|---|---|---|---|---|---|---|---|---|---|
| valida | valido | valida (≥1 tag) | valido | presente e ben formato | presente e ben formata | `true` | `Some(<abstract>)` | `Some(<immagine>)` | AC-1 |
| valida | valido | valida (≥1 tag) | valido | assente | presente e ben formata | `true` | `None` | `Some(<immagine>)` | AC-1 |
| valida | valido | valida (≥1 tag) | valido | presente e ben formato | assente | `true` | `Some(<abstract>)` | `None` | AC-1 |

---

## Component: Article — costruzione rifiutata (campo obbligatorio assente)

> Source: Story AC-2, AC-3, AC-4, AC-5

| data | slug | tag | titolo | costruzione riuscita? | campo causa? | ref |
|---|---|---|---|---|---|---|
| assente | valido | valida (≥1 tag) | valido | `false` | `Data` | AC-2 |
| valida | assente | valida (≥1 tag) | valido | `false` | `Slug` | AC-3 |
| valida | valido | assente (lista vuota) | valido | `false` | `Tag` | AC-4 |
| valida | valido | valida (≥1 tag) | assente | `false` | `Titolo` | AC-5 |

---

## Component: Article — costruzione rifiutata (campo obbligatorio malformato)

> Source: Story AC-2, AC-3, AC-4, AC-5

| data | slug | tag | titolo | costruzione riuscita? | campo causa? | ref |
|---|---|---|---|---|---|---|
| malformata (`2026-02-30` — calendario inesistente) | valido | valida (≥1 tag) | valido | `false` | `Data` | AC-2 |
| valida | malformato (`Il Mio Slug!` — maiuscole/spazi/punteggiatura non ammessi) | valida (≥1 tag) | valido | `false` | `Slug` | AC-3 |
| valida | valido | contiene un valore malformato (`Rust Web` — maiuscole/spazi non ammessi) | valido | `false` | `Tag` | AC-4 |
| valida | valido | valida (≥1 tag) | malformato (contiene un carattere di controllo, es. newline) | `false` | `Titolo` | AC-5 |

---

## Component: ContentSource — forma della porta

> Source: Story AC-6

| metodo | dichiarato nell'interfaccia? | implementato in S001? | ref |
|---|---|---|---|
| lettura articolo singolo per slug | `true` | `true` | AC-6 |
| elenco articoli pubblicati | `true` | `false` (implementato in EP-001-UC-001-S002/S003) | AC-6 |

---

## Open Issues

- ~~**Formato "malformato" per data/slug/tag/titolo non ancora deciso**~~ — risolto in `/parse-dont-validate` (2026-08-23): `PublicationDate` = `YYYY-MM-DD` calendario reale; `Slug`/`Tag` = kebab-case ASCII minuscolo condiviso; `Title` = non vuoto dopo trim, nessun carattere di controllo. Dettaglio: [docs/design/article.md](../design/article.md).
- ~~**Tipo/messaggio esatto dell'errore di costruzione**~~ — risolto (2026-08-23): implementazione in `src/domain/article.rs` e `src/domain/value_objects/*.rs` corrisponde esattamente al design artefact; verificato dai 14 test di dominio (`cargo test`, tutti verdi).
- ~~**Fallimenti di lettura da ContentSource (I/O, file non trovato) non coperti**~~ — risolto (2026-08-23): in scope di S001, coperti da `FilesystemContentSource` (`src/adapters/secondary/content_source/filesystem.rs`) — `NotFound` (file assente) e `Malformed` (frontmatter assente/non valido/campo invalido) testati su file temporanei reali. `FetchError::Io` (errori di I/O diversi da "file non trovato", es. permessi) resta mappato nel codice ma non ha un test automatico dedicato — dipendente da OS, giudicato non necessario per questa story; nessuna riga AT lo richiede.

---

## Traceability

Ogni riga di queste tabelle traccia a un acceptance criterion di [EP-001-UC-001-S001](../stories/EP-001-UC-001-S001-modellare-articolo-dominio-content-source.md) ed è un sottoinsieme più fine delle righe 1-5 e 3a di [AT-UC-001](AT-UC-001-autore-pubblica-articolo.md), limitato alla costruzione del dominio `Article` (le righe UC-level su abstract/immagine calcolati automaticamente a build time restano fuori scope, vedi [[EP-001-UC-001-S003]]).
