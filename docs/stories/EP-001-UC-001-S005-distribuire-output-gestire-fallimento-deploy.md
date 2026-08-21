## EP-001-UC-001-S005 — Distribuire l'output generato e gestire il fallimento di deploy

**Decision:** Con quale meccanismo l'output statico generato viene distribuito in produzione, e cosa succede quando il deploy fallisce — il sistema deve mantenere attiva l'ultima versione distribuita con successo e informare l'autore tramite lo stesso canale (check fallito su GitHub Actions + email), essendo il deploy uno step della stessa pipeline.

As Autore/Editore, voglio che un deploy fallito non lasci il sito in uno stato rotto e che io ne sia informato so that i visitatori vedono sempre una versione funzionante del sito, anche quando la mia ultima pubblicazione non è andata a buon fine.

| Field | Value |
|---|---|
| **Epic** | EP-001 — Rilancio del blog personale come presenza professionale |
| **UC** | UC-001 — Autore pubblica un nuovo articolo |
| **Pattern** | Workflow steps + Simple/Complex |
| **AT rows** | AT-UC-001 righe: 1-5 (happy path, limitatamente a "distribuzione riuscita → sito aggiornato servito"), 4a (distribuzione fallisce) |

### Acceptance criteria
- Given un output statico generato con successo da [[EP-001-UC-001-S003]], when il deploy riesce, then il sito servito ai visitatori riflette la nuova versione, con l'articolo raggiungibile
- Given un output statico generato con successo, when il deploy fallisce, then il sistema mantiene attiva l'ultima versione distribuita con successo (non la nuova build)
- Given un deploy fallito, when la pipeline lo rileva, then l'autore è informato tramite check fallito su GitHub Actions con dettaglio nel log, più email di notifica standard di GitHub, e può ripetere la pubblicazione

### Design pipeline
Before any implementation, complete in order:
- [ ] `/software-design`        — coupling, ownership, accidental complexity
- [ ] `/hexagonal-architecture` — ports, adapters, composition root
- [ ] `/parse-dont-validate`    — domain types and invariants
- [ ] `/sw-practices`           — naming, error handling, bootstrap

### Next steps after agreement
- [ ] `/acceptance-tests EP-001-UC-001-S005` — story-level AT table (narrower than UC level)
- [ ] `/story-size EP-001-UC-001-S005`       — assign XS / S / M / L / XL / XXL

### Open questions
- Meccanismo di deploy concreto (target di hosting, come "l'ultima versione distribuita con successo" resta servita — es. atomic swap, versioning per commit) non ancora deciso; materia del design pipeline di questa story, non risolvibile a livello di story-split.

### Dependencies
- Richiede l'output generato da [[EP-001-UC-001-S003]] e si aggancia allo stesso canale di segnalazione errori introdotto da [[EP-001-UC-001-S004]].

### Decisions Log
| Date | Decision | Reasoning | Alternatives Considered |
|---|---|---|---|

INVEST: I✓ N✓ V✓ E✓ S✓ T✓  |  1 Decision: ✓  |  Coherence: ✓
