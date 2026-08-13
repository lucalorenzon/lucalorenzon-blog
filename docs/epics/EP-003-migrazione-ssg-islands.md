# EP-003: Migrazione architetturale a SSG + islands

> Trasformare il sito da SSR+islands (server actix-web sempre attivo) a SSG+islands (output statico), per renderlo deployabile su GitHub Pages e risolvere alla radice il problema di peso del bundle WASM percepito nel setup attuale.

---

## Motivation

GitHub Pages serve solo contenuto statico: un backend SSR come quello attuale non è compatibile. Inoltre l'utente ha già segnalato che il setup SSR+islands attuale si sente nelle prestazioni di download del WASM — un problema che si affronta meglio a livello architetturale che con solo un aggiornamento di dipendenze (EP-001).

## Context

Oggi il sito è SSR+islands via `leptos_actix`/`actix-web`: ogni richiesta viene renderizzata da un processo server sempre attivo. Non esiste hosting gratuito adatto a questo modello nel piano dell'utente; GitHub Pages, gratuito e già disponibile, richiede file statici.

## Business Outcome

- Il sito genera un output completamente statico (HTML/CSS/JS/WASM) a build-time, senza necessità di un server applicativo
- Le islands restano interattive lato client come oggi
- Il bundle WASM è percepibilmente più leggero rispetto alla baseline SSR attuale

## Constraints

| Type | Constraint |
|---|---|
| Technical | GitHub Pages è hosting statico puro, nessun compute server-side |
| Technical | Da valutare in ADR se Leptos supporta nativamente un output SSG per le route con islands, o se serve una strategia di generazione manuale delle pagine |

## Scope

### Out of scope
- Motore di ricerca (EP-004)
- Deploy e CI/CD (EP-005)

## Open Issues

- Verificare il supporto SSG nella versione di Leptos scelta in EP-001 (potrebbe influenzare quale versione target scegliere in quell'epica)

---

| Updated | What changed |
|---|---|
| 2026-08-13 | Epic created (stub sintetico — orizzonte Next) |
