# EP-004: Motore di ricerca client-side

> Offrire una ricerca full-text sui contenuti del sito senza alcun backend di ricerca gestito dall'utente, coerente con un hosting statico a costo zero.

---

## Motivation

Nessun budget per un motore di ricerca server-side (Elasticsearch, un'istanza Meilisearch self-hosted, ecc.), e l'hosting su GitHub Pages (EP-005) non offre compute lato server. La ricerca deve quindi funzionare interamente nel browser del visitatore.

## Context

Nessuna ricerca esiste oggi. Opzioni allo studio segnalate dall'utente: lunr.js (indicizzazione e ricerca client-side), Orama (scartata per vicissitudini recenti del progetto), Meilisearch (da chiarire se richiede comunque un server o supporta indici trasportabili ed eseguibili lato client). La scelta tecnica va approfondita e fissata in un ADR quando questa epica viene affrontata.

## Business Outcome

- Un visitatore può cercare testo nei contenuti pubblicati e ottenere risultati pertinenti
- Nessuna chiamata di rete verso un backend di ricerca gestito dall'utente

## Constraints

| Type | Constraint |
|---|---|
| Business | Costo di infrastruttura server-side pari a zero |
| Technical | L'indice di ricerca deve poter essere generato a build-time, coerente con l'output statico di EP-003 |

## Scope

### Out of scope
- Ricerca semantica/AI-powered
- Ranking avanzato o suggerimenti

## Open Issues

- Scelta definitiva tra lunr.js, Meilisearch (in modalità indice portabile, se esiste) o altra libreria — da chiarire con uno spike prima di committarsi

---

| Updated | What changed |
|---|---|
| 2026-08-13 | Epic created (stub sintetico — orizzonte Next) |
