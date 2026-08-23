## EP-001-UC-001-S002 — Verificare l'unicità dello slug rispetto agli articoli già pubblicati

**Decision:** Come e dove il sistema verifica che lo slug del nuovo articolo non sia già usato da un articolo esistente, tramite un nuovo metodo `ContentSource::exists` (presence-check, non `get_by_slug`/`list_published` — vedi Decisions Log 2026-08-24).

As Autore/Editore, voglio essere bloccato se scelgo uno slug già usato so that non sovrascrivo o creo ambiguità con un articolo già pubblicato.

| Field | Value |
|---|---|
| **Epic** | EP-001 — Rilancio del blog personale come presenza professionale |
| **UC** | UC-001 — Autore pubblica un nuovo articolo |
| **Pattern** | Business rule variations |
| **AT rows** | AT-UC-001 righe: 3b (slug duplicato) |

### Acceptance criteria
- Given un frontespizio altrimenti valido con uno slug uguale a quello di un articolo già pubblicato, when il sistema verifica l'unicità tramite `ContentSource`, then la pubblicazione non procede e l'errore identifica il conflitto di slug
- Given un frontespizio altrimenti valido con uno slug non ancora usato, when il sistema verifica l'unicità tramite `ContentSource`, then la verifica passa e non blocca la pubblicazione

### Design pipeline
Before any implementation, complete in order:
- [x] `/software-design`        — coupling, ownership, accidental complexity
- [x] `/hexagonal-architecture` — ports, adapters, composition root
- [ ] `/parse-dont-validate`    — domain types and invariants
- [ ] `/sw-practices`           — naming, error handling, bootstrap

Hexagonal design artefact: [docs/architecture/hexagonal.md](../architecture/hexagonal.md)

### Next steps after agreement
- [ ] `/acceptance-tests EP-001-UC-001-S002` — story-level AT table (narrower than UC level)
- [ ] `/story-size EP-001-UC-001-S002`       — assign XS / S / M / L / XL / XXL

### Open questions
- AT-UC-001 nota esplicitamente che UC-001 non specifica se il controllo di unicità avviene in uno step CI dedicato o durante il build stesso — questa story decide solo la logica di dominio/porta; il punto esatto nella pipeline è materia di [[EP-001-UC-001-S004]].

### Dependencies
- Richiede il tipo `Article`, `Slug` e la porta `ContentSource` di [[EP-001-UC-001-S001]], già implementati e disponibili (external design dependency — Independence sub-check b). Questa story estende la porta con un nuovo metodo `exists` (vedi Decisions Log 2026-08-24) — non ridisegna `get_by_slug`/`list_published`. Nessuna dipendenza da `ContentSource::list_published`, che resta `NotImplemented`, di competenza di [[EP-001-UC-001-S003]] (LISTING-PAGE).

### Decisions Log
| Date | Decision | Reasoning | Alternatives Considered |
|---|---|---|---|
| 2026-08-24 | Prima ipotesi: il controllo di unicità slug usa `ContentSource::get_by_slug(candidate)`, non `list_published`. Mapping: `FetchError::NotFound` → slug libero; `Ok(article)` → conflitto; `FetchError::Malformed` → conflitto comunque (il nome file `{slug}.md` è già occupato, indipendentemente dalla validità del suo contenuto — evita una collisione silenziosa sul filesystem); `FetchError::Io` → propagato come errore infrastrutturale, non tradotto in unique/conflict | Interrogare un singolo slug è l'informazione minima necessaria per l'AT-3b (esiste/non esiste); `list_published` richiederebbe di leggere e parsare *tutti* gli articoli pubblicati solo per verificarne uno, con relativa complessità accidentale (duplicazione parsing tra i due metodi dell'adapter, gestione di un eventuale articolo già pubblicato ma malformato in mezzo alla lista) | Usare `list_published` e confrontare il candidato contro l'elenco intero — scartata dopo discussione, complessità non necessaria per questo controllo; trattare `Malformed` come "slug libero" — scartata, permetterebbe una collisione silenziosa di nome file |
| 2026-08-24 | Rivista in `/hexagonal-architecture`: aggiunto un terzo metodo `exists(&self, slug: &Slug) -> Result<bool, FetchError>` a `ContentSource`, invece di riusare `get_by_slug`. `exists` fa solo un presence-check sul path `{slug}.md` (nessuna lettura, nessun parsing YAML, nessuna `Article::new`) — più leggero di `get_by_slug` e, soprattutto, elimina l'ambiguità della riga precedente su `Malformed`: un file presente occupa lo slug indipendentemente dalla validità del contenuto, quindi non serve più un caso speciale per distinguerlo. Nuovo dominio: `SlugUniquenessError` (`AlreadyExists` \| `CheckFailed(FetchError)`) e la funzione di dominio `ensure_slug_is_unique(source: &impl ContentSource, candidate: &Slug)` che chiama `exists` e mappa il risultato | Il terzo metodo non riapre la segregazione in più trait scartata in S001 (resta un solo `ContentSource`), quindi non è la complessità accidentale che quella decisione escludeva — è un'estensione dello stesso concetto "chiedi alla porta qualcosa su uno slug". Per un blog personale il costo del parsing scartato da `get_by_slug` sarebbe comunque trascurabile; il motivo reale non è la performance ma la pulizia semantica (chiedere esattamente l'informazione che serve, azzerando un caso limite) | Restare su `get_by_slug` accettando il parsing scartato — scartata dopo discussione: non elimina l'ambiguità su `Malformed` e fa più lavoro del necessario per rispondere a una domanda booleana |

INVEST: I✓ N✓ V✓ E✓ S✓ T✓  |  1 Decision: ✓  |  Coherence: ✓
