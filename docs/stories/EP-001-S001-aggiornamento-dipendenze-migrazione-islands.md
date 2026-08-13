## EP-001-S001 — Aggiornare le dipendenze Cargo e migrare l'architettura islands al feature flag stabilizzato

**Decision:** Eseguire l'aggiornamento delle dipendenze Cargo alle versioni target di ADR-001 in un solo passaggio (con `wasm-bindgen` su range `"0.2"` invece di pin esatto), migrare l'architettura islands dal feature flag sperimentale `experimental-islands` al flag stabilizzato `islands` (nuovo entrypoint `hydrate_islands()`, `HydrationScripts(islands=true)`), e passare il toolchain da nightly a stable (rimuovendo la feature `nightly` da `leptos`/`leptos_meta`/`leptos_router` e riscrivendo l'unico signal a sintassi function-call in `.get()`), fino a ottenere una build che compila senza errori e un bundle che si idrata senza errori in console.

Obiettivo tecnico: portare il progetto a compilare ed eseguire sulle versioni stabili correnti delle dipendenze Leptos, su toolchain Rust stable, senza introdurre regressioni percepibili dall'utente finale del sito, come base su cui verificare e correggere il resto delle funzionalità.

| Field | Value |
|---|---|
| **Epic** | EP-001 — Rilancio del sito su stack Leptos/Rust aggiornato |
| **UC** | N/A — refactor tecnico, tracciato su EP-001 AC-1..AC-4 e AT-EP-001 (nessuna logica di business coinvolta) |
| **Pattern** | Major effort isolation (isola la parte più rischiosa — la migrazione delle islands e del toolchain — dalla verifica funzionale successiva) + Spike first (bump in un solo passaggio, si corregge ciò che il compilatore segnala, come da ADR-001) |
| **AT rows** | AT-EP-001: tabella "Build" (ref AC-1), tabella "Disponibilità del dev server" (ref AC-1), tabella "Caricamento client (hydration)" (ref AC-1), tabella "Versioni delle dipendenze e motivazione" (ref AC-4) |

### Acceptance criteria
- Given `Cargo.toml` aggiornato alle versioni target di ADR-001 (incluso `wasm-bindgen` su range `"0.2"`), when si esegue `cargo leptos watch`, then il comando termina con exit code `0` e nessun errore di compilazione
- Given il dev server è in esecuzione, when si effettua `GET /` su `LEPTOS_SITE_ADDR`, then la risposta ha status `200`
- Given la pagina `/` è caricata nel browser, when l'hydration è completata, then non compaiono errori in console
- Given il codice usa ancora il feature flag `experimental-islands`, when si migra a `islands` con `hydrate_islands()` e `HydrationScripts(islands=true)`, then il progetto compila senza riferimenti al vecchio flag/entrypoint
- Given il toolchain è nightly con la feature `nightly` attiva su `leptos`/`leptos_meta`/`leptos_router`, when si passa a stable (aggiornando `rust-toolchain.toml` e rimuovendo la feature), then il progetto compila su stable senza la feature `nightly`, con `dark_mode_enabled()` riscritto in `dark_mode_enabled.get()` in `src/components/menu/menu.rs`
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
- Se emergono breaking change nelle API di `leptos-use`, `leptos_router` o `leptos_icons`/`icondata` non ancora note dalla ricerca in ADR-001, vanno risolte qui prima di procedere alle storie successive
- `CLAUDE.md` (root del progetto) documenta oggi "Requires Rust nightly" — va aggiornato per riflettere il passaggio a stable, ma solo a implementazione avvenuta (non ora, in fase di sola documentazione)

INVEST: I✓ N✓ V✓ E✓ S✓ T✓  |  1 Decision: ✓
