## EP-001-UC-001-S006 — Servire il sito sotto una `base_url` configurabile

> Creata nel replan del 2026-10-26. **Ordine di consegna: 1° di 7** (**S006** → S005 → S007 → S008 → S004 → S009 → S010).

**Decision:** Come ogni percorso prodotto dal sito (route, CSS, wasm/js in `/pkg`, immagini, asset, link interni) deriva da una `base_url` sempre definita e configurabile, invece di percorsi assoluti dalla radice. È un principio permanente, non un adattamento temporaneo all'indirizzo di GitHub Pages: il passaggio al dominio personale diventa un cambio di configurazione. Decisione trasversale → richiede un ADR.

As Visitatore del sito, voglio che stili, immagini, link e interattività funzionino a qualunque indirizzo il sito sia pubblicato, so that vedo sempre un sito completo e navigabile.

| Field | Value |
|---|---|
| **Epic** | EP-001 — Rilancio del blog personale come presenza professionale |
| **UC** | UC-001 — Autore pubblica un nuovo articolo |
| **Pattern** | Major effort isolation |
| **AT rows** | AT-UC-001 righe: 1-5 (success guarantee "articolo raggiungibile pubblicamente"); epic AC-1 |

### Acceptance criteria
- Given `base_url` = `/lucalorenzon-blog/`, when il sito generato è servito sotto quel prefisso, then CSS, wasm/js, immagini, asset e link interni risolvono tutti, senza 404
- Given `base_url` = `/` (dominio personale), when il sito generato è servito alla radice, then vale lo stesso
- Given il sito servito sotto un prefisso, when il visitatore usa le islands (cambio tema, header), then l'hydration funziona
- Given lo sviluppo locale (`cargo leptos watch`), when il sito è servito in locale, then continua a funzionare

### Design pipeline
Before any implementation, complete in order:
- [ ] `/adr`                    — `base_url` configurabile come principio trasversale
- [ ] `/software-design`        — coupling, ownership, accidental complexity
- [ ] `/hexagonal-architecture` — ports, adapters, composition root
- [ ] `/parse-dont-validate`    — domain types and invariants
- [ ] `/sw-practices`           — naming, error handling, bootstrap

### Next steps after agreement
- [ ] `/acceptance-tests EP-001-UC-001-S006` — story-level AT table (narrower than UC level)
- [ ] `/story-size EP-001-UC-001-S006`       — assign XS / S / M / L / XL / XXL

### Open questions
- Come Leptos 0.8 / cargo-leptos applicano un prefisso agli script di hydration e a `/pkg`: da verificare, eventualmente con uno `/spike` breve.
- "Sempre definita": `base_url` obbligatoria (build rifiutata se manca) o con un valore di default?
- `base_url` è solo il percorso o include l'origine (`https://…`)? L'origine servirà più avanti a canonical/sitemap, oggi fuori scope.
- Probabile bug esistente: `src/components/logos/logo.rs` usa `/assets/logoIcon.png`, ma gli asset sono serviti dalla radice (commit `30a1e27`).

### Dependencies
- Nessuna: è la prima storia dell'ordine di consegna.

### Decisions Log
| Date | Decision | Reasoning | Alternatives Considered |
|---|---|---|---|

INVEST: I✓ N✓ V✓ E✓ S✓ T✓  |  1 Decision: ✓  |  Coherence: ✓
