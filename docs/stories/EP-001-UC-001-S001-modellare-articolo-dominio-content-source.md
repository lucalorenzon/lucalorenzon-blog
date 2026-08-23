## EP-001-UC-001-S001 — Modellare l'articolo e la porta ContentSource

**Decision:** Quali invarianti codifica il tipo di dominio `Article` (parse-dont-validate: data, slug, uno o più tag, titolo obbligatori — costruzione rifiutata se mancanti/malformati; abstract e immagine di sintesi opzionali, non validati qui — il loro calcolo di default è responsabilità di [[EP-001-UC-001-S003]]) e la forma completa della porta `ContentSource` (lettura di un singolo articolo per slug + elenco degli articoli pubblicati), con adapter filesystem sul repo contenuti dedicato. S001 implementa solo la lettura singola; l'elenco è implementato da [[EP-001-UC-001-S002]]/[[EP-001-UC-001-S003]] sulla stessa interfaccia già concordata qui.

As Autore/Editore, voglio che un frontespizio ben formato venga riconosciuto come articolo valido (e uno malformato venga rifiutato) so that non posso pubblicare per errore un articolo con metadati incompleti o inconsistenti.

| Field | Value |
|---|---|
| **Epic** | EP-001 — Rilancio del blog personale come presenza professionale |
| **UC** | UC-001 — Autore pubblica un nuovo articolo |
| **Pattern** | Major effort isolation + Business rule variations |
| **AT rows** | AT-UC-001 righe: 1-5 (happy path, limitatamente a "frontmatter valido → costruzione riuscita"; righe "abstract assente"/"immagine assente" NON coperte qui, vedi [[EP-001-UC-001-S003]]), 3a (data/slug/tag/titolo mancanti o malformati) |
| **Size** | L — estimated 2026-08-21 |

### Acceptance criteria
- Given un frontespizio con data, slug, uno o più tag e titolo tutti presenti e ben formati, when il sistema lo legge dal repo contenuti tramite `ContentSource`, then costruisce un `Article` valido con quei metadati (abstract e immagine di sintesi, se presenti, vengono letti così come sono; se assenti, l'`Article` li rappresenta come opzionali assenti, senza rifiutare la costruzione)
- Given un frontespizio con data mancante o malformata, when il sistema tenta di costruire l'`Article`, then la costruzione è rifiutata e l'errore identifica il campo `data` come causa
- Given un frontespizio con slug mancante o malformato, when il sistema tenta di costruire l'`Article`, then la costruzione è rifiutata e l'errore identifica il campo `slug` come causa
- Given un frontespizio con lista di tag assente (vuota) o con un valore malformato, when il sistema tenta di costruire l'`Article`, then la costruzione è rifiutata e l'errore identifica il campo `tag` come causa
- Given un frontespizio con titolo mancante o malformato, when il sistema tenta di costruire l'`Article`, then la costruzione è rifiutata e l'errore identifica il campo `titolo` come causa
- Given la porta `ContentSource`, when se ne definisce l'interfaccia, then espone sia la lettura di un singolo articolo per slug sia l'elenco degli articoli pubblicati, anche se questa story implementa solo il primo metodo

### Design pipeline
Before any implementation, complete in order:
- [x] `/software-design`        — coupling, ownership, accidental complexity
- [x] `/hexagonal-architecture` — ports, adapters, composition root
- [x] `/parse-dont-validate`    — domain types and invariants
- [x] `/sw-practices`           — naming, error handling, bootstrap

Hexagonal design artefact: [docs/architecture/hexagonal.md](../architecture/hexagonal.md)
Parse-dont-validate design artefact: [docs/design/article.md](../design/article.md)

### Next steps after agreement
- [x] `/acceptance-tests EP-001-UC-001-S001` — story-level AT table (narrower than UC level) → [AT-EP-001-UC-001-S001](../acceptance-tests/AT-EP-001-UC-001-S001-modellare-articolo-dominio-content-source.md)
- [ ] `/story-size EP-001-UC-001-S001`       — assign XS / S / M / L / XL / XXL

### Acceptance Tests
[AT-EP-001-UC-001-S001-modellare-articolo-dominio-content-source](../acceptance-tests/AT-EP-001-UC-001-S001-modellare-articolo-dominio-content-source.md)

### Open questions
- Nessuna nota bloccante allo split. La forma esatta dell'errore di costruzione (tipo Rust, messaggio) è materia del design pipeline (`parse-dont-validate`), non di questa story.

### Escalation
Questa decisione corrisponde all'ADR già raccomandato dall'epic ma non ancora creato: **"Content come markdown-in-git via porta ContentSource, nessun CMS per ora"**. Formalizzare con `/adr` prima o durante il design pipeline di questa story. → Formalizzata: [ADR-002](../adr/ADR-002-content-markdown-in-git.md) (2026-08-23).

### Dependencies (infra)
Il repo contenuti dedicato (target del `ContentSource` filesystem adapter) non risulta ancora creato/configurato — nessuna evidenza in `docs/` di dove o come esista oggi. Prima di implementare l'adapter, chore prerequisito: **creare/configurare il repo contenuti dedicato su GitHub e verificarne permessi di scrittura ed eventuale branch protection**, includendo la verifica manuale dell'estensione UC-001 2a (push respinto per conflitto o permessi mancanti) — comportamento nativo git/GitHub, non applicativo, ma la cui configurazione va comunque controllata. Lane `chore`, non una story: nessuna Decision di dominio, solo config da verificare.

### Decisions Log
| Date | Decision | Reasoning | Alternatives Considered |
|---|---|---|---|
| 2026-08-21 | Titolo obbligatorio quanto data/slug/tag (costruzione rifiutata se assente/malformato); abstract e immagine di sintesi restano opzionali nell'`Article`, il loro default è calcolato a build time da [[EP-001-UC-001-S003]], non qui | Riduce l'attrito di pubblicazione senza sacrificare i metadati che guidano URL/categorizzazione (data/slug/tag) e la user experience minima (titolo) | Rendere abstract/immagine obbligatorie quanto gli altri campi — scartata, più attrito per un blog personale |
| 2026-08-21 | `tag` è una lista non vuota di uno o più valori, non un valore singolo | Coerente con menu a tag statici (UC-005) e con articoli che coprono più argomenti | Tag singolo per articolo — scartata, meno flessibile |
| 2026-08-21 | La porta `ContentSource` è progettata ora con interfaccia completa (lettura singola per slug + elenco articoli pubblicati); S001 implementa solo la lettura singola | Evita di ridisegnare il port quando S002 (unicità slug) e S003 (listing) avranno bisogno dell'elenco | Progettare solo "read one" ora e aggiungere "list" più avanti — scartata, rischio di retrofit del port |
| 2026-08-21 | `ContentSource` resta un solo trait Rust con entrambi i metodi dichiarati; `get_by_slug` implementato da S001 con logica reale, `list_published` implementato da S001 con un tipo di errore esplicito (es. `NotYetSupported`), sostituito da S002 con la logica reale | Un errore tipato ed esplicito rappresenta onestamente l'assenza della capacità (nessun `todo!()`/stub silenzioso, coerente con parse-dont-validate); evita anche la segregazione in più trait (`ArticleReader`/`ArticleLister`), complessità accidentale non giustificata da un bisogno reale — fetch-lista è oggi una sola variazione dello stesso concetto di fetch, non un'operazione indipendente; eventuali variazioni future (ricerca, ricerca filtrata) restano fuori scope finché non avranno una story propria | (a) Implementare già in S001 la logica reale di `list_published` (walk-directory) — scartata, sposterebbe fuori dal perimetro AC/AT di S001 un comportamento non testato qui; (b) segregare `ContentSource` in due trait indipendenti — scartata, complessità accidentale per un bisogno di dominio che oggi è unico |
| 2026-08-23 | Formato di ciascun campo obbligatorio deciso in `/parse-dont-validate`: `PublicationDate` = `YYYY-MM-DD`, calendario reale (validazione bisestile inclusa); `Slug` e `Tag` = kebab-case ASCII minuscolo (`^[a-z0-9]+(-[a-z0-9]+)*$`), stesso charset per entrambi; `Title` = non vuoto dopo trim, nessun carattere di controllo, nessun limite di lunghezza. `Tags` introdotto come newtype dedicato (non solo un controllo di lunghezza in `Article::new`) per rendere irrappresentabile l'articolo senza tag | Rende esplicito e testabile ciò che l'AT lasciava come `?UNKNOWN?`; il charset condiviso Slug/Tag evita un secondo tipo "tag slug" quando UC-005 (menu a tag) arriverà; `Tags` come tipo dedicato è coerente con "illegal states unrepresentable" più di un controllo runtime dentro `Article::new` | Tag come testo libero — scartata, richiederebbe un secondo tipo derivato per l'URL del tag; validare la lunghezza dei tag dentro `Article::new` anziché in un tipo `Tags` dedicato — scartata, lo stato illegale resterebbe rappresentabile a livello di tipo |
| 2026-08-23 | `Article::new` verifica i campi in ordine fisso (data, slug, tag, titolo) e ritorna al primo errore; nessuna aggregazione multi-campo in questa story | Ogni riga AC/AT di questa story esercita un solo campo invalido alla volta; aggregare gli errori (es. `Vec<ArticleError>`) aggiungerebbe una forma di ritorno che nessun AC/AT richiede oggi | Aggregare tutti gli errori di `RawFrontmatter` in una sola `Result` con `Vec` di cause — scartata come complessità non richiesta, valutabile in una story futura se emerge un bisogno UX reale (mostrare tutti gli errori di frontmatter insieme) |
| 2026-08-23 | Dipendenze per il layer di dominio: `thiserror` adottato per `ArticleError` e i 5 errori dei value object; `anyhow` escluso dal dominio (riservato, se servirà, al boundary applicativo di una story futura, da valutare quando ci si arriva); validazione della data scritta a mano, `std`-only (la std non ha alcun tipo/parser calendario) | `thiserror` è a costo zero a runtime (proc-macro) ed è lo standard idiomatico per enum di errore piccoli e tipati — evita boilerplate manuale su `Display`/`Error`; `anyhow` farebbe type-erasure e romperebbe il match statico sul campo causa richiesto dall'AC; la data non richiede una dipendenza come `chrono` finché non emerge un bisogno reale di date arithmetic (es. S003) | `chrono` per la data — rimandata, non scartata, si rivaluta se S003 ne avrà bisogno; errori scritti a mano senza `thiserror` — scartata dopo revisione, boilerplate ingiustificato per 6 enum quasi identici |
| 2026-08-21 | Errore del port unificato in un solo tipo `FetchError` con quattro varianti (`NotFound`, `Io`, `Malformed(ArticleError)`, `NotImplemented`), condiviso da `get_by_slug` e `list_published` invece di due tipi separati; confine adapter/dominio reso esplicito da un tipo boundary con campi stringa grezzi (letto dal frontespizio, non ancora validato), consumato una sola volta da `Article::new` e mai propagato oltre — che a sua volta costruisce i value type interni (`PublicationDate`, `Slug`, `Tag`, `Title`), ciascuno col proprio smart constructor. Confermata anche la classificazione di `Article`/`ContentSource` come subdominio Supporting: il vero differenziatore (qualità editoriale) è esplicitamente fuori dal perimetro tecnico del sistema (out of scope in EP-001), quindi nessun modulo tecnico può rivendicare lo status di Core — sono tutti al servizio di quel core esterno | `NotFound` si applica a entrambi i metodi (slug inesistente, ma anche repo contenuti mancante per `list_published`); solo `NotImplemented` è asimmetrico e temporaneo — un enum unico evita due tipi quasi identici per una sola variante temporanea. Il tipo boundary tiene la validazione centralizzata nel dominio (un solo posto dove i campi vengono controllati), non nell'adapter — coerente con parse-dont-validate. Decisione incrociata con un run indipendente di `/modularity:design` (branch `experiment/modularity-plugin-trial`, mai mergiato) su cui questa story si è convergentemente allineata senza condividerne in anticipo i dettagli — vedi `docs/licences/third-party-usage.md` | Due tipi di errore separati per `get_by_slug`/`list_published` — scartata, duplica una struttura quasi identica per una sola variante; validazione fatta direttamente nell'adapter invece che in un tipo boundary dedicato — scartata, sposterebbe la conoscenza del formato fuori dal dominio |

INVEST: I✓ N✓ V✓ E✓ S✓ T✓  |  1 Decision: ✓  |  Coherence: ✓
