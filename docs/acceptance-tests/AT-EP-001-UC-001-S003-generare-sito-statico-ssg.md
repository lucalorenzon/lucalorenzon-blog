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
| 2 articoli pubblicati con la **stessa data**, slug `b-article` e `a-article` | include una voce per ciascuno slug | `a-article` prima di `b-article` — tie-break deterministico su `Slug` (ordine alfabetico), non l'ordine di iterazione di `list_published` | tie-break, residuality 2026-08-30 |

---

## Component: `effective_abstract` — fallback quando l'abstract è assente

> Source: Story AC-4

| `Article.abstract_text` | `Article.body` | `effective_abstract()`? | ref |
|---|---|---|---|
| presente (`Some(Abstract)`) | qualsiasi | il testo dell'abstract, invariato | AC-4 |
| assente (`None`) | ≤ 200 caratteri | il body per intero, invariato — nessun taglio, nessuna ellissi | AC-4 |
| assente (`None`) | > 200 caratteri | body troncato all'ultimo confine di parola raggiungibile entro 200 caratteri, con "…" finale — valore esatto per un body di test specifico lasciato a `/test` (stessa logica della riga markdown→HTML sotto) | AC-4 |

---

## Component: `resolve_image` — risoluzione dell'immagine (residuality extension, 2026-08-30/31)

> Source: Story AC-5, estesa da `/residuality` a "referenziata ma assente su disco"

| `Article.image` | `ContentSource::image_exists`? | `resolve_image()`? | ref |
|---|---|---|---|
| assente (`None`) | n/a, non invocato | `Ok(Fallback { attempted: None })` | AC-5 |
| presente (`Some(path)`) | `Ok(true)` | `Ok(Own(path))` | AC-5 |
| presente (`Some(path)`) | `Ok(false)` | `Ok(Fallback { attempted: Some(path) })` | residuality 2026-08-30 |
| presente (`Some(path)`) | `Err(Io(_))` | `Err(Io(_))` — propagato, non inghiottito | residuality 2026-08-30 |

---

## Component: `effective_image_path` — mapping a path renderizzabile

> Source: Story AC-5

| `ResolvedImage` | `effective_image_path()`? | ref |
|---|---|---|
| `Own(path)` | il path, invariato | AC-5 |
| `Fallback { .. }` (entrambe le varianti) | `/assets/images/article-image-not-found.svg` (nuovo SVG dedicato, confermato da Luca 2026-08-30 — non `ostia_sea_top_image.webp`) | AC-5 |

---

## Open Issues

- **Rendering markdown → HTML esatto** (contenuto di ARTICLE-PAGE): la riga "generazione ARTICLE-PAGE" assume che `pulldown-cmark` (deciso in `/sw-practices`, vedi `docs/design/ssg-page-generation.md`) produca HTML a partire dal body — il valore atteso esatto per un body di test specifico non è fissato qui (dipende dalla resa CommonMark esatta), lasciato ai test di implementazione (`/test`) come confronto sull'HTML effettivamente prodotto dalla libreria, non come valore inventato in questa tabella.
- **Ordine di `list_published`**: la tabella conferma esplicitamente che la porta non garantisce un ordine (deciso in `/hexagonal-architecture`) — l'ordinamento cronologico per HOME-PAGE è testato separatamente nella tabella "generazione LISTING-PAGE / HOME-PAGE", a livello di composition root, non di `list_published` stesso.

---

## Traceability

Ogni riga traccia a un acceptance criterion di
[EP-001-UC-001-S003](../stories/EP-001-UC-001-S003-generare-sito-statico-ssg.md)
ed è un sottoinsieme più fine delle righe Happy path di
[AT-UC-001](AT-UC-001-autore-pubblica-articolo.md), limitato al meccanismo di
generazione statica — deploy e trigger CI restano fuori scope.
