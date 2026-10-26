## EP-001-UC-001-S004 — Rigenerare e pubblicare il sito a ogni push su `main` del repo contenuti

> Riscritta nel replan del 2026-10-26: prima copriva anche validazione e build da sorgente, ora divise fra [[EP-001-UC-001-S007]] (validazione) e [[EP-001-UC-001-S008]] (artefatti fissi). **Ordine di consegna: 5° di 7** (S006 → S005 → S007 → S008 → **S004** → S009 → S010).

**Decision:** Come un push su `main` del repo contenuti avvia la pipeline di questo repo (trigger e relativa credenziale, custodita nel repo contenuti), che genera il sito usando gli artefatti fissi già pubblicati invece di ricompilare il codice, e come l'esito, compreso un fallimento, arriva all'autore sul suo stesso push.

As Autore/Editore, voglio che il solo push su `main` del repo contenuti pubblichi l'articolo e che ogni errore mi arrivi sul mio push, come check fallito ed email, so that non devo eseguire comandi manuali e so subito se qualcosa è andato storto.

| Field | Value |
|---|---|
| **Epic** | EP-001 — Rilancio del blog personale come presenza professionale |
| **UC** | UC-001 — Autore pubblica un nuovo articolo |
| **Pattern** | Workflow steps |
| **AT rows** | AT-UC-001 righe: 1-5 (limitatamente a "push accettato → pipeline avviata automaticamente"), 3c (build fallisce); esito di 3a/3b propagato all'autore (il controllo stesso è in S007) |

### Acceptance criteria
- Given un push accettato su `main` del repo contenuti, when il push arriva, then la pipeline di `lucalorenzon-blog` parte automaticamente, senza comandi manuali dell'autore
- Given un push su un branch diverso da `main` del repo contenuti, when il push arriva, then non parte nessuna pipeline (lo staging resta privato)
- Given la pipeline avviata, when genera il sito, then usa gli artefatti fissi pubblicati da [[EP-001-UC-001-S008]] e non ricompila il codice
- Given un errore di validazione ([[EP-001-UC-001-S007]]) o di generazione, when la pipeline fallisce, then il check fallito è visibile sul push dell'autore nel repo contenuti, l'autore riceve l'email standard di GitHub e il sito già pubblicato resta invariato
- Given pipeline riuscita, when termina, then il sito pubblicato include il nuovo contenuto (deploy come in [[EP-001-UC-001-S005]])

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
- Come il workflow lato contenuti riflette l'esito della pipeline di questo repo, invece di risultare verde solo per aver dato il trigger (ADR-003, nota del 2026-10-26).
- Quale credenziale per il trigger: PAT fine-grained o GitHub App.
- Cosa succede se gli artefatti fissi non sono ancora disponibili (Release mancante o corrotta): fallimento esplicito, mai una ricompilazione silenziosa.

### Constraints agreed at split (2026-10-26)
- Topologia ADR-003 riconfermata: il repo contenuti (privato) dà solo il trigger, questo repo (pubblico) genera e pubblica.
- I log della pipeline sono pubblici: accettato, perché su `main` arriva solo ciò che va pubblicato.

### Dependencies
- [[EP-001-UC-001-S008]] (artefatti fissi), [[EP-001-UC-001-S007]] (validazione), [[EP-001-UC-001-S005]] (passo di deploy), [[EP-001-UC-001-S006]] (`CONTEXT_ROOT`).

### Decisions Log
| Date | Decision | Reasoning | Alternatives Considered |
|---|---|---|---|

INVEST: I✓ N✓ V✓ E✓ S✓ T✓  |  1 Decision: ✓  |  Coherence: ✓
