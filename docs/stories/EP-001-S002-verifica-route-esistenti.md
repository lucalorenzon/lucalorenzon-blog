## EP-001-S002 — Verificare e correggere le route esistenti dopo l'aggiornamento

**Decision:** Trattare la verifica/correzione delle route SSR (home, 404) come slice separato dalla verifica delle islands interattive (EP-001-S003), perché sono meccanismi diversi (routing/rendering server-side vs hydration client-side) con modalità di rottura indipendenti.

Obiettivo tecnico: garantire che tutte le route esistenti restino raggiungibili e visivamente equivalenti dopo l'aggiornamento, così che l'utente finale del sito non percepisca alcuna differenza.

| Field | Value |
|---|---|
| **Epic** | EP-001 — Rilancio del sito su stack Leptos/Rust aggiornato |
| **UC** | N/A — vedi EP-001-S001 |
| **Pattern** | Simple/Complex (parte "semplice": routing SSR, rispetto alle islands interattive di EP-001-S003) |
| **AT rows** | AT-EP-001: tabella "Route" (ref AC-2) |

### Acceptance criteria
- Given il sito è in esecuzione (dopo EP-001-S001), when si richiede `GET /`, then la risposta ha status `200` e il contenuto è equivalente alla baseline pre-aggiornamento
- Given il sito è in esecuzione, when si richiede una route inesistente (es. `GET /qualcosa-che-non-esiste`), then la risposta ha status `404` (impostato esplicitamente in `src/app.rs:59`) e il contenuto è equivalente alla baseline pre-aggiornamento

### Design pipeline
Before any implementation, complete in order:
- [ ] `/software-design`        — coupling, ownership, accidental complexity
- [ ] `/hexagonal-architecture` — ports, adapters, composition root
- [ ] `/parse-dont-validate`    — domain types and invariants
- [ ] `/sw-practices`           — naming, error handling, bootstrap

### Next steps after agreement
- [ ] `/acceptance-tests EP-001-S002` — story-level AT table
- [ ] `/story-size EP-001-S002`       — assign XS / S / M / L / XL / XXL

### Open questions
- Dipende dal completamento di EP-001-S001 (il sito deve compilare ed eseguire prima di poter verificare le route)

INVEST: I✓ *(sequenziale dopo S001, nessuna dipendenza circolare)* N✓ V✓ E✓ S✓ T✓  |  1 Decision: ✓
