## EP-001-UC-001-S011 — Reintrodurre le islands (menu e header dinamico) sotto `CONTEXT_ROOT`

> Creata il 2026-10-26 dallo split per decisione di [[EP-001-UC-001-S006]] (pezzo D2). **Ordine di consegna: 3° di 10** (S006 → S005 → **S011** → S012 → S013 → S007 → S008 → S004 → S009 → S010).

**Decision:** Come gli script di hydration (js e wasm in `pkg/`) derivano da `CONTEXT_ROOT` e come le islands tornano attive sotto prefisso — menu con selettore tema e header che si adatta allo scroll — chiudendo la sospensione introdotta da [[EP-001-UC-001-S006]] (e la relativa nota in ADR-004).

As Visitatore del sito, voglio aprire il menu, cambiare tema e vedere l'header adattarsi allo scroll anche quando il sito è servito sotto un prefisso, so that ritrovo l'interattività del sito ovunque sia pubblicato.

| Field | Value |
|---|---|
| **Epic** | EP-001 — Rilancio del blog personale come presenza professionale |
| **UC** | UC-001 — Autore pubblica un nuovo articolo (dallo split di S006); comportamento delle islands da [UC-004](../use-cases/UC-004-header-adattivo-scroll.md) e [UC-005](../use-cases/UC-005-visitatore-usa-menu-navigazione.md) |
| **Pattern** | Major effort isolation (split per decisione di S006) |
| **AT rows** | AT-UC-001 righe: 1-5 (success guarantee, limitatamente all'interattività); success guarantee di UC-004 e UC-005; epic AC-1 |

### Acceptance criteria
- Given `CONTEXT_ROOT` = `/lucalorenzon-blog/` o `/`, when si genera il sito, then gli `href` di script di hydration (`modulepreload`) e modulo wasm (`preload`) includono la `CONTEXT_ROOT`
- Given il sito servito sotto un prefisso, when il visitatore apre la homepage, then nessuna richiesta di js o wasm risponde 404
- Given il sito servito sotto un prefisso, when il visitatore apre il menu e clicca il selettore tema, then il tema cambia (classe `dark` su `<html>` e testo del selettore alternati a ogni clic)
- Given il sito servito sotto un prefisso, when il visitatore scorre oltre la cima della pagina, then l'header cambia aspetto (classe `bg-white`)
- Given lo sviluppo locale con `CONTEXT_ROOT` = `/`, when il sito è servito in locale, then le islands funzionano come sotto prefisso

Acceptance tests: [AT-EP-001-UC-001-S011](../acceptance-tests/AT-EP-001-UC-001-S011-reintrodurre-islands-context-root.md) — righe spostate da AT-S006 (già approvate), da completare in `/acceptance-tests` (menu presente, sviluppo locale).

### Design pipeline
Before any implementation, complete in order:
- [ ] `/software-design`        — coupling, ownership, accidental complexity
- [ ] `/hexagonal-architecture` — ports, adapters, composition root
- [ ] `/parse-dont-validate`    — domain types and invariants
- [ ] `/sw-practices`           — naming, error handling, bootstrap

### Next steps after agreement
- [ ] `/acceptance-tests EP-001-UC-001-S011` — story-level AT table (narrower than UC level)
- [ ] `/story-size EP-001-UC-001-S011`       — assign XS / S / M / L / XL / XXL

### Open questions
- L'islands router (`islands_router=true`) usa gli `href` così come sono: il controllo sul `basePath` è commentato in `islands_routing.js`. Da verificare che la navigazione client-side non esca dal prefisso.
- Le righe nel browser (menu, tema, header) richiedono un host che gestisca il prefisso e un browser reale: manuali finché non c'è l'infrastruttura Playwright di [[EP-001-S010]], salvo diversa scelta di design in [[EP-001-UC-001-S006]].

### Dependencies
- [[EP-001-UC-001-S006]]: `CONTEXT_ROOT` e la sospensione delle islands che questa storia chiude.
- [[EP-001-UC-001-S005]] (deploy su push di codice): per vedere il risultato online.

### Decisions Log
| Date | Decision | Reasoning | Alternatives Considered |
|---|---|---|---|

INVEST: I✓ N✓ V✓ E✓ S✓ T✓  |  1 Decision: ✓  |  Coherence: ✓
