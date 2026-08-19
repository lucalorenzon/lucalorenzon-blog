# EP-002: Copertura di test automatica

> Dotare il sito di una suite di unit test e test end-to-end reali, oggi assente, per proteggere le epiche successive da regressioni silenziose.

---

## Motivation

Oggi non esiste alcuna copertura di test reale: solo lo scaffold Playwright di default del template Leptos, mai adattato al sito effettivo. Man mano che il progetto avanza su epiche ad alto rischio di rottura (EP-001 aggiornamento stack, EP-003 migrazione a SSG, EP-005 pipeline CI/CD), l'assenza di test rende ogni cambiamento una scommessa verificabile solo a occhio.

## Context

`end2end/tests/example.spec.ts` verifica ancora il titolo "Welcome to Leptos!" del template — non un contenuto del sito reale. Non esiste alcun unit test Rust nel crate.

## Business Outcome

- Esiste una suite di unit test Rust che copre la logica non banale dei componenti
- Esiste una suite Playwright che copre almeno il percorso utente principale (caricamento homepage, interazione con menu/toggle)
- L'intera suite è eseguibile con un singolo comando

## Constraints

| Type | Constraint |
|---|---|
| Business | Manutenuta da una sola persona: la suite deve restare piccola e mirata, non esaustiva |
| Technical | Deve restare eseguibile in CI (dipendenza con EP-005) |

## Scope

### Out of scope
- Test di performance/carico
- Test di accessibilità approfonditi (EP-006)

## Open Issues

- Da riprendere quando EP-001 è completata (i test vanno scritti contro l'interfaccia stabilizzata, non contro codice in corso di aggiornamento)

---

| Updated | What changed |
|---|---|
| 2026-08-13 | Epic created (stub sintetico — orizzonte Next) |
