# AT-EP-001: Aggiornamento dello stack Leptos/Rust

Epic: EP-001 | ADR: [ADR-001](../../adr/ADR-001-leptos-target-version.md)

> Livello: Component. EP-001 non ha una Use Case (refactor tecnico senza logica di business), quindi questo file deriva direttamente dagli Acceptance Criteria AC-1..AC-4 dell'epica invece che da estensioni di una UC.

Component under test: build di lucalorenzon-blog (insieme di dipendenze Cargo + toolchain, per ADR-001) e relativo dev server locale (`cargo leptos watch`).

---

## Build

| Cargo.toml dependency set | Comando | Exit code? | Errori di compilazione? | ref |
|---|---|---|---|---|
| Aggiornato alle versioni target ADR-001 (leptos 0.8.20, leptos_meta 0.8.6, leptos_router 0.8.15, leptos_actix 0.8.7, leptos-use 0.19.0, leptos_icons 0.7.1, icondata 0.7.0, wasm-bindgen 0.2.x su range `"0.2"`, actix-web 4.14.1, toolchain stable) | `cargo leptos watch` | `0` | `none` | AC-1 |

## Disponibilità del dev server

| Precondizione | Richiesta | Response status? | ref |
|---|---|---|---|
| `cargo leptos watch` in esecuzione | `GET /` su `LEPTOS_SITE_ADDR` | `200` | AC-1 |

## Caricamento client (hydration)

| Precondizione | Azione | Errori in console del browser? | ref |
|---|---|---|---|
| Pagina `/` caricata nel browser dopo il deploy locale | Attesa del completamento dell'hydration (bundle WASM caricato e islands montate) | `nessuno` | AC-1 |

## Route

| Route richiesta | Response status? | Contenuto equivalente alla baseline pre-aggiornamento? | ref |
|---|---|---|---|
| `GET /` | `200` | `true` | AC-2 |
| `GET /<path-inesistente>` | `404` *(impostato esplicitamente in `src/app.rs:59` via `resp.set_status(StatusCode::NOT_FOUND)`)* | `true` | AC-2 |

## Funzionalità interattive (islands)

| Feature | Azione utente | Risultato osservabile atteso | ref |
|---|---|---|---|
| Menu (`DynamicHeader`) | Click sul toggle del menu | Lo stato aperto/chiuso del menu si inverte, come prima dell'aggiornamento | AC-3 |
| Dark/Light switch (`LightDarkSwitch`) | Click sul toggle del tema | La classe `dark` sull'elemento radice si attiva/disattiva, come prima dell'aggiornamento | AC-3 |

## Versioni delle dipendenze e motivazione

| Cargo.toml / Cargo.lock | Riferimento ADR | Versioni documentate con motivazione? | ref |
|---|---|---|---|
| `leptos`/`leptos_meta`/`leptos_router`/`leptos_actix` = `"0.8"` (senza feature `nightly`), `leptos-use` = `"0.19"`, `leptos_icons` = `"0.7"`, `icondata` = `"0.7"`, `wasm-bindgen` = `"0.2"`, `actix-web` = `"4.14"` | ADR-001 | `true` | AC-4 |

## Open Issues

- L'"equivalenza visiva" nelle righe AC-2/AC-3 è verificata manualmente (nessun tool di visual regression automatico è in scope per EP-001 — quello è EP-002). Il valore `true` rappresenta "nessuna regressione osservata", accertata soggettivamente dall'unico stakeholder, non da un diff automatico.
