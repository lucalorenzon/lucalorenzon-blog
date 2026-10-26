## EP-001-UC-001-S012 — Pubblicare pagine di esempio dal repo contenuti sotto `CONTEXT_ROOT`

> Creata il 2026-10-26 dallo split per decisione di [[EP-001-UC-001-S006]] (pezzo D3). **Ordine di consegna: 4° di 10** (S006 → S005 → S011 → **S012** → S013 → S007 → S008 → S004 → S009 → S010).

**Decision:** Come i percorsi derivati dai contenuti (pagine di articoli ed elenco, link agli articoli, immagini) derivano da `CONTEXT_ROOT`, e come la CI legge il repo privato dei contenuti (credenziale: PAT fine-grained o deploy key, ADR-003). **Due decisioni tenute insieme per scelta dell'utente**: hanno valore solo insieme — senza credenziale nessun contenuto va online, senza percorsi corretti il contenuto online è rotto. Chiude la sospensione della generazione dei contenuti introdotta da [[EP-001-UC-001-S006]]: la homepage con l'elenco degli articoli (S003) torna al posto della pagina "in costruzione".

As Visitatore del sito, voglio sfogliare l'elenco e aprire alcune pagine di esempio con link e immagini funzionanti all'indirizzo pubblico del sito, so that vedo il sito come apparirà con gli articoli veri.

| Field | Value |
|---|---|
| **Epic** | EP-001 — Rilancio del blog personale come presenza professionale |
| **UC** | UC-001 — Autore pubblica un nuovo articolo |
| **Pattern** | Major effort isolation (split per decisione di S006) |
| **AT rows** | AT-UC-001 righe: 1-5 (success guarantee "articolo raggiungibile pubblicamente"); epic AC-1, AC-2 |

### Acceptance criteria
- Given il repo contenuti con alcuni articoli di esempio "Lorem ipsum" su `main`, when la pipeline su push di codice gira, then la CI legge il repo privato dei contenuti con la credenziale configurata e genera articoli, elenco e homepage
- Given `CONTEXT_ROOT` = `/lucalorenzon-blog/` o `/`, when si genera il sito, then link agli articoli, immagini proprie e immagine di fallback includono la `CONTEXT_ROOT`
- Given il sito servito sotto un prefisso, when il visitatore apre homepage, elenco e un articolo di esempio, then ogni pagina risponde 200 e nessuna risorsa risponde 404
- Given la generazione dei contenuti riattivata, when la si apre, then la homepage mostra l'elenco degli articoli al posto della pagina "in costruzione"
- Given lo sviluppo locale con `CONTEXT_ROOT` = `/`, when si apre un articolo di esempio, then si carica senza 404

Acceptance tests: [AT-EP-001-UC-001-S012](../acceptance-tests/AT-EP-001-UC-001-S012-pubblicare-pagine-esempio-repo-contenuti.md) — righe spostate da AT-S006 (già approvate), da completare in `/acceptance-tests` (lettura in CI, homepage con elenco).

### Design pipeline
Before any implementation, complete in order:
- [ ] `/software-design`        — coupling, ownership, accidental complexity
- [ ] `/hexagonal-architecture` — ports, adapters, composition root
- [ ] `/parse-dont-validate`    — domain types and invariants
- [ ] `/sw-practices`           — naming, error handling, bootstrap

### Next steps after agreement
- [ ] `/acceptance-tests EP-001-UC-001-S012` — story-level AT table (narrower than UC level)
- [ ] `/story-size EP-001-UC-001-S012`       — assign XS / S / M / L / XL / XXL

### Open questions
- Credenziale di lettura del repo contenuti: PAT fine-grained o deploy key (ADR-003, Technical Notes; spostata qui da [[EP-001-UC-001-S005]]).
- Gli articoli "Lorem ipsum" non devono contenere link interni nel markdown: la loro riscrittura arriva con [[EP-001-UC-001-S013]].
- Gli articoli di esempio restano pubblici fino all'arrivo degli articoli veri: quando e come rimuoverli non è deciso.

### Dependencies
- [[EP-001-UC-001-S006]]: `CONTEXT_ROOT` e la sospensione dei contenuti che questa storia chiude.
- [[EP-001-UC-001-S005]] (deploy su push di codice): da qui il deploy torna a includere i contenuti attuali del repo contenuti.
- Il trigger da push sul repo contenuti, con la sua seconda credenziale, resta a [[EP-001-UC-001-S004]] e riusa la lettura configurata qui.

### Decisions Log
| Date | Decision | Reasoning | Alternatives Considered |
|---|---|---|---|

INVEST: I✓ N✓ V✓ E✓ S✓ T✓  |  1 Decision: ✗ (due, tenute insieme per scelta dell'utente: valore solo insieme)  |  Coherence: ✓
