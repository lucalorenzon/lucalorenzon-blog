## EP-001-UC-001-S013 — Riscrivere secondo `CONTEXT_ROOT` i link interni scritti dall'autore nel markdown

> Creata il 2026-10-26 dallo split per decisione di [[EP-001-UC-001-S006]]. **Ordine di consegna: 5° di 10** (S006 → S005 → S011 → S012 → **S013** → S007 → S008 → S004 → S009 → S010) — subito dopo le pagine di esempio, prima della validazione dei contenuti: dalla rigenerazione su push del repo contenuti ([[EP-001-UC-001-S004]]) l'autore pubblica da solo, e un link interno non riscritto andrebbe online rotto.

**Decision:** Come la conversione markdown→HTML aggiunge la `CONTEXT_ROOT` ai link e alle immagini scritti dall'autore con percorso dalla radice (opzione A, concordata in [[EP-001-UC-001-S006]]), lasciando invariati link esterni, ancore, `mailto:` e link relativi. Ripiego su un segnaposto `$CONTEXT_ROOT/…` scritto dall'autore (opzione B) solo se A risulta complicata in design.

As Visitatore del sito, voglio che i link e le immagini interni dentro un articolo funzionino a qualunque indirizzo il sito sia pubblicato, so that posso seguire i rimandi tra articoli senza trovare pagine rotte.

| Field | Value |
|---|---|
| **Epic** | EP-001 — Rilancio del blog personale come presenza professionale |
| **UC** | UC-001 — Autore pubblica un nuovo articolo |
| **Pattern** | Business rule variations (split per decisione di S006) |
| **AT rows** | AT-UC-001 righe: 1-5 (success guarantee "articolo raggiungibile pubblicamente"); epic AC-1 |

### Acceptance criteria
- Given un articolo il cui markdown contiene un link o un'immagine con percorso dalla radice (`/articles/altro`, `/images/x.webp`), when il sito è generato con un prefisso, then nell'HTML il percorso include la `CONTEXT_ROOT`
- Given lo stesso articolo, when il sito è generato con `CONTEXT_ROOT` = `/`, then il percorso resta `/articles/altro`
- Given link esterni (`https://…`, `//…`), ancore (`#…`), `mailto:` o link relativi, when il sito è generato, then restano invariati
- Given HTML grezzo nel markdown (`<a href="/…">`), when il sito è generato, then non viene riscritto (limite accettato)

Acceptance tests: [AT-EP-001-UC-001-S013](../acceptance-tests/AT-EP-001-UC-001-S013-riscrivere-link-markdown-context-root.md) — righe spostate da AT-S006 (già approvate).

### Design pipeline
Before any implementation, complete in order:
- [ ] `/software-design`        — coupling, ownership, accidental complexity
- [ ] `/hexagonal-architecture` — ports, adapters, composition root
- [ ] `/parse-dont-validate`    — domain types and invariants
- [ ] `/sw-practices`           — naming, error handling, bootstrap

### Next steps after agreement
- [ ] `/acceptance-tests EP-001-UC-001-S013` — revisione delle righe spostate
- [ ] `/story-size EP-001-UC-001-S013`       — assign XS / S / M / L / XL / XXL

### Open questions
- Link relativi (`[rel](altro)`): restano invariati per regola, ma il browser li risolve rispetto all'URL della pagina. Se vietarli o segnalarli (validazione dei contenuti, [[EP-001-UC-001-S007]], o controllo dell'output, [[EP-001-UC-001-S009]]) è da decidere fuori da questa storia.

### Dependencies
- [[EP-001-UC-001-S006]] (`CONTEXT_ROOT`), [[EP-001-UC-001-S012]] (contenuti online).
- Fattibilità di A già verificata: `pulldown-cmark` 0.13 (già usato in `src/pages/view_model.rs`) espone `Tag::Link`/`Tag::Image` con `dest_url`.

### Decisions Log
| Date | Decision | Reasoning | Alternatives Considered |
|---|---|---|---|

INVEST: I✓ N✓ V✓ E✓ S✓ T✓  |  1 Decision: ✓  |  Coherence: ✓
