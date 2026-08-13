# AT-EP-001: Aggiornamento dello stack Leptos/Rust

Epic: EP-001 | ADR: [ADR-001](../adr/ADR-001-leptos-target-version.md)

> Livello: Component. EP-001 non ha una Use Case (refactor tecnico senza logica di business), quindi questo file deriva direttamente dagli Acceptance Criteria AC-1..AC-4 dell'epica invece che da estensioni di una UC.

Component under test: build di lucalorenzon-blog (insieme di dipendenze Cargo + toolchain, per ADR-001) e relativo dev server locale (`cargo leptos watch`).

---

## Build

| Cargo.toml dependency set | Comando | Exit code? | Errori di compilazione? | ref |
|---|---|---|---|---|
| Aggiornato alle versioni target ADR-001 (leptos 0.8.20, leptos_meta 0.8.6, leptos_router 0.8.15, leptos_actix 0.8.7, leptos-use 0.19.0, leptos_icons 0.7.1, icondata 0.7.0, wasm-bindgen 0.2.127, actix-web 4.14.1) | `cargo leptos watch` | `0` | `none` | AC-1 |

## Disponibilità del dev server

| Precondizione | Richiesta | Response status? | ref |
|---|---|---|---|
| `cargo leptos watch` in esecuzione | `GET /` su `LEPTOS_SITE_ADDR` | `200` | AC-1 |

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
| `leptos`/`leptos_meta`/`leptos_router`/`leptos_actix` = `"0.8"`, `leptos-use` = `"0.19"`, `leptos_icons` = `"0.7"`, `icondata` = `"0.7"`, `wasm-bindgen` = `"0.2.127"`, `actix-web` = `"4.14"` | ADR-001 | `true` | AC-4 |

## Open Issues

- L'"equivalenza visiva" nelle righe AC-2/AC-3 è verificata manualmente da Luca (nessun tool di visual regression automatico è in scope per EP-001 — quello è EP-002). Il valore `true` rappresenta "nessuna regressione osservata", accertata soggettivamente dall'unico stakeholder, non da un diff automatico.
- `rust-toolchain.toml` attualmente fissa solo `channel = "nightly"` senza data pinnata. Se Leptos 0.8 richiede uno snapshot nightly specifico, il valore esatto è `?UNKNOWN?` finché non si tenta la build — da definire durante l'implementazione.
