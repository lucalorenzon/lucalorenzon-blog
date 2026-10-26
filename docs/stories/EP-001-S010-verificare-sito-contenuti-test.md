## EP-001-S010 — Verificare il sito generato con contenuti di test

> Creata nel replan del 2026-10-26. Riguarda il Visitatore, non la pubblicazione: non appartiene a UC-001, quindi è tracciata direttamente sugli AC dell'epic. **Ordine di consegna: 7° di 7** (S006 → S005 → S007 → S008 → S004 → S009 → **S010**).

**Decision:** Cosa verifica la suite end-to-end Playwright sul sito generato a partire da un insieme di contenuti di test, non dal repo contenuti reale: comportamento delle pagine e delle islands, accessibilità WCAG 2.2 AA, SEO di base (AC-6). La storia comprende anche il far passare la suite: meta description e gestione dei `#` nel corpo markdown, che oggi producono un secondo `<h1>`.

As Visitatore del sito, voglio pagine accessibili, con titolo, descrizione e struttura corrette, e un'interfaccia che si comporta come previsto, so that posso leggere e navigare il sito senza ostacoli e trovarlo dai motori di ricerca.

| Field | Value |
|---|---|
| **Epic** | EP-001 — Rilancio del blog personale come presenza professionale |
| **Pattern** | Simple/Complex |
| **AT rows** | Epic AC-4 (WCAG 2.2 AA), AC-6 (title, meta description, un solo h1); AC-3 (Lighthouse) da decidere, vedi Open questions |

### Acceptance criteria
- Given un insieme di contenuti di test versionato in questo repo, when la suite gira, then il sito viene generato da quei contenuti e non dal repo contenuti reale
- Given ogni pagina generata (articolo, elenco, home, 404), when la suite la visita, then ha un `<title>`, una meta description e un solo `<h1>`
- Given ogni pagina generata, when la suite esegue un controllo di accessibilità automatico (es. axe), then non risultano violazioni WCAG 2.2 AA
- Given le islands (cambio tema chiaro/scuro, header adattivo), when il visitatore interagisce, then si comportano come descritto in UC-004 e UC-005
- Given la CI su push e PR, when gira, then esegue la suite; il test `example.spec.ts` del template viene rimosso

### Design pipeline
Before any implementation, complete in order:
- [ ] `/software-design`        — coupling, ownership, accidental complexity
- [ ] `/hexagonal-architecture` — ports, adapters, composition root
- [ ] `/parse-dont-validate`    — domain types and invariants
- [ ] `/sw-practices`           — naming, error handling, bootstrap

### Next steps after agreement
- [ ] `/acceptance-tests EP-001-S010` — story-level AT table
- [ ] `/story-size EP-001-S010`       — assign XS / S / M / L / XL / XXL

### Open questions
- Un `#` nel corpo markdown: rifiutarlo in validazione ([[EP-001-UC-001-S007]]) o trasformare le intestazioni del corpo (`#` → `<h2>`)?
- Fonte della meta description: l'abstract (già calcolato quando manca) o un campo dedicato del frontespizio?
- Lighthouse (AC-3) in questa suite o in un controllo separato?
- **Possibili due decisioni** (verifica + adeguamento del sito): se `/story-size` dà più di L, la storia torna a `/sw-story-split`.

### Dependencies
- Nessuna dipendenza di design. Usa la `base_url` di [[EP-001-UC-001-S006]] per servire il sito sotto test.

### Decisions Log
| Date | Decision | Reasoning | Alternatives Considered |
|---|---|---|---|

INVEST: I✓ N✓ V✓ E✓ S? T✓  |  1 Decision: ? (vedi Open questions)  |  Coherence: ✓
