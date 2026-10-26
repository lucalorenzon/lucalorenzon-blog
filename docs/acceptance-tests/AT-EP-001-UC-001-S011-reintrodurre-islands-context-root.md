# AT-EP-001-UC-001-S011: Reintrodurre le islands (menu e header dinamico) sotto `CONTEXT_ROOT`

Epic: EP-001 | UC: UC-001 (+ UC-004, UC-005) | Story: EP-001-UC-001-S011

> **Righe spostate il 2026-10-26** da
> [AT-S006](AT-EP-001-UC-001-S006-servire-sito-context-root-configurabile.md),
> dove erano già approvate, senza riscriverle. **Da completare in
> `/acceptance-tests`:** menu presente nell'header, islands in sviluppo
> locale, zero 404 su js/wasm sotto prefisso.

Valori di riferimento: `output-name` = `blog_start`.

---

## Percorsi generati: script di hydration

> Source: Story AC-1. Valore dell'attributo nell'HTML di ogni pagina generata.

| elemento | con `CONTEXT_ROOT` = `/lucalorenzon-blog/`? | con `CONTEXT_ROOT` = `/`? | ref |
|---|---|---|---|
| script di hydration (`<link rel="modulepreload">` `href`) | `/lucalorenzon-blog/pkg/blog_start.js` | `/pkg/blog_start.js` | AC-1 |
| modulo wasm (`<link rel="preload">` `href`) | `/lucalorenzon-blog/pkg/blog_start.wasm` | `/pkg/blog_start.wasm` | AC-1 |

---

## Islands sotto prefisso

> Source: Story AC-3, AC-4. Browser reale, sito servito con `CONTEXT_ROOT` = `/lucalorenzon-blog/`, pagina `/lucalorenzon-blog/`.

| azione del visitatore | `<html>` ha la classe `dark`? | testo del selettore tema? | `<header>` ha la classe `bg-white`? | ref |
|---|---|---|---|---|
| pagina appena caricata, in cima | `true` | `Enable light mode` | `false` | AC-3 |
| clic sul selettore tema | `false` | `Enable dark mode` | `false` | AC-3 |
| secondo clic sul selettore tema | `true` | `Enable light mode` | `false` | AC-3 |
| scroll oltre la cima della pagina | `true` | `Enable light mode` | `true` | AC-4 |

---

## Open Issues

- **Host statico e browser reale** per le righe "Islands": manuali finché
  non c'è l'infrastruttura Playwright di [[EP-001-S010]].
- **Righe dell'header**: dipendono dal fatto che la pagina sia abbastanza
  lunga da scorrere; nel test va fissata un'altezza del viewport minore
  della pagina.
- **Righe da aggiungere** (vedi nota in cima).
