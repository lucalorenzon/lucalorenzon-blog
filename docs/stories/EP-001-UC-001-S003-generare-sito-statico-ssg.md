## EP-001-UC-001-S003 — Generare il sito statico da un Article (ARTICLE-PAGE, LISTING-PAGE, HOME-PAGE)

**Decision:** Con quale meccanismo il sistema trasforma un `Article` valido in output HTML statico distribuibile — ARTICLE-PAGE del nuovo articolo, più aggiornamento di LISTING-PAGE e HOME-PAGE — sostituendo la generazione server-side (SSR) attuale.

As Autore/Editore, voglio che il mio articolo compaia come pagina statica generata dal sito so that i visitatori possano leggerlo senza dipendere da un processo server sempre attivo.

| Field | Value |
|---|---|
| **Epic** | EP-001 — Rilancio del blog personale come presenza professionale |
| **UC** | UC-001 — Autore pubblica un nuovo articolo |
| **Pattern** | Major effort isolation |
| **AT rows** | AT-UC-001 righe: 1-5 (happy path, limitatamente a "build riuscita → ARTICLE-PAGE/LISTING-PAGE/HOME-PAGE generate correttamente"; il deploy stesso è fuori da questa story, vedi [[EP-001-UC-001-S005]]), righe happy path "abstract assente" e "immagine di sintesi assente" |

### Acceptance criteria
- Given un `Article` valido prodotto da [[EP-001-UC-001-S001]], when il build statico viene eseguito, then genera una ARTICLE-PAGE raggiungibile allo slug dell'articolo con metadati e contenuto corretti
- Given un `Article` valido pubblicato, when il build statico viene eseguito, then la LISTING-PAGE include una voce per il nuovo articolo
- Given un `Article` valido pubblicato, when il build statico viene eseguito, then la HOME-PAGE (caso particolare cronologico della LISTING-PAGE) riflette il nuovo articolo secondo l'ordine cronologico
- Given un `Article` con abstract assente, when il build statico viene eseguito, then l'abstract mostrato in ARTICLE-PAGE/LISTING-PAGE/HOME-PAGE è calcolato automaticamente dal corpo dell'articolo
- Given un `Article` con immagine di sintesi assente, when il build statico viene eseguito, then viene usata un'immagine di fallback predefinita in ARTICLE-PAGE/LISTING-PAGE/HOME-PAGE

### Design pipeline
Before any implementation, complete in order:
- [x] `/software-design`        — coupling, ownership, accidental complexity
- [ ] `/hexagonal-architecture` — ports, adapters, composition root
- [ ] `/parse-dont-validate`    — domain types and invariants
- [ ] `/sw-practices`           — naming, error handling, bootstrap

### Next steps after agreement
- [ ] `/acceptance-tests EP-001-UC-001-S003` — story-level AT table (narrower than UC level)
- [ ] `/story-size EP-001-UC-001-S003`       — assign XS / S / M / L / XL / XXL

### Open questions
- Meccanismo di paginazione della LISTING-PAGE non deciso (già segnalato come Open Issue in EP-001 e in UC-003) — non blocca questa story se si assume una LISTING-PAGE senza paginazione come primo incremento, ma va riallineato quando UC-003 verrà splittata.
- **Decision needed (non architetturale):** quale asset usare come immagine di fallback predefinita (AC "immagine di sintesi assente"). Non è una scelta di design ma di contenuto/brand — va confermata da Luca prima dell'implementazione. `assets/images/ostia_sea_top_image.webp` esiste già nel repo ma è oggi lo sfondo fisso del layout (`layout.rs:20`), non pensato come immagine segnaposto per articoli — riusarlo richiede conferma esplicita, non è un default silenzioso.

### Escalation
Questa decisione corrisponde a [ADR-004](../adr/ADR-004-ssg-islands-replaces-ssr.md) — **"SSG + islands al posto di SSR"**, formalizzata il 2026-08-24. Resta la decisione più impattante e meno reversibile dell'intera UC-001 (tocca l'intero bootstrap dell'applicazione, non solo la pubblicazione articoli); il meccanismo esatto di implementazione è demandato al design pipeline di questa story (ADR-004, sezione Technical Notes).

### Dependencies
- Richiede il tipo `Article` e la porta `ContentSource` di [[EP-001-UC-001-S001]].
- Richiede `ContentSource::list_published` — oggi `NotImplemented` su entrambi gli adapter, esplicitamente deferito a questa story da [[EP-001-UC-001-S002]] (vedi il suo Decisions Log, 2026-08-24: "di competenza di S003, LISTING-PAGE, non di questa story").
- Estende `Article` (S001) con un campo `body` (contenuto markdown) — assente oggi: `FilesystemContentSource` legge solo il blocco YAML e scarta tutto ciò che segue la seconda fence `---`.

### Decisions Log
| Date | Decision | Reasoning | Alternatives Considered |
|---|---|---|---|
| 2026-08-24 | `Article` guadagna un campo `body: String`, validato non-vuoto nello smart constructor (`Article::new`), stesso pattern di `date`/`slug`/`tags`/`title`. `FilesystemContentSource` legge il body come tutto il testo dopo la seconda fence YAML | Serve per generare il contenuto di ARTICLE-PAGE (AC-1) e come sorgente per l'abstract calcolato (AC-4); un body vuoto non è un articolo pubblicabile, quindi è un invariante reale da rifiutare a costruzione, non un default silenzioso | Lasciare `body` come `Option<String>` senza validazione — scartata: sposterebbe il controllo "articolo senza contenuto" a valle, in un punto meno vicino alla causa |
| 2026-08-24 | Il calcolo dell'abstract effettivo (truncation del `body` quando `abstract_text` è assente) e dell'immagine di sintesi effettiva (fallback a un asset predefinito quando `image` è assente) vive nella **presentation layer** (view-model consumato da ARTICLE-PAGE/LISTING-PAGE/HOME-PAGE), non in `Article` | `Article` resta un valore puramente parsato (parse-dont-validate senza logica derivata): la lunghezza di truncation e il path dell'asset di fallback sono decisioni di presentazione/brand, non invarianti del dominio. Tenerle fuori dal dominio evita che `Article` conosca path di asset infrastrutturali | Metodi calcolati su `Article` stesso (`effective_abstract()`, `effective_image()`) — scartata: accoppierebbe il dominio a una policy di rendering che può cambiare senza che il significato di "articolo" cambi |
| 2026-08-24 | `ContentSource::list_published` viene implementato ora su entrambi gli adapter (filesystem: enumera i file `.md` in `articles_dir` e riusa `get_by_slug` per ciascuno; in-memory: restituisce gli articoli inseriti). L'ordine cronologico richiesto da HOME-PAGE (AC-3) è una policy applicata a valle, in fase di build della pagina, non una garanzia del contratto della porta | La porta risponde solo "quali articoli esistono", senza impegnarsi su un ordine — tenere l'ordinamento fuori dal port contract evita di dover ridefinire la porta se in futuro cambia il criterio (es. per-tag, non solo cronologico) | Garantire l'ordine già dentro `list_published` — scartata: accoppierebbe il contratto della porta a una policy di presentazione che può cambiare indipendentemente |
| 2026-08-24 | Composition root SSG: `leptos_actix::generate_route_list_with_ssg` produce le route + uno `StaticRouteGenerator`; ARTICLE-PAGE è una `StaticRoute` con `prerender_params` alimentato da `ContentSource::list_published()` (uno slug per articolo pubblicato); LISTING-PAGE/HOME-PAGE sono `StaticRoute` senza parametri. `StaticRouteGenerator::generate(&leptos_options)` scrive l'HTML in `site-root` **una volta, a build time**; in produzione il processo termina dopo la generazione (nessun listener always-on, coerente con ADR-004) — verificato leggendo il sorgente di `leptos_actix` 0.8.7 (`write_static_route` scrive file `.html` reali) e l'esempio ufficiale `examples/static_routing` di leptos-rs/leptos. In dev (`cargo leptos watch`) lo stesso binario può continuare a tenere un listener attivo per comodità di sviluppo — non contraddice ADR-004, che vincola solo la produzione | Verificato da fonte primaria (sorgente crate + esempio ufficiale) prima di fissare la decisione, non assunto; evita di scoprire a metà implementazione che il meccanismo richiede un server sempre attivo anche in produzione | Scrivere un renderer HTML custom senza passare da `leptos_router`/`leptos_actix` (rendering manuale di ogni `Article` a stringa) — scartata: reinventerebbe la gestione di route/param/hydration già offerta dal framework, aumentando complessità accidentale |
| 2026-08-24 | Guardrail: nessuna island può invocare una `#[server]` function a runtime — in produzione non esiste alcun server a rispondere dopo la generazione statica. I dati devono essere risolti interamente in fase di `generate()` e serializzati nell'HTML prerenderizzato. Le island esistenti (`DynamicHeader`, `LightDarkSwitch`) già rispettano questo vincolo (nessuna server function); vincolo esplicito per `/hexagonal-architecture` e per island future | Se non esplicitato ora, una futura island che chiama una server function funzionerebbe in dev (`cargo leptos watch`, server attivo) ma fallirebbe silenziosamente in produzione (nessun server) — rischio di regressione difficile da notare in un ambiente di sviluppo che nasconde il problema | — |

INVEST: I✓ N✓ V✓ E✓ S✓ T✓  |  1 Decision: ✓  |  Coherence: ✓
