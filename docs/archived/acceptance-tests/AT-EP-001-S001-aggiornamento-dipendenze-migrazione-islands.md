# AT-EP-001-S001: Aggiornare le dipendenze Cargo e migrare l'architettura islands al feature flag stabilizzato

Epic: EP-001 | ADR: [ADR-001](../../adr/ADR-001-leptos-target-version.md) | Story: [EP-001-S001](../stories/EP-001-S001-aggiornamento-dipendenze-migrazione-islands.md)

> Livello: Story. Le 6 righe Given/When/Then della Decision di EP-001-S001 sono numerate AC-1..AC-6 nell'ordine in cui compaiono nella storia.

---

## Component: Build

| Cargo.toml dependency set | Comando | Exit code? | Errori di compilazione? | ref |
|---|---|---|---|---|
| Aggiornato alle versioni target ADR-001 (leptos 0.8.20, leptos_meta 0.8.6, leptos_router 0.8.15, leptos_actix 0.8.7, leptos-use 0.19.0, leptos_icons 0.7.1, icondata 0.7.0, wasm-bindgen su range `"0.2"`, actix-web 4.14.1, toolchain stable) | `cargo leptos watch` | `0` | `none` | AC-1 |

## Component: Disponibilità del dev server

| Precondizione | Richiesta | Response status? | ref |
|---|---|---|---|
| `cargo leptos watch` in esecuzione | `GET /` su `LEPTOS_SITE_ADDR` | `200` | AC-2 |

## Component: Caricamento client (hydration)

| Precondizione | Azione | Errori in console del browser? | ref |
|---|---|---|---|
| Pagina `/` caricata nel browser dopo `cargo leptos watch` | Attesa del completamento dell'hydration (bundle WASM caricato, islands montate) | `nessuno` | AC-3 |

## Component: Migrazione feature flag islands

| Cargo.toml `leptos`/`leptos_actix` features (stato pre-migrazione) | Stato atteso post-migrazione | Entry point hydration (`src/lib.rs`) | HydrationScripts (route SSR, `src/main.rs`) | Riferimenti residui a `experimental-islands` nel codice? | ref |
|---|---|---|---|---|---|
| `"experimental-islands"` (Leptos 0.6) | `"islands"` (Leptos 0.8) | `leptos::mount::hydrate_islands()` | `<HydrationScripts islands=true/>` | `0` | AC-4 |

## Component: Migrazione toolchain a stable

| `rust-toolchain.toml` (pre) | `rust-toolchain.toml` (atteso post) | Feature `nightly` su `leptos`/`leptos_meta`/`leptos_router` (atteso post) | Chiamata al signal in `src/components/menu/menu.rs` (`LightDarkSwitch`, atteso post) | Compila su toolchain stable senza feature `nightly`? | ref |
|---|---|---|---|---|---|
| `channel = "nightly"` | `channel = "stable"` (o file rimosso) | assente | `dark_mode_enabled.get()` (nel `view!` e nella closure `Show when=`) | `true` | AC-5 |

## Component: Corrispondenza versioni Cargo.toml/Cargo.lock con ADR-001

| Dependency | Cargo.toml (range atteso) | Cargo.lock (versione risolta attesa) | ref |
|---|---|---|---|
| `leptos` | `"0.8"` (senza feature `nightly`) | `0.8.20` | AC-6 |
| `leptos_meta` | `"0.8"` (senza feature `nightly`) | `0.8.6` | AC-6 |
| `leptos_router` | `"0.8"` (senza feature `nightly`) | `0.8.15` | AC-6 |
| `leptos_actix` | `"0.8"` | `0.8.7` | AC-6 |
| `leptos-use` | `"0.19"` | `0.19.0` | AC-6 |
| `leptos_icons` | `"0.7"` | `0.7.1` | AC-6 |
| `icondata` | `"0.7"` | `0.7.0` | AC-6 |
| `wasm-bindgen` | `"0.2"` | `0.2.127` *(risolta al momento di ADR-001; può driftare con un `cargo update` futuro, per design — vedi ADR-001 "Consequences")* | AC-6 |
| `actix-web` | `"4.14"` | `4.14.1` | AC-6 |

---

## Open Issues

- Riga `wasm-bindgen` in AC-6: il valore `0.2.127` è quello risolto al momento di ADR-001. Se `cargo update` produce una versione più recente entro il range `"0.2"` prima dell'implementazione di questa storia, la riga va aggiornata al valore effettivamente risolto — il test verifica la coerenza Cargo.toml/Cargo.lock, non un numero di build congelato.
- Riga AC-4 "HydrationScripts (route SSR, `src/main.rs`)": il codice attuale non ha uno `shell`/`HydrationScripts` esplicito in `src/main.rs` — è generato internamente da `leptos_actix::{generate_route_list, LeptosRoutes}`. Se la migrazione a `islands` in Leptos 0.8 richiede di renderlo esplicito (verificare in fase di implementazione), la riga resta valida sul risultato osservabile (`islands=true` è effettivo), non sulla sua collocazione nel codice.
