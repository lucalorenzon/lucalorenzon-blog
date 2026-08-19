## EP-001-S003 — Verificare e correggere le funzionalità interattive (islands) dopo l'aggiornamento

**Decision:** Isolare la verifica/correzione delle islands interattive (menu, toggle dark/light) come slice a sé, perché è la parte a rischio più alto vista la migrazione del feature flag `experimental-islands` → `islands` fatta in EP-001-S001.

Obiettivo tecnico: garantire che le funzionalità interattive esistenti (apertura/chiusura menu, toggle dark/light) continuino a funzionare dopo l'aggiornamento, così che l'utente finale del sito non percepisca alcuna differenza.

| Field | Value |
|---|---|
| **Epic** | EP-001 — Rilancio del sito su stack Leptos/Rust aggiornato |
| **UC** | N/A — vedi EP-001-S001 |
| **Pattern** | Simple/Complex (parte "complessa": hydration client-side delle islands) + Major effort isolation |
| **AT rows** | AT-EP-001: tabella "Funzionalità interattive (islands)" (ref AC-3) |

### Acceptance criteria
- Given il sito è in esecuzione (dopo EP-001-S001), when si clicca sul toggle del menu (`DynamicHeader`), then lo stato aperto/chiuso del menu si inverte, come prima dell'aggiornamento
- Given il sito è in esecuzione, when si clicca sul toggle del tema (`LightDarkSwitch`), then la classe `dark` sull'elemento radice si attiva/disattiva, come prima dell'aggiornamento

### Design pipeline
Before any implementation, complete in order:
- [ ] `/software-design`        — coupling, ownership, accidental complexity
- [ ] `/hexagonal-architecture` — ports, adapters, composition root
- [ ] `/parse-dont-validate`    — domain types and invariants
- [ ] `/sw-practices`           — naming, error handling, bootstrap

### Next steps after agreement
- [ ] `/acceptance-tests EP-001-S003` — story-level AT table
- [ ] `/story-size EP-001-S003`       — assign XS / S / M / L / XL / XXL

### Open questions
- Dipende dal completamento di EP-001-S001
- Da verificare se il guard `is_browser()` usato dalle islands (vedi CLAUDE.md del progetto) richiede modifiche con il nuovo entrypoint `hydrate_islands()`

INVEST: I✓ *(sequenziale dopo S001, nessuna dipendenza circolare)* N✓ V✓ E✓ S✓ T✓  |  1 Decision: ✓
