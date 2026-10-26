## EP-001-UC-001-S007 — Validare l'insieme dei contenuti prima della generazione

> Creata nel replan del 2026-10-26. **Ordine di consegna: 6° di 10** (S006 → S005 → S011 → S012 → S013 → **S007** → S008 → S004 → S009 → S010).

**Decision:** La validazione dei contenuti diventa un passo esplicito, eseguito una volta sull'intero insieme di articoli prima della generazione delle pagine. Si decide dove vive questo passo e come segnala gli errori: messaggi leggibili che nominano file e campo, codice di uscita diverso da zero, nessun file saltato in silenzio. Le pagine smettono di essere il luogo in cui un articolo malformato viene scoperto.

As Autore/Editore, voglio che un errore nei miei contenuti fermi la pubblicazione con un messaggio che mi dice quale file e quale campo correggere, so that non pubblico mai un articolo rotto e so subito come sistemarlo.

| Field | Value |
|---|---|
| **Epic** | EP-001 — Rilancio del blog personale come presenza professionale |
| **UC** | UC-001 — Autore pubblica un nuovo articolo |
| **Pattern** | Business rule variations |
| **AT rows** | AT-UC-001 righe: 3a (frontespizio mancante o malformato), 3b (slug duplicato) |

### Acceptance criteria
- Given un articolo con data, slug, tag o titolo mancanti o malformati, when la validazione gira, then fallisce prima della generazione, con un messaggio che nomina il file e il campo, e il processo esce con codice diverso da zero
- Given due articoli che dichiarano lo stesso slug, when la validazione gira, then fallisce nominando entrambi i file
- Given un file `.md` il cui nome non è uno slug valido, when la validazione gira, then fallisce invece di saltarlo in silenzio
- Given un insieme di contenuti valido, when la validazione gira, then la generazione procede
- Given la generazione delle pagine, when un articolo è malformato, then non è la pagina a scoprirlo: `ArticlePage` non converte più l'errore in "Article not found", e `ListingPage` non è più il punto in cui la build si interrompe

### Design pipeline
Before any implementation, complete in order:
- [ ] `/software-design`        — coupling, ownership, accidental complexity
- [ ] `/hexagonal-architecture` — ports, adapters, composition root
- [ ] `/parse-dont-validate`    — domain types and invariants
- [ ] `/sw-practices`           — naming, error handling, bootstrap

### Next steps after agreement
- [ ] `/acceptance-tests EP-001-UC-001-S007` — story-level AT table (narrower than UC level)
- [ ] `/story-size EP-001-UC-001-S007`       — assign XS / S / M / L / XL / XXL

### Open questions
- Slug nel frontespizio diverso dal nome del file: errore o no? Oggi la lettura usa il nome del file, mentre l'unicità riguarda lo slug dichiarato.
- `ensure_slug_is_unique` (S002) risponde a "questo slug è già usato?", non a "gli slug dell'insieme sono tutti diversi?": riusarlo, adattarlo o affiancargli un controllo sull'insieme.
- Il modello futuro "id + lista di slug" ([backlog](../backlog/slug-model-id-and-slug-list.md)) estenderà queste regole. Questa storia non lo anticipa.

### Dependencies
- Nessuna dipendenza di design. Serve a [[EP-001-UC-001-S004]] (esito per l'autore) e a [[EP-001-UC-001-S005]] (build su push di codice).

### Decisions Log
| Date | Decision | Reasoning | Alternatives Considered |
|---|---|---|---|

INVEST: I✓ N✓ V✓ E✓ S✓ T✓  |  1 Decision: ✓  |  Coherence: ✓
