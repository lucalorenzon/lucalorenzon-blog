# ADR-004: SSG + islands al posto di SSR

- **Date:** 2026-08-24
- **Status:** Proposed
- **Stories:** EP-001-UC-001-S003

## Context

Il sito è oggi generato da un processo server (`actix-web`) che deve
restare sempre attivo per rispondere alle richieste dei visitatori — anche
se il contenuto che serve, un blog personale, non cambia tra una visita e
l'altra e non richiede nulla di dinamico per-visitatore (niente login,
niente dati personalizzati).

Questo è in tensione con due decisioni già prese per rilanciare il sito
come vetrina professionale:

- L'hosting scelto è **GitHub Pages** (ADR-003), che pubblica solo file
  statici e non fa girare un processo server.
- Gli obiettivi dichiarati per il rilancio sono caricamento veloce
  (Lighthouse >90), SEO, e nessun costo/manutenzione di un server sempre
  acceso per un progetto a proprietario singolo.

Serve quindi decidere: il sito continua a essere generato a ogni
richiesta da un server always-on, oppure viene generato una volta in fase
di build e pubblicato come pagine statiche?

## Decision

Il sito passa da **SSR (Server-Side Rendering)** a **SSG (Static Site
Generation)**, mantenendo l'architettura a **islands** già adottata
(ADR-001): la maggior parte della pagina resta HTML statico generato in
build, e solo i componenti esplicitamente interattivi (menu, cambio
tema chiaro/scuro) restano idratati via WASM sul client.

Il processo server always-on (`actix-web` in `main.rs`) viene ritirato e
sostituito da una **composition root di build**: uno step eseguito una
volta per ogni pubblicazione, che legge gli `Article` dal content layer
(EP-001-UC-001-S001/S002) e genera ARTICLE-PAGE, LISTING-PAGE e HOME-PAGE
come file HTML statici, distribuiti poi via GitHub Pages.

La scelta è guidata da coerenza con l'hosting già deciso (ADR-003, niente
server always-on possibile su GitHub Pages) e dal fatto che il contenuto è
identico per ogni visitatore — non c'è nessun bisogno reale di
rendering per-richiesta, solo il costo storico di aver scelto SSR "per
lo sfizio di deployarlo" senza un requisito dietro (vedi
`docs/analysis/architecture-assessment-lucalorenzon-blog.md`).

## Consequences

**Positive:**
- Nessun server da mantenere, aggiornare o pagare: il deploy è la
  pubblicazione di file statici, coerente con ADR-003.
- Superficie di attacco ridotta (nessun processo applicativo esposto in
  produzione, nessuna route dinamica reale — l'unica oggi presente,
  `/api/{tail:.*}`, è già dormiente e priva di server function).
- Caricamento più veloce per il visitatore (HTML pre-generato, nessun
  round-trip di rendering server-side), in linea con l'obiettivo
  Lighthouse >90.
- Reversibilità economica: reintrodurre un server dietro lo stesso
  dominio/presentazione, se SSG risultasse insufficiente in futuro, resta
  un'aggiunta contenuta, non un redesign (dominio e presentazione restano
  gli stessi; cambia solo la composition root).

**Negative / Risks:**
- Ogni nuovo articolo richiede una build+deploy per comparire online, non
  basta più che il processo server legga il contenuto aggiornato al volo.
  Il costo è assorbito dalla pipeline CI già pianificata
  (EP-001-UC-001-S004), non da un intervento manuale ricorrente.
- Il meccanismo di paginazione/ordinamento della LISTING-PAGE va deciso
  come parte della build statica — già segnalato come open issue in S003
  e nell'epic, non introdotto da questa ADR ma reso più concreto da essa.
- Il vincolo storico sulla dimensione del bundle wasm (per le islands
  idratate) resta invariato: questa ADR sposta il meccanismo di
  generazione della pagina, non risolve da sola quel vincolo.

## Alternatives Considered

| Alternative | Why rejected |
|---|---|
| Mantenere SSR always-on (`actix-web`) | In conflitto diretto con l'hosting statico già deciso in ADR-003; richiede un processo sempre attivo e i relativi costi/manutenzione per un contenuto che, di fatto, non cambia per richiesta |
| SSR con cache/reverse proxy davanti ad `actix-web` | Aggiunge complessità infrastrutturale (cache, invalidazione) senza eliminare il bisogno di un processo server sempre attivo, quindi non risolve la tensione con ADR-003 |
| CSR puro (rendering interamente lato client da dati JSON) | Peggiora SEO e first-paint rispetto agli obiettivi dichiarati (Lighthouse >90, SEO-friendly); contraddice l'architettura islands già adottata, che assume HTML server-rendered di default e idratazione solo dove serve interattività |

## Technical Notes

Il meccanismo esatto di generazione statica (quale comando/feature di
`cargo-leptos` guida la build SSG, come vengono enumerate le route da
generare a partire dagli `Article` disponibili, come cambia
`Cargo.toml`/`main.rs`) è materia della design pipeline di
EP-001-UC-001-S003 (`/software-design` → `/hexagonal-architecture` →
`/parse-dont-validate` → `/sw-practices`), non di questa ADR. Questa ADR
fissa la direzione (SSG + islands, niente server always-on); l'albero
cartelle indicativo (`ssg/` come composition root al posto di `main.rs`)
è già abbozzato in
`docs/analysis/architecture-assessment-lucalorenzon-blog.md` come
riferimento, non come implementazione vincolante.

## References

- Stories: EP-001-UC-001-S003
- Related ADRs: ADR-001 (islands architecture, Leptos 0.8 stable), ADR-003
  (GitHub Pages hosting, assume output statico)
- Related: `docs/analysis/architecture-assessment-lucalorenzon-blog.md`
  (Recommended Direction, ADR necessarie #1; Move 3)
- Epic: `docs/epics/EP-001-rilancio-blog-professionale.md` (AC-1: output
  statico deployabile senza processo server sempre attivo)
