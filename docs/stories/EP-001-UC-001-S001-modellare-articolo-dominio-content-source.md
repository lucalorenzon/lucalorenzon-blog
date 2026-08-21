## EP-001-UC-001-S001 — Modellare l'articolo e la porta ContentSource

**Decision:** Quali invarianti codifica il tipo di dominio `Article` (parse-dont-validate: data, slug, uno o più tag, titolo obbligatori — costruzione rifiutata se mancanti/malformati; abstract e immagine di sintesi opzionali, non validati qui — il loro calcolo di default è responsabilità di [[EP-001-UC-001-S003]]) e la forma completa della porta `ContentSource` (lettura di un singolo articolo per slug + elenco degli articoli pubblicati), con adapter filesystem sul repo contenuti dedicato. S001 implementa solo la lettura singola; l'elenco è implementato da [[EP-001-UC-001-S002]]/[[EP-001-UC-001-S003]] sulla stessa interfaccia già concordata qui.

As Autore/Editore, voglio che un frontespizio ben formato venga riconosciuto come articolo valido (e uno malformato venga rifiutato) so that non posso pubblicare per errore un articolo con metadati incompleti o inconsistenti.

| Field | Value |
|---|---|
| **Epic** | EP-001 — Rilancio del blog personale come presenza professionale |
| **UC** | UC-001 — Autore pubblica un nuovo articolo |
| **Pattern** | Major effort isolation + Business rule variations |
| **AT rows** | AT-UC-001 righe: 1-5 (happy path, limitatamente a "frontmatter valido → costruzione riuscita"; righe "abstract assente"/"immagine assente" NON coperte qui, vedi [[EP-001-UC-001-S003]]), 3a (data/slug/tag/titolo mancanti o malformati) |

### Acceptance criteria
- Given un frontespizio con data, slug, uno o più tag e titolo tutti presenti e ben formati, when il sistema lo legge dal repo contenuti tramite `ContentSource`, then costruisce un `Article` valido con quei metadati (abstract e immagine di sintesi, se presenti, vengono letti così come sono; se assenti, l'`Article` li rappresenta come opzionali assenti, senza rifiutare la costruzione)
- Given un frontespizio con data mancante o malformata, when il sistema tenta di costruire l'`Article`, then la costruzione è rifiutata e l'errore identifica il campo `data` come causa
- Given un frontespizio con slug mancante o malformato, when il sistema tenta di costruire l'`Article`, then la costruzione è rifiutata e l'errore identifica il campo `slug` come causa
- Given un frontespizio con lista di tag assente (vuota) o con un valore malformato, when il sistema tenta di costruire l'`Article`, then la costruzione è rifiutata e l'errore identifica il campo `tag` come causa
- Given un frontespizio con titolo mancante o malformato, when il sistema tenta di costruire l'`Article`, then la costruzione è rifiutata e l'errore identifica il campo `titolo` come causa
- Given la porta `ContentSource`, when se ne definisce l'interfaccia, then espone sia la lettura di un singolo articolo per slug sia l'elenco degli articoli pubblicati, anche se questa story implementa solo il primo metodo

### Design pipeline
Before any implementation, complete in order:
- [ ] `/software-design`        — coupling, ownership, accidental complexity
- [ ] `/hexagonal-architecture` — ports, adapters, composition root
- [ ] `/parse-dont-validate`    — domain types and invariants
- [ ] `/sw-practices`           — naming, error handling, bootstrap

### Next steps after agreement
- [x] `/acceptance-tests EP-001-UC-001-S001` — story-level AT table (narrower than UC level) → [AT-EP-001-UC-001-S001](../acceptance-tests/AT-EP-001-UC-001-S001-modellare-articolo-dominio-content-source.md)
- [ ] `/story-size EP-001-UC-001-S001`       — assign XS / S / M / L / XL / XXL

### Acceptance Tests
[AT-EP-001-UC-001-S001-modellare-articolo-dominio-content-source](../acceptance-tests/AT-EP-001-UC-001-S001-modellare-articolo-dominio-content-source.md)

### Open questions
- Nessuna nota bloccante allo split. La forma esatta dell'errore di costruzione (tipo Rust, messaggio) è materia del design pipeline (`parse-dont-validate`), non di questa story.

### Escalation
Questa decisione corrisponde all'ADR già raccomandato dall'epic ma non ancora creato: **"Content come markdown-in-git via porta ContentSource, nessun CMS per ora"**. Formalizzare con `/adr` prima o durante il design pipeline di questa story.

### Dependencies (infra)
Il repo contenuti dedicato (target del `ContentSource` filesystem adapter) non risulta ancora creato/configurato — nessuna evidenza in `docs/` di dove o come esista oggi. Prima di implementare l'adapter, chore prerequisito: **creare/configurare il repo contenuti dedicato su GitHub e verificarne permessi di scrittura ed eventuale branch protection**, includendo la verifica manuale dell'estensione UC-001 2a (push respinto per conflitto o permessi mancanti) — comportamento nativo git/GitHub, non applicativo, ma la cui configurazione va comunque controllata. Lane `chore`, non una story: nessuna Decision di dominio, solo config da verificare.

### Decisions Log
| Date | Decision | Reasoning | Alternatives Considered |
|---|---|---|---|
| 2026-08-21 | Titolo obbligatorio quanto data/slug/tag (costruzione rifiutata se assente/malformato); abstract e immagine di sintesi restano opzionali nell'`Article`, il loro default è calcolato a build time da [[EP-001-UC-001-S003]], non qui | Riduce l'attrito di pubblicazione senza sacrificare i metadati che guidano URL/categorizzazione (data/slug/tag) e la user experience minima (titolo) | Rendere abstract/immagine obbligatorie quanto gli altri campi — scartata, più attrito per un blog personale |
| 2026-08-21 | `tag` è una lista non vuota di uno o più valori, non un valore singolo | Coerente con menu a tag statici (UC-005) e con articoli che coprono più argomenti | Tag singolo per articolo — scartata, meno flessibile |
| 2026-08-21 | La porta `ContentSource` è progettata ora con interfaccia completa (lettura singola per slug + elenco articoli pubblicati); S001 implementa solo la lettura singola | Evita di ridisegnare il port quando S002 (unicità slug) e S003 (listing) avranno bisogno dell'elenco | Progettare solo "read one" ora e aggiungere "list" più avanti — scartata, rischio di retrofit del port |

INVEST: I✓ N✓ V✓ E✓ S✓ T✓  |  1 Decision: ✓  |  Coherence: ✓
