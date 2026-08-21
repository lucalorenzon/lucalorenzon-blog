# AT-EP-001-UC-001-S001: Modellare l'articolo e la porta ContentSource

Epic: EP-001 | UC: UC-001 | Story: EP-001-UC-001-S001

---

## Component: Article — costruzione da frontespizio (happy path)

> Source: Story AC-1

| data | slug | tag | titolo | abstract | immagine di sintesi | costruzione riuscita? | Article.abstract? | Article.immagine? | ref |
|---|---|---|---|---|---|---|---|---|---|
| valida | valido | valida (≥1 tag) | valido | presente e ben formato | presente e ben formata | `true` | `Some(<abstract>)` | `Some(<immagine>)` | AC-1 |
| valida | valido | valida (≥1 tag) | valido | assente | presente e ben formata | `true` | `None` | `Some(<immagine>)` | AC-1 |
| valida | valido | valida (≥1 tag) | valido | presente e ben formato | assente | `true` | `Some(<abstract>)` | `None` | AC-1 |

---

## Component: Article — costruzione rifiutata (campo obbligatorio assente)

> Source: Story AC-2, AC-3, AC-4, AC-5

| data | slug | tag | titolo | costruzione riuscita? | campo causa? | ref |
|---|---|---|---|---|---|---|
| assente | valido | valida (≥1 tag) | valido | `false` | `Data` | AC-2 |
| valida | assente | valida (≥1 tag) | valido | `false` | `Slug` | AC-3 |
| valida | valido | assente (lista vuota) | valido | `false` | `Tag` | AC-4 |
| valida | valido | valida (≥1 tag) | assente | `false` | `Titolo` | AC-5 |

---

## Component: Article — costruzione rifiutata (campo obbligatorio malformato)

> Source: Story AC-2, AC-3, AC-4, AC-5

| data | slug | tag | titolo | costruzione riuscita? | campo causa? | ref |
|---|---|---|---|---|---|---|
| malformata (`?UNKNOWN?` formato atteso) | valido | valida (≥1 tag) | valido | `false` | `Data` | AC-2 |
| valida | malformato (`?UNKNOWN?` formato atteso) | valida (≥1 tag) | valido | `false` | `Slug` | AC-3 |
| valida | valido | contiene un valore malformato (`?UNKNOWN?` formato atteso) | valido | `false` | `Tag` | AC-4 |
| valida | valido | valida (≥1 tag) | malformato (`?UNKNOWN?` formato atteso) | `false` | `Titolo` | AC-5 |

---

## Component: ContentSource — forma della porta

> Source: Story AC-6

| metodo | dichiarato nell'interfaccia? | implementato in S001? | ref |
|---|---|---|---|
| lettura articolo singolo per slug | `true` | `true` | AC-6 |
| elenco articoli pubblicati | `true` | `false` (implementato in EP-001-UC-001-S002/S003) | AC-6 |

---

## Open Issues

- **Formato "malformato" per data/slug/tag/titolo non ancora deciso** (`?UNKNOWN?` nelle tre righe della tabella "campo obbligatorio malformato"): sintassi esatta attesa per ciascun campo (es. formato data, charset slug, vincoli sul valore di un tag, vincoli sul titolo) è materia di `/parse-dont-validate`, non ancora eseguito per questa story (design pipeline non ancora completata).
- **Tipo/messaggio esatto dell'errore di costruzione non ancora deciso**: le tabelle sopra assertano solo `costruzione riuscita? = false` e il campo causa (nome semantico, es. `Data`/`Slug`/`Tag`/`Titolo`), come richiesto dagli AC. La forma Rust esatta (variante enum, struct, messaggio) è esplicitamente rimandata dalla story al design pipeline (`/parse-dont-validate`) — da colmare prima di `/test`.
- **Fallimenti di lettura da ContentSource (I/O, file non trovato) non coperti**: gli AC di questa story riguardano solo la costruzione del tipo `Article` da un frontespizio e la forma dell'interfaccia della porta, non il comportamento di errore della lettura filesystem stessa — non è chiaro se questa story includa anche quel caso o se sia materia di `/hexagonal-architecture` (adapter filesystem). Da chiarire nel design pipeline, non qui.

---

## Traceability

Ogni riga di queste tabelle traccia a un acceptance criterion di [EP-001-UC-001-S001](../stories/EP-001-UC-001-S001-modellare-articolo-dominio-content-source.md) ed è un sottoinsieme più fine delle righe 1-5 e 3a di [AT-UC-001](AT-UC-001-autore-pubblica-articolo.md), limitato alla costruzione del dominio `Article` (le righe UC-level su abstract/immagine calcolati automaticamente a build time restano fuori scope, vedi [[EP-001-UC-001-S003]]).
