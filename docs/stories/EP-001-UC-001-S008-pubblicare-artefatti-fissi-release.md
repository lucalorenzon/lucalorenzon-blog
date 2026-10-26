## EP-001-UC-001-S008 — Costruire e pubblicare gli artefatti fissi come Release

> Creata nel replan del 2026-10-26: è la parte "artefatti" che l'utente aveva chiesto in S005, separata per rispettare una decisione per storia. **Ordine di consegna: 7° di 10** (S006 → S005 → S011 → S012 → S013 → S007 → **S008** → S004 → S009 → S010) — prima di S004, per scelta dell'utente: il push di contenuto non ricompila mai il codice.

**Decision:** Quali artefatti non cambiano quando cambia un contenuto e vengono quindi costruiti una sola volta per ogni push di codice: wasm, js, CSS, icone e asset statici, e il binario che genera l'HTML. Si decide anche dove vengono pubblicati (una Release, che a differenza degli artefatti di Actions non scade dopo 90 giorni) e come la pipeline dei contenuti sceglie la versione da usare.

Obiettivo tecnico: costruire il codice una sola volta e renderlo riusabile, così che un push di contenuto generi il sito senza ricompilare Rust, so that la pubblicazione di un articolo resti veloce ed economica in minuti di Actions. Assorbe il residuo #1 (build incrementale) dello Stressor Analysis di S003.

| Field | Value |
|---|---|
| **Epic** | EP-001 — Rilancio del blog personale come presenza professionale |
| **UC** | UC-001 — Autore pubblica un nuovo articolo |
| **Pattern** | Workflow steps (supporto tecnico al passo 3) |
| **AT rows** | Nessuna riga AT-UC-001 diretta: abilita le righe 1-5 di [[EP-001-UC-001-S004]]; epic AC-2 |

### Acceptance criteria
- Given un push di codice su `main` con CI verde, when la pipeline termina, then è pubblicata una Release con l'intero insieme di artefatti fissi
- Given gli artefatti fissi di una Release e un insieme di contenuti, when una pipeline li combina, then genera il sito completo senza compilare Rust
- Given oltre 90 giorni senza push di codice, when la pipeline dei contenuti cerca gli artefatti, then li trova ancora
- Given una CI rossa su `main`, when la pipeline termina, then non viene pubblicata nessuna Release

### Design pipeline
Before any implementation, complete in order:
- [ ] `/software-design`        — coupling, ownership, accidental complexity
- [ ] `/hexagonal-architecture` — ports, adapters, composition root
- [ ] `/parse-dont-validate`    — domain types and invariants
- [ ] `/sw-practices`           — naming, error handling, bootstrap

### Next steps after agreement
- [ ] `/acceptance-tests EP-001-UC-001-S008` — story-level AT table (narrower than UC level)
- [ ] `/story-size EP-001-UC-001-S008`       — assign XS / S / M / L / XL / XXL

### Open questions
- Versioning: la pipeline dei contenuti usa sempre l'ultima Release o una versione fissata? Cosa succede in caso di Release incompleta o corrotta (fail-closed, residuo #2)?
- Anche il deploy su push di codice ([[EP-001-UC-001-S005]]) deve passare dagli artefatti, così da avere un solo percorso di generazione?
- Target del binario generatore: statico musl come nel `Dockerfile`, o il runner `ubuntu-latest`?

### Dependencies
- Nessuna dipendenza di design. [[EP-001-UC-001-S004]] dipende da questa storia.

### Decisions Log
| Date | Decision | Reasoning | Alternatives Considered |
|---|---|---|---|

INVEST: I✓ N✓ V✓ E✓ S✓ T✓  |  1 Decision: ✓  |  Coherence: ✓
