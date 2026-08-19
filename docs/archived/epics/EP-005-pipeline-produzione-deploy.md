# EP-005: Pipeline di produzione — CI/CD, deploy GitHub Pages, dominio custom

> Rendere pubblicabile il sito con un semplice push su git, raggiungibile dal dominio personale dell'utente.

---

## Motivation

Il sito non è mai stato deployato. Perché il rilancio del progetto abbia senso, pubblicare un aggiornamento deve essere un'azione a basso attrito (push su git), non una procedura manuale.

## Context

Nessuna pipeline CI/CD esiste oggi. L'utente possiede i domini `lucalorenzon.it` e `lucalorenzon.com`, non ancora collegati a nulla. Il sito, una volta online, non deve necessariamente essere subito pubblico a tutti: l'utente vuole prima organizzare i contenuti (EP-007) prima di renderlo visibile.

## Business Outcome

- Un push su git pubblica automaticamente il sito su GitHub Pages
- Il sito è raggiungibile dal dominio custom quando l'utente decide di collegarlo
- Il sito può restare accessibile solo all'utente (o comunque non pubblicizzato) finché i contenuti non sono pronti

## Constraints

| Type | Constraint |
|---|---|
| Technical | GitHub Pages: hosting statico gratuito, richiede l'output di EP-003 |
| Business | Dominio già posseduto, va solo collegato via DNS |

## Scope

### Out of scope
- Selezione e pubblicazione dei contenuti reali (EP-007)

## Open Issues

- Verificare se GitHub Pages consente di pubblicare un sito raggiungibile solo dal proprietario (repo privato + Pages, o altra soluzione), oppure se conviene tenerlo pubblico ma con contenuti minimi finché non è pronto

---

| Updated | What changed |
|---|---|
| 2026-08-13 | Epic created (stub sintetico — orizzonte Next) |
