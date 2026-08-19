# EP-001: Rilancio del sito su stack Leptos/Rust aggiornato

> **Superseded by [EP-008](./EP-008-rilancio-blog-professionale.md)** — 2026-08-19: un assessment architetturale ha ampliato lo scope (content layer + migrazione SSG, non solo aggiornamento dipendenze). Questo epic resta come riferimento storico; ADR-001 collegata resta valida come riferimento tecnico sulle versioni verificate.

> Riportare il sito a compilare ed eseguire con le feature attuali su dipendenze e toolchain correnti, come prerequisito bloccante per tutte le epiche successive.

---

## Motivation

Il progetto è fermo dal 2024-08-14 (ultimo commit di dipendenze), quasi due anni fa rispetto a oggi. È basato su Leptos 0.6 (nightly), mai aggiornato oltre un bump di patch. Nessuna delle epiche successive — migrazione a SSG (EP-003), motore di ricerca (EP-004), pipeline di deploy (EP-005), revisione grafica (EP-006), contenuti (EP-007) — può partire in modo sensato su una base che non è allineata alle versioni correnti dell'ecosistema Leptos: il rischio è costruire su fondamenta che andranno comunque riscritte.

Il motivo per cui questo lavoro riparte ora è che l'utente ha deciso di riportare in vita il progetto per costruirsi una rete professionale in vista di una futura attività in proprio, oltre che come spazio personale.

## Context

Stato attuale: sito Leptos SSR + islands (`experimental-islands`), servito da `actix-web`, con Rust nightly pinato (`rust-toolchain.toml`). Non è mai stato deployato in produzione — è sempre rimasto un banco di prova locale ("mai andato in produzione, sempre stato solo una scusa per provare qualcosa"). Le dipendenze principali (`leptos`, `leptos_meta`, `leptos_router`, `leptos_actix`, `leptos-use`, `leptos_icons`/`icondata`, `wasm-bindgen`) sono ferme alla riga di Leptos 0.6. Il `Cargo.lock` ha attualmente una modifica non committata (bump `leptos` 0.6.14→0.6.15, probabile effetto collaterale di una build locale, non un aggiornamento intenzionale). Il problema noto e già segnalato dall'utente è che il bundle WASM, pur non enorme, si sente nelle prestazioni di download.

## Business Outcome

- Il sito compila ed esegue in locale (`cargo leptos watch`) senza errori, su dipendenze e toolchain correnti
- Tutte le route e le funzionalità interattive oggi presenti (homepage, 404, header dinamico, menu con toggle dark/light, footer, logo) restano funzionanti dopo l'aggiornamento
- Le versioni delle dipendenze Cargo e la relativa motivazione delle scelte sono documentate in un ADR collegato a questa epica

## Scope

### In scope
- Aggiornamento delle dipendenze Cargo (ecosistema Leptos, actix-web, wasm-bindgen, leptos-use, leptos_icons/icondata) a versioni stabili correnti
- Adeguamento del codice alle breaking change introdotte dalle nuove versioni
- Verifica manuale che tutte le feature attuali continuino a funzionare
- Passaggio del toolchain Rust da nightly a stable (non più richiesto da Leptos 0.8, vedi ADR-001)
- ADR sulla versione target di Leptos e sulle breaking change rilevanti

### Out of scope
- Migrazione architetturale a SSG (EP-003)
- Riduzione misurabile del bundle WASM come criterio di accettazione — è un effetto atteso ma non l'obiettivo di questa epica (l'obiettivo architetturale è EP-003)
- Scrittura di test automatici oltre alla verifica manuale (EP-002)
- Deploy, CI/CD, dominio custom (EP-005)
- Motore di ricerca (EP-004)
- Revisione grafica e accessibilità (EP-006)
- Contenuti (EP-007)

## Constraints

| Type | Constraint |
|---|---|
| Technical | Toolchain Rust in transizione da nightly a stable (vedi ADR-001); target `wasm32-unknown-unknown`; le crate dell'ecosistema Leptos devono avanzare in modo coerente tra loro (versioni compatibili) |
| Technical | `actix-web`/`leptos_actix` restano il backend SSR per questa epica — la sostituzione con SSG è fuori scope qui |
| Business | Nessuna scadenza esterna rigida, ma priorità alta: blocca l'avvio di tutte le altre epiche |

## Acceptance Criteria

| ID | Criterion | Verified by |
|---|---|---|
| AC-1 | `cargo leptos watch` avvia il sito in locale senza errori di build | Esecuzione del comando e osservazione dell'esito |
| AC-2 | Tutte le route esistenti (home, 404) sono raggiungibili e visivamente equivalenti a prima dell'aggiornamento | Navigazione manuale del sito in locale |
| AC-3 | Le funzionalità interattive esistenti (apertura/chiusura menu, toggle dark/light) continuano a funzionare | Interazione manuale con il sito in locale |
| AC-4 | Le dipendenze Cargo sono aggiornate a versioni stabili correnti, con motivazione documentata | Lettura di `Cargo.toml`/`Cargo.lock` e dell'ADR collegato |

## ADRs

| ADR | Title | Date | Status |
|---|---|---|---|
| [ADR-001](../adr/ADR-001-leptos-target-version.md) | Target version for Leptos and the related dependency ecosystem | 2026-08-14 | Proposed |

## Use Cases

| UC ID | Title | Goal level | Status |
|---|---|---|---|
| — | — | — | — |

> Nota: questa epica è un lavoro tecnico di ripristino/refactor (aggiornamento dipendenze, adeguamento a breaking change) senza logica di business né interazione attore-sistema da modellare. Non prevede Use Case; si procede direttamente a story-split.

## Acceptance Tests

[AT-EP-001](../acceptance-tests/AT-EP-001-aggiornamento-stack-leptos-rust.md) — livello Component, derivato dagli Acceptance Criteria AC-1..AC-4 in assenza di una UC.

## Stories

| Story ID | Title | Status |
|---|---|---|
| [EP-001-S001](../stories/EP-001-S001-aggiornamento-dipendenze-migrazione-islands.md) | Aggiornare le dipendenze Cargo e migrare l'architettura islands al feature flag stabilizzato | Pending discussion |
| [EP-001-S002](../stories/EP-001-S002-verifica-route-esistenti.md) | Verificare e correggere le route esistenti dopo l'aggiornamento | Pending discussion |
| [EP-001-S003](../stories/EP-001-S003-verifica-funzionalita-interattive-islands.md) | Verificare e correggere le funzionalità interattive (islands) dopo l'aggiornamento | Pending discussion |

## Open Issues

- —

## Resolved Issues

- ~~Il `Cargo.lock` ha una modifica non committata (bump 0.6.14→0.6.15) da chiarire prima di iniziare~~ — risolto: era l'effetto di un `cargo update` esplorativo, non un aggiornamento intenzionale. `Cargo.lock` è stato ripristinato con `git restore` per ripartire da uno stato pulito prima di iniziare il lavoro su questa epica.
- ~~Verificare qual è l'ultima versione stabile di Leptos e se introduce breaking change rilevanti per `experimental-islands`~~ — risolto: vedi [ADR-001](../adr/ADR-001-leptos-target-version.md) (target Leptos 0.8.20, feature flag `experimental-islands` → `islands`).

---

| Updated | What changed |
|---|---|
| 2026-08-13 | Epic created |
| 2026-08-14 | Closed the Cargo.lock open issue (was a leftover from an exploratory `cargo update`, deliberately reverted with `git restore`) |
| 2026-08-14 | Determined this epic has no business-facing interaction to model as a Use Case (pure technical refactor) — will proceed directly to story-split |
| 2026-08-14 | Added ADR-001 (Leptos target version 0.8.20 and related ecosystem) and closed the last open issue |
| 2026-08-14 | Added AT-EP-001 (Component-level acceptance tests, derived from AC-1..AC-4 in place of a missing UC) |
| 2026-08-14 | Split into 3 stories (EP-001-S001..S003) via sw-story-split, traced to AT-EP-001 |
| 2026-08-14 | Rewrote actor references and verification steps in an impersonal tone across the epic and AT-EP-001 (documentation must not read as a narrative about a named person) |
| 2026-08-14 | Removed the Actors section — not meaningful for a technical epic with no actor-system interaction to model |
| 2026-08-14 | Discussed and resolved EP-001-S001's Decision field: wasm-bindgen moves to a range instead of an exact pin, toolchain moves from nightly to stable (no longer required by Leptos 0.8), added a hydration/console-error check to scope. Updated ADR-001, AT-EP-001, and the story accordingly |
| 2026-08-19 | Superseded by EP-008 — vedi nota sotto il titolo |
