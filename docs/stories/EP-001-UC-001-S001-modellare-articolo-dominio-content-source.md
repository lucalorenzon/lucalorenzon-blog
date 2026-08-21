## EP-001-UC-001-S001 — Modellare l'articolo e la porta ContentSource

**Decision:** Quali invarianti codifica il tipo di dominio `Article` (parse-dont-validate su data, slug, tag, titolo, abstract, immagine di sintesi) e come si legge un frontespizio dal repo contenuti dedicato tramite una porta `ContentSource` con adapter filesystem.

As Autore/Editore, voglio che un frontespizio ben formato venga riconosciuto come articolo valido (e uno malformato venga rifiutato) so that non posso pubblicare per errore un articolo con metadati incompleti o inconsistenti.

| Field | Value |
|---|---|
| **Epic** | EP-001 — Rilancio del blog personale come presenza professionale |
| **UC** | UC-001 — Autore pubblica un nuovo articolo |
| **Pattern** | Major effort isolation + Business rule variations |
| **AT rows** | AT-UC-001 righe: 1-5 (happy path, limitatamente a "frontmatter valido → costruzione riuscita"), 3a (data/slug/tag mancanti o malformati) |

### Acceptance criteria
- Given un frontespizio con data, slug, tag, titolo, abstract e immagine di sintesi tutti presenti e ben formati, when il sistema lo legge dal repo contenuti tramite `ContentSource`, then costruisce un `Article` valido con quei metadati
- Given un frontespizio con data mancante o malformata, when il sistema tenta di costruire l'`Article`, then la costruzione è rifiutata e l'errore identifica il campo `data` come causa
- Given un frontespizio con slug mancante o malformato, when il sistema tenta di costruire l'`Article`, then la costruzione è rifiutata e l'errore identifica il campo `slug` come causa
- Given un frontespizio con tag mancante o malformato, when il sistema tenta di costruire l'`Article`, then la costruzione è rifiutata e l'errore identifica il campo `tag` come causa

### Design pipeline
Before any implementation, complete in order:
- [ ] `/software-design`        — coupling, ownership, accidental complexity
- [ ] `/hexagonal-architecture` — ports, adapters, composition root
- [ ] `/parse-dont-validate`    — domain types and invariants
- [ ] `/sw-practices`           — naming, error handling, bootstrap

### Next steps after agreement
- [ ] `/acceptance-tests EP-001-UC-001-S001` — story-level AT table (narrower than UC level)
- [ ] `/story-size EP-001-UC-001-S001`       — assign XS / S / M / L / XL / XXL

### Open questions
- Nessuna nota bloccante allo split. La forma esatta dell'errore di costruzione (tipo Rust, messaggio) è materia del design pipeline (`parse-dont-validate`), non di questa story.

### Escalation
Questa decisione corrisponde all'ADR già raccomandato dall'epic ma non ancora creato: **"Content come markdown-in-git via porta ContentSource, nessun CMS per ora"**. Formalizzare con `/adr` prima o durante il design pipeline di questa story.

### Dependencies (infra)
Il repo contenuti dedicato (target del `ContentSource` filesystem adapter) non risulta ancora creato/configurato — nessuna evidenza in `docs/` di dove o come esista oggi. Prima di implementare l'adapter, chore prerequisito: **creare/configurare il repo contenuti dedicato su GitHub e verificarne permessi di scrittura ed eventuale branch protection**, includendo la verifica manuale dell'estensione UC-001 2a (push respinto per conflitto o permessi mancanti) — comportamento nativo git/GitHub, non applicativo, ma la cui configurazione va comunque controllata. Lane `chore`, non una story: nessuna Decision di dominio, solo config da verificare.

### Decisions Log
| Date | Decision | Reasoning | Alternatives Considered |
|---|---|---|---|

INVEST: I✓ N✓ V✓ E✓ S✓ T✓  |  1 Decision: ✓  |  Coherence: ✓
