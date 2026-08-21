## EP-001-UC-001-S004 — Automatizzare la pipeline CI: trigger su push e segnalazione errori

**Decision:** Come la build viene innescata automaticamente da CI (GitHub Actions) al push sul repo contenuti — senza comando manuale dell'autore — eseguendo validazione frontespizio, verifica unicità slug e build statico in sequenza, e come ciascun fallimento viene segnalato all'autore (check fallito su GitHub Actions con dettaglio nel log, più email di notifica standard di GitHub).

As Autore/Editore, voglio che il solo push del file scateni automaticamente la pubblicazione e che ogni errore mi arrivi come check fallito e email so that non devo eseguire comandi manuali e sono avvisato immediatamente se qualcosa va storto.

| Field | Value |
|---|---|
| **Epic** | EP-001 — Rilancio del blog personale come presenza professionale |
| **UC** | UC-001 — Autore pubblica un nuovo articolo |
| **Pattern** | Workflow steps + Business rule variations |
| **AT rows** | AT-UC-001 righe: 1-5 (happy path, limitatamente a "push accettato → CI avviata automaticamente"), 3a (canale/formato errore di validazione), 3b (canale/formato errore di slug duplicato), 3c (build fallisce, canale/formato errore) |

### Acceptance criteria
- Given un push valido sul repo contenuti, when il push viene accettato, then CI (GitHub Actions) si avvia automaticamente senza intervento manuale dell'autore
- Given un frontespizio invalido rilevato dalla pipeline ([[EP-001-UC-001-S001]]), when CI esegue la validazione, then il check GitHub Actions fallisce con dettaglio nel log e l'autore riceve l'email di notifica standard di GitHub
- Given uno slug duplicato rilevato dalla pipeline ([[EP-001-UC-001-S002]]), when CI esegue la verifica di unicità, then il check GitHub Actions fallisce con dettaglio nel log e l'autore riceve l'email di notifica standard di GitHub
- Given un errore nel processo di generazione del sito ([[EP-001-UC-001-S003]]), when CI esegue il build, then il check GitHub Actions fallisce con dettaglio nel log e l'autore riceve l'email di notifica standard di GitHub, e il sito precedentemente distribuito resta invariato

### Design pipeline
Before any implementation, complete in order:
- [ ] `/software-design`        — coupling, ownership, accidental complexity
- [ ] `/hexagonal-architecture` — ports, adapters, composition root
- [ ] `/parse-dont-validate`    — domain types and invariants
- [ ] `/sw-practices`           — naming, error handling, bootstrap

### Next steps after agreement
- [ ] `/acceptance-tests EP-001-UC-001-S004` — story-level AT table (narrower than UC level)
- [ ] `/story-size EP-001-UC-001-S004`       — assign XS / S / M / L / XL / XXL

### Open questions
- Nessuna nota bloccante: trigger (CI-on-push) e canale errori (check fallito + email standard GitHub) sono già decisi a livello di UC-001 (Open Issues risolti 2026-08-21). Questa story ne è l'implementazione, non una nuova decisione di prodotto.

### Dependencies
- Wires insieme [[EP-001-UC-001-S001]] (validazione frontespizio), [[EP-001-UC-001-S002]] (unicità slug) e [[EP-001-UC-001-S003]] (build statico) in un'unica pipeline automatica.

### Decisions Log
| Date | Decision | Reasoning | Alternatives Considered |
|---|---|---|---|

INVEST: I✓ N✓ V✓ E✓ S✓ T✓  |  1 Decision: ✓  |  Coherence: ✓
