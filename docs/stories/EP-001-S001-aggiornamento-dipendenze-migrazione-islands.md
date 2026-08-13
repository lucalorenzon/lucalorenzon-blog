## EP-001-S001 — Aggiornare le dipendenze Cargo e migrare l'architettura islands al feature flag stabilizzato

**Decision:** Eseguire l'aggiornamento delle dipendenze Cargo alle versioni target di ADR-001 in un solo passaggio, e migrare l'architettura islands dal feature flag sperimentale `experimental-islands` al flag stabilizzato `islands` (nuovo entrypoint `hydrate_islands()`, `HydrationScripts(islands=true)`), fino a ottenere una build che compila senza errori.

Obiettivo tecnico: portare il progetto a compilare ed eseguire sulle versioni stabili correnti delle dipendenze Leptos, senza introdurre regressioni percepibili dall'utente finale del sito, come base su cui verificare e correggere il resto delle funzionalità.

| Field | Value |
|---|---|
| **Epic** | EP-001 — Rilancio del sito su stack Leptos/Rust aggiornato |
| **UC** | N/A — refactor tecnico, tracciato su EP-001 AC-1..AC-4 e AT-EP-001 (nessuna logica di business coinvolta) |
| **Pattern** | Major effort isolation (isola la parte più rischiosa — la migrazione delle islands — dalla verifica funzionale successiva) + Spike first (bump in un solo passaggio, si corregge ciò che il compilatore segnala, come da ADR-001) |
| **AT rows** | AT-EP-001: tabella "Build" (ref AC-1), tabella "Disponibilità del dev server" (ref AC-1), tabella "Versioni delle dipendenze e motivazione" (ref AC-4) |

### Acceptance criteria
- Given `Cargo.toml` aggiornato alle versioni target di ADR-001, when si esegue `cargo leptos watch`, then il comando termina con exit code `0` e nessun errore di compilazione
- Given il dev server è in esecuzione, when si effettua `GET /` su `LEPTOS_SITE_ADDR`, then la risposta ha status `200`
- Given il codice usa ancora il feature flag `experimental-islands`, when si migra a `islands` con `hydrate_islands()` e `HydrationScripts(islands=true)`, then il progetto compila senza riferimenti al vecchio flag/entrypoint
- Given `Cargo.toml`/`Cargo.lock` aggiornati, when si confrontano con ADR-001, then le versioni corrispondono esattamente a quelle documentate

### Design pipeline
Before any implementation, complete in order:
- [ ] `/software-design`        — coupling, ownership, accidental complexity
- [ ] `/hexagonal-architecture` — ports, adapters, composition root
- [ ] `/parse-dont-validate`    — domain types and invariants
- [ ] `/sw-practices`           — naming, error handling, bootstrap

### Next steps after agreement
- [ ] `/acceptance-tests EP-001-S001` — story-level AT table
- [ ] `/story-size EP-001-S001`       — assign XS / S / M / L / XL / XXL

### Open questions
- Se la build richiede uno snapshot nightly diverso da quello attualmente pinnato in `rust-toolchain.toml` (bare `channel = "nightly"`), va aggiornato come parte di questa storia — verificare durante l'implementazione (vedi AT-EP-001 Open Issues)
- Se emergono breaking change nelle API di `leptos-use`, `leptos_router` o `leptos_icons`/`icondata` non ancora note dalla ricerca in ADR-001, vanno risolte qui prima di procedere alle storie successive

INVEST: I✓ N✓ V✓ E✓ S✓ T✓  |  1 Decision: ✓
