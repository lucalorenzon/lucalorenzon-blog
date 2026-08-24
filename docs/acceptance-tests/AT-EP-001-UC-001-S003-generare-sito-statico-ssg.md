# AT-EP-001-UC-001-S003: Generare il sito statico da un Article (ARTICLE-PAGE, LISTING-PAGE, HOME-PAGE)

Epic: EP-001 | UC: UC-001 | Story: EP-001-UC-001-S003

Scope: sottoinsieme più fine delle righe Happy path 1-5 e "abstract/immagine
assente" di [AT-UC-001](AT-UC-001-autore-pubblica-articolo.md), limitato al
meccanismo di generazione statica (da un insieme di `Article` di dominio a
output HTML) — il deploy resta fuori scope ([[EP-001-UC-001-S005]]), così
come il trigger CI ([[EP-001-UC-001-S004]]).

---

## Component: `ContentSource::list_published` — elenco articoli per la build

> Source: Story AC-2, AC-3 (LISTING-PAGE/HOME-PAGE hanno bisogno dell'intero elenco)

| stato del content source | `list_published()`? | ref |
|---|---|---|
| nessun articolo pubblicato | `Ok([])` | AC-2 |
| N articoli pubblicati, tutti ben formati | `Ok(<N Article>)`, ordine non garantito dal contratto della porta | AC-2 |
| N articoli pubblicati, uno malformato | `Err(FetchError::Malformed(_))` — propagato, la build si interrompe piuttosto che pubblicare un elenco parziale silenzioso | AC-2 |

---

## Component: generazione ARTICLE-PAGE

> Source: Story AC-1

| `Article` di dominio | ARTICLE-PAGE(slug)? | ref |
|---|---|---|
| valido, con abstract e immagine presenti | `{ reachable: true, metadata: { date, slug, tags, title, abstract: <abstract dell'Article>, image: <image dell'Article> }, content: <body renderizzato come HTML> }` | AC-1 |

---

## Component: generazione LISTING-PAGE / HOME-PAGE

> Source: Story AC-2, AC-3

| articoli pubblicati | LISTING-PAGE? | HOME-PAGE? | ref |
|---|---|---|---|
| 1 articolo pubblicato | include una voce per il suo slug | uguale a LISTING-PAGE (caso particolare, 1 solo articolo) | AC-2, AC-3 |
| 3 articoli pubblicati con date diverse | include una voce per ciascuno slug | voci in ordine cronologico decrescente (più recente prima) | AC-2, AC-3 |

---

## Component: `effective_abstract` — fallback quando l'abstract è assente

> Source: Story AC-4

| `Article.abstract_text` | `Article.body` | `effective_abstract()`? | ref |
|---|---|---|---|
| presente (`Some(Abstract)`) | qualsiasi | il testo dell'abstract, invariato | AC-4 |
| assente (`None`) | `"Some content."` | `?UNKNOWN?` — troncamento del body a una lunghezza non ancora decisa (vedi Open Issues) | AC-4 |
| assente (`None`) | body più corto della soglia di troncamento | `?UNKNOWN?` — il body intero, o troncato comunque? Stessa decisione pendente | AC-4 |

---

## Component: `effective_image` — fallback quando l'immagine è assente

> Source: Story AC-5

| `Article.image` | `effective_image()`? | ref |
|---|---|---|
| presente (`Some(ImagePath)`) | il path dell'immagine, invariato | AC-5 |
| assente (`None`) | `?UNKNOWN?` — asset di fallback non ancora scelto da Luca (vedi Open Issues e la story's Open questions) | AC-5 |

---

## Open Issues

- **Lunghezza di troncamento per `effective_abstract`**: non decisa. Nessuna riga di questa tabella con abstract assente può avere un valore atteso esatto finché non è fissata. Bloccante solo per l'implementazione di `effective_abstract`, non per il resto della story (value object `Body`/`Abstract`/`ImagePath`, `list_published`, già implementati e testati indipendentemente da questa decisione).
- **Asset di fallback per `effective_image`**: non deciso (stessa nota nella story, Open questions) — `assets/images/ostia_sea_top_image.webp` esiste ma è oggi lo sfondo del layout, non un placeholder per articoli; riusarlo richiede conferma esplicita di Luca, non un default silenzioso.
- **Rendering markdown → HTML esatto** (contenuto di ARTICLE-PAGE): la riga "generazione ARTICLE-PAGE" assume che `pulldown-cmark` (deciso in `/sw-practices`, vedi `docs/design/ssg-page-generation.md`) produca HTML a partire dal body — il valore atteso esatto per un body di test specifico non è fissato qui (dipende dalla resa CommonMark esatta), lasciato ai test di implementazione (`/test`) come confronto sull'HTML effettivamente prodotto dalla libreria, non come valore inventato in questa tabella.
- **Ordine di `list_published`**: la tabella conferma esplicitamente che la porta non garantisce un ordine (deciso in `/hexagonal-architecture`) — l'ordinamento cronologico per HOME-PAGE è testato separatamente nella tabella "generazione LISTING-PAGE / HOME-PAGE", a livello di composition root, non di `list_published` stesso.

---

## Traceability

Ogni riga traccia a un acceptance criterion di
[EP-001-UC-001-S003](../stories/EP-001-UC-001-S003-generare-sito-statico-ssg.md)
ed è un sottoinsieme più fine delle righe Happy path di
[AT-UC-001](AT-UC-001-autore-pubblica-articolo.md), limitato al meccanismo di
generazione statica — deploy e trigger CI restano fuori scope.
