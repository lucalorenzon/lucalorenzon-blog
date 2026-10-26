## EP-001-UC-001-S005 — Pubblicare il sito su GitHub Pages a ogni push di codice, compreso il primo deploy

> Riscritta nel replan del 2026-10-26: lo scope (deploy e ultima versione buona) resta, si aggiungono il primo deploy e la ripubblicazione a ogni push di codice. **Ordine di consegna: 2° di 7** (S006 → **S005** → S007 → S008 → S004 → S009 → S010) — è la storia che porta il sito online.

**Decision:** Con quale meccanismo l'output generato viene pubblicato su GitHub Pages a ogni push di codice su `main` (rigenerando con i contenuti attuali del repo contenuti), compreso il primo deploy, e come resta online l'ultima versione pubblicata con successo quando un deploy fallisce.

As Autore/Editore, voglio che ogni modifica al codice del sito arrivi online con i contenuti attuali, e che un deploy fallito non rompa il sito, so that i visitatori vedono sempre una versione funzionante e aggiornata.

| Field | Value |
|---|---|
| **Epic** | EP-001 — Rilancio del blog personale come presenza professionale |
| **UC** | UC-001 — Autore pubblica un nuovo articolo |
| **Pattern** | Workflow steps + Simple/Complex |
| **AT rows** | AT-UC-001 righe: 1-5 (limitatamente a "distribuzione riuscita → sito aggiornato servito"), 4a (distribuzione fallisce) |

### Acceptance criteria
- Given nessun sito ancora pubblicato, when la pipeline su `main` riesce, then il sito è raggiungibile all'indirizzo di GitHub Pages con stili, immagini, link e interattività funzionanti
- Given un push di codice su `main` con build riuscita, when il deploy riesce, then il sito servito riflette il nuovo codice con i contenuti attuali del repo contenuti
- Given un push di codice su `main`, when il deploy fallisce, then resta servita l'ultima versione pubblicata con successo, il check fallisce con dettaglio nel log e l'autore riceve l'email standard di GitHub
- Given una pull request (non un push su `main`), when la CI gira, then non avviene nessun deploy

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
- Verificare che il deploy di Pages tramite Actions sia atomico (un deploy fallito non sostituisce il sito servito). Se lo è, l'estensione 4a è comportamento nativo e non va costruita.
- Rapporto con il workflow `ci.yml` già esistente: deploy come job successivo dello stesso workflow o workflow separato.
- Credenziale di lettura del repo contenuti (ADR-003, Technical Notes): PAT fine-grained o deploy key.

### Constraints agreed at split (2026-10-26)
- Si parte dall'indirizzo di GitHub Pages; il dominio personale arriva dopo, come semplice cambio di configurazione ([[EP-001-UC-001-S006]]).

### Dependencies
- [[EP-001-UC-001-S006]] (`base_url`): senza, sotto `lucalorenzon.github.io/lucalorenzon-blog/` CSS, wasm, immagini e link interni restano rotti.
- Output generato da [[EP-001-UC-001-S003]].

### Decisions Log
| Date | Decision | Reasoning | Alternatives Considered |
|---|---|---|---|

INVEST: I✓ N✓ V✓ E✓ S✓ T✓  |  1 Decision: ✓  |  Coherence: ✓
