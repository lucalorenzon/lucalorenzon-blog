## EP-001-UC-001-S009 — Controllare l'output generato prima del deploy

> Creata nel replan del 2026-10-26. **Ordine di consegna: 9° di 10** (S006 → S005 → S011 → S012 → S013 → S007 → S008 → S004 → **S009** → S010).

**Decision:** Cosa si controlla sull'output generato prima del deploy, con quale soglia, e cosa blocca e cosa avvisa. Concordato al momento dello split: un link interno, un'immagine o un CSS non raggiungibili, o una risorsa oltre la dimensione massima X configurabile, **bloccano** il deploy. Un link esterno rotto produce **solo un avviso**. Il controllo vale per entrambe le pipeline, su push di codice e su push di contenuto.

As Visitatore del sito, voglio che ogni link, immagine e stile del sito funzioni e si carichi rapidamente, so that non trovo mai pagine rotte o lente.

| Field | Value |
|---|---|
| **Epic** | EP-001 — Rilancio del blog personale come presenza professionale |
| **UC** | UC-001 — Autore pubblica un nuovo articolo |
| **Pattern** | Business rule variations |
| **AT rows** | Nessuna riga AT-UC-001 esistente: **estensione mancante in UC-001** (output generato non valido), da aggiungere a UC-001 e AT-UC-001 prima dell'implementazione; epic AC-3 |

### Acceptance criteria
- Given un output in cui un link interno, un'immagine o un CSS non risolvono, when il controllo gira, then il deploy non parte e il check fallisce nominando pagina e risorsa
- Given una risorsa più grande della soglia X configurata, when il controllo gira, then il deploy non parte e il check fallisce nominando risorsa e dimensione
- Given un link esterno non raggiungibile, when il controllo gira, then viene prodotto un avviso e il deploy procede
- Given un output valido, when il controllo gira, then il deploy procede

### Design pipeline
Before any implementation, complete in order:
- [ ] `/software-design`        — coupling, ownership, accidental complexity
- [ ] `/hexagonal-architecture` — ports, adapters, composition root
- [ ] `/parse-dont-validate`    — domain types and invariants
- [ ] `/sw-practices`           — naming, error handling, bootstrap

### Next steps after agreement
- [ ] `/acceptance-tests EP-001-UC-001-S009` — story-level AT table (narrower than UC level)
- [ ] `/story-size EP-001-UC-001-S009`       — assign XS / S / M / L / XL / XXL

### Open questions
- Come l'avviso sui link esterni arriva davvero all'autore: GitHub manda l'email solo per i check falliti, e un avviso su un check verde si vede solo aprendo il run.
- Valore di default di X, e se la soglia varia per tipo di risorsa (immagine, CSS, wasm). Va coordinata con EP-002 (varianti responsive delle immagini).
- Link esterni: timeout e tentativi, perché un sito di terzi lento non diventi rumore a ogni pubblicazione (residuo #2).

### Dependencies
- Si inserisce nelle pipeline di [[EP-001-UC-001-S005]] e [[EP-001-UC-001-S004]], fra generazione e deploy.

### Decisions Log
| Date | Decision | Reasoning | Alternatives Considered |
|---|---|---|---|

INVEST: I✓ N✓ V✓ E✓ S✓ T✓  |  1 Decision: ✓  |  Coherence: ✓
