## EP-001-UC-001-S002 — Verificare l'unicità dello slug rispetto agli articoli già pubblicati

**Decision:** Come e dove il sistema verifica che lo slug del nuovo articolo non sia già usato da un articolo esistente, interrogando `ContentSource` per l'elenco degli articoli già pubblicati.

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
- [ ] `/software-design`        — coupling, ownership, accidental complexity
- [ ] `/hexagonal-architecture` — ports, adapters, composition root
- [ ] `/parse-dont-validate`    — domain types and invariants
- [ ] `/sw-practices`           — naming, error handling, bootstrap

### Next steps after agreement
- [ ] `/acceptance-tests EP-001-UC-001-S002` — story-level AT table (narrower than UC level)
- [ ] `/story-size EP-001-UC-001-S002`       — assign XS / S / M / L / XL / XXL

### Open questions
- AT-UC-001 nota esplicitamente che UC-001 non specifica se il controllo di unicità avviene in uno step CI dedicato o durante il build stesso — questa story decide solo la logica di dominio/porta; il punto esatto nella pipeline è materia di [[EP-001-UC-001-S004]].

### Dependencies
- Richiede il tipo `Article` e la porta `ContentSource` di [[EP-001-UC-001-S001]] già disponibili (external design dependency — Independence sub-check b).

### Decisions Log
| Date | Decision | Reasoning | Alternatives Considered |
|---|---|---|---|

INVEST: I✓ N✓ V✓ E✓ S✓ T✓  |  1 Decision: ✓  |  Coherence: ✓
