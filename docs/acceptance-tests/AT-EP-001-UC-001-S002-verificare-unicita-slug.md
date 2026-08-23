# AT-EP-001-UC-001-S002: Verificare l'unicità dello slug rispetto agli articoli già pubblicati

Epic: EP-001 | UC: UC-001 | Story: EP-001-UC-001-S002

---

## Component: `ContentSource::exists` — presence check (adapter-level)

> Source: Story AC-1, AC-2 (implementazione della verifica)

| slug candidato | documento presente nel content source? | `exists()`? | ref |
|---|---|---|---|
| slug con documento già pubblicato | sì, ben formato | `Ok(true)` | AC-1 |
| slug con documento presente ma malformato (contenuto non valido come `Article`) | sì, malformato | `Ok(true)` (presenza, non validità, è ciò che conta) | AC-1 |
| slug senza alcun documento | no | `Ok(false)` | AC-2 |

---

## Component: `ensure_slug_is_unique` — esito della verifica

> Source: Story AC-1, AC-2

| `exists(candidate)`? | esito? | errore identifica lo slug in conflitto? | ref |
|---|---|---|---|
| `Ok(true)` | `Err(SlugUniquenessError::AlreadyExists { slug: candidate })` | `true` | AC-1 |
| `Ok(false)` | `Ok(())` | — | AC-2 |

---

## Open Issues

- **Fallimento infrastrutturale di `exists` (`FetchError::Io`) non coperto da nessuna riga AC/AT**: nessun acceptance criterion di questa story richiede un comportamento specifico su errore I/O (permessi, disco) — solo che il dominio non lo traduca in "slug libero" (già garantito dal match esaustivo di `ensure_slug_is_unique`, vedi `docs/design/slug-uniqueness.md`). Stesso trattamento già accettato per `FetchError::Io` in AT-EP-001-UC-001-S001 (dipendente da OS, giudicato non necessario come test automatico dedicato).
- **Punto esatto della pipeline in cui `ensure_slug_is_unique` viene invocato**: non deciso da questa story (vedi Open Questions della story) — materia di [EP-001-UC-001-S004](../stories/EP-001-UC-001-S004-automatizzare-pipeline-ci-build.md).

---

## Traceability

Ogni riga di queste tabelle traccia a un acceptance criterion di [EP-001-UC-001-S002](../stories/EP-001-UC-001-S002-verificare-unicita-slug.md) ed è un sottoinsieme più fine della riga 3b di [AT-UC-001](AT-UC-001-autore-pubblica-articolo.md), limitato alla logica di dominio/porta (l'esito osservabile a livello di pipeline CI resta fuori scope, vedi [[EP-001-UC-001-S004]]).
