## EP-001-UC-001-S003 — Generare il sito statico da un Article (ARTICLE-PAGE, LISTING-PAGE, HOME-PAGE)

**Decision:** Con quale meccanismo il sistema trasforma un `Article` valido in output HTML statico distribuibile — ARTICLE-PAGE del nuovo articolo, più aggiornamento di LISTING-PAGE e HOME-PAGE — sostituendo la generazione server-side (SSR) attuale.

As Autore/Editore, voglio che il mio articolo compaia come pagina statica generata dal sito so that i visitatori possano leggerlo senza dipendere da un processo server sempre attivo.

| Field | Value |
|---|---|
| **Epic** | EP-001 — Rilancio del blog personale come presenza professionale |
| **UC** | UC-001 — Autore pubblica un nuovo articolo |
| **Pattern** | Major effort isolation |
| **AT rows** | AT-UC-001 righe: 1-5 (happy path, limitatamente a "build riuscita → ARTICLE-PAGE/LISTING-PAGE/HOME-PAGE generate correttamente"; il deploy stesso è fuori da questa story, vedi [[EP-001-UC-001-S005]]) |

### Acceptance criteria
- Given un `Article` valido prodotto da [[EP-001-UC-001-S001]], when il build statico viene eseguito, then genera una ARTICLE-PAGE raggiungibile allo slug dell'articolo con metadati e contenuto corretti
- Given un `Article` valido pubblicato, when il build statico viene eseguito, then la LISTING-PAGE include una voce per il nuovo articolo
- Given un `Article` valido pubblicato, when il build statico viene eseguito, then la HOME-PAGE (caso particolare cronologico della LISTING-PAGE) riflette il nuovo articolo secondo l'ordine cronologico

### Design pipeline
Before any implementation, complete in order:
- [ ] `/software-design`        — coupling, ownership, accidental complexity
- [ ] `/hexagonal-architecture` — ports, adapters, composition root
- [ ] `/parse-dont-validate`    — domain types and invariants
- [ ] `/sw-practices`           — naming, error handling, bootstrap

### Next steps after agreement
- [ ] `/acceptance-tests EP-001-UC-001-S003` — story-level AT table (narrower than UC level)
- [ ] `/story-size EP-001-UC-001-S003`       — assign XS / S / M / L / XL / XXL

### Open questions
- Meccanismo di paginazione della LISTING-PAGE non deciso (già segnalato come Open Issue in EP-001 e in UC-003) — non blocca questa story se si assume una LISTING-PAGE senza paginazione come primo incremento, ma va riallineato quando UC-003 verrà splittata.

### Escalation
Questa decisione corrisponde all'ADR già raccomandato dall'epic ma non ancora creato: **"SSG + islands al posto di SSR"**. Formalizzare con `/adr` prima o durante il design pipeline di questa story — è la decisione più impattante e meno reversibile dell'intera UC-001 (tocca l'intero bootstrap dell'applicazione, non solo la pubblicazione articoli).

### Dependencies
- Richiede il tipo `Article` di [[EP-001-UC-001-S001]].

### Decisions Log
| Date | Decision | Reasoning | Alternatives Considered |
|---|---|---|---|

INVEST: I✓ N✓ V✓ E✓ S✓ T✓  |  1 Decision: ✓  |  Coherence: ✓
