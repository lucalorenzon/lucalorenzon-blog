# EP-001: Rilancio del blog personale come presenza professionale

> Riportare il sito a compilare e a pubblicare articoli reali su una base architetturale (SSG + content layer) coerente con il bisogno di farsi conoscere professionalmente, sostituendo il precedente uso del repo come banco di prova tecnologico.

---

## Motivation

Il repo è fermo a circa due anni fa (dipendenze Leptos/wasm-bindgen pinnate a versioni ormai superate) e oggi non compila più. Storicamente è stato ricreato più volte come banco di prova tecnologico — un backend SSR scelto "per lo sfizio" di deployarlo, un tentativo di CMS in Flutter/Dart abbandonato per un'esperienza negativa (Dart trovato verboso e illeggibile, costo infrastrutturale eccessivo rispetto al bisogno) — senza che ci fosse mai una necessità reale dietro.

Quella necessità ora esiste: farsi conoscere professionalmente e valorizzare 30 anni di esperienza nel settore. Il sito deve smettere di essere uno sfizio intermittente e diventare qualcosa che resta vivo, pubblica contenuti reali e regge un pubblico professionale.

Questo epic sostituisce il precedente epic di solo aggiornamento dipendenze — [archiviato: EP-001 aggiornamento Leptos/Rust](../archived/epics/EP-001-aggiornamento-leptos-rust.md) — con uno scope più ampio, emerso da un assessment architetturale reale — vedi [docs/analysis/architecture-assessment-lucalorenzon-blog.md](../analysis/architecture-assessment-lucalorenzon-blog.md).

## Context

Stato attuale (fotografato nell'assessment architetturale collegato): sito Leptos SSR + islands parziali, servito da un processo `actix-web` sempre attivo. Nessun contenuto reale — titolo, abstract e corpo dell'unica pagina sono stringhe segnaposto hardcoded nel codice. Le dipendenze principali sono ferme alla riga Leptos 0.6 e il progetto non compila più sulla toolchain corrente. Il lavoro riparte da un assessment architetturale (`architecture-compass` → `software-design`, 2026-08-18) che ha determinato un mismatch tra l'architettura storica e il bisogno attuale, e ha raccomandato una direzione (Hexagonal Architecture, content layer via porta `ContentSource`, passaggio a SSG) — vedi [docs/analysis/architecture-assessment-lucalorenzon-blog.md](../analysis/architecture-assessment-lucalorenzon-blog.md).

## Business Outcome

- Il sito compila e genera un output statico deployabile, senza processo server sempre attivo
- È possibile pubblicare un nuovo articolo scrivendo un file markdown+frontespizio nel repo contenuti dedicato e pushando, senza modificare codice applicativo
- Il sito generato ottiene punteggio Lighthouse >90 (performance)
- Il layout rispetta WCAG 2.2 AA
- Google Tag Manager è attivo e traccia le visite

## Actors

| Actor | Type | Role in this epic |
|---|---|---|
| Autore/Editore | Primary | Scrive articoli in markdown in un editor (es. Zed, app GitHub) e li pubblica pushando sul repo contenuti dedicato |
| Visitatore del sito | Primary | Legge gli articoli pubblicati, naviga il sito |

## Scope

### In scope
- ~~Aggiornamento delle dipendenze Cargo e del toolchain a versioni correnti mantenute (sblocca la build, oggi rotta)~~ — **fatto** (2026-08-20, lane `chore`, commit `169b30c`; vedi changelog)
- Dominio `Article` (parse-dont-validate sul frontmatter: data, slug, tag) e porta `ContentSource` con adapter filesystem sul repo contenuti dedicato
- Elenco articoli (LISTING-PAGE, con HOME-PAGE come caso particolare in ordine cronologico) oltre alla singola ARTICLE-PAGE — vedi UC-003
- Passaggio da SSR (actix-web sempre attivo) a SSG + islands, con ritiro del server always-on
- Adapter di osservabilità isolato per Google Tag Manager
- ADR "SSG + islands al posto di SSR" e ADR "Content come markdown-in-git via porta ContentSource, nessun CMS per ora"
- Verifica dei requisiti non funzionali: Lighthouse >90, WCAG 2.2 AA sul layout esistente

### Out of scope
- SEO on-page tattico (meta OG/Twitter card, canonical URL, JSON-LD)
- Discoverability (robots.txt, sitemap, feed RSS/Atom)
- Pagine di identità professionale (chi sono, contatti)
- Consenso/cookie-banner prima del caricamento di GTM (vedi Open Issues per il rischio associato)
- Selezione e rielaborazione dei contenuti da pubblicare
- Un CMS più evoluto di markdown+git (resta valutabile in futuro, non deciso qui)

## Constraints

| Type | Constraint |
|---|---|
| Business | Nessuna scadenza esterna rigida, ma priorità alta: bisogno professionale reale, non più uno sfizio rimandabile |
| Technical | Toolchain Rust/Leptos corrente; budget di dimensione del bundle wasm (problema storico già segnalato); deploy senza processo server sempre attivo (SSG) |
| Regulatory | Google Tag Manager raccoglie dati di navigazione; il consenso/cookie-banner è demandato a un epic compliance dedicato (vedi Open Issues) — GTM non va in produzione da EP-001 prima che quell'epic lo copra |

## Acceptance Criteria

| ID | Criterion | Verified by |
|---|---|---|
| AC-1 | Il sito compila e genera un output statico deployabile senza processo server sempre attivo | Esecuzione della build e verifica dell'assenza di un processo server in produzione — **build sbloccata** (2026-08-20); output ancora SSR, non SSG |
| AC-2 | È possibile pubblicare un nuovo articolo scrivendo un file markdown+frontespizio nel repo contenuti dedicato e pushando, senza modificare codice applicativo | Pubblicazione end-to-end di un articolo di prova |
| AC-3 | Il sito generato ottiene punteggio Lighthouse >90 (performance) | Audit Lighthouse sull'output di build |
| AC-4 | Il layout rispetta WCAG 2.2 AA | Audit di accessibilità (es. axe / Lighthouse a11y) |
| AC-5 | Google Tag Manager è attivo e traccia le visite | Verifica in GTM/GA che gli eventi di visita arrivano |

## ADRs

| ADR | Title | Date | Status |
|---|---|---|---|
| [ADR-001](../adr/ADR-001-leptos-target-version.md) | Leptos target version | 2026-08-14 | Applicata (2026-08-20) |
| — | SSG + islands al posto di SSR | — | Da creare |
| [ADR-002](../adr/ADR-002-content-markdown-in-git.md) | Content come markdown-in-git via porta ContentSource, nessun CMS | 2026-08-23 | Accepted (dominio implementato; adapter filesystem e repo contenuti dedicato ancora da creare) |
| [ADR-003](../adr/ADR-003-repo-topology-github-pages-hosting.md) | Topologia repo e hosting per il deploy su GitHub Pages | 2026-08-23 | Proposed (decisione concordata; cambio visibilità repo e creazione repo contenuti non ancora eseguiti) |

> La ADR mancante (SSG + islands al posto di SSR) resta da creare con `/adr` — raccomandata dall'assessment architetturale collegato, non ancora formalizzata.

## Use Cases

| UC ID | Title | Goal level | Status |
|---|---|---|---|
| UC-001 | Autore pubblica un nuovo articolo | ⚡ User-Goal | Draft |
| UC-002 | Visitatore legge un articolo pubblicato | ⚡ User-Goal | Draft |
| UC-003 | Visitatore sfoglia l'elenco degli articoli | ⚡ User-Goal | Draft |
| UC-004 | Header si adatta allo scroll e al contesto della pagina | ⬇ Subfunction | Draft |
| UC-005 | Visitatore usa il menu di navigazione | ⬇ Subfunction | Draft |

## Open Issues

- ~~GTM è in scope di questo epic ma il consenso/cookie-banner è esplicitamente fuori scope...~~ — **risolto** (2026-08-21): il consenso/cookie-banner sarà affrontato in un epic compliance dedicato, separato da EP-001 (non ancora creato/numerato). **Vincolo:** GTM non va distribuito in produzione da EP-001 finché quell'epic non ha coperto il consenso — rischio di non conformità UE altrimenti.
- ~~Il dominio `Article` raccomandato dall'assessment copre un singolo articolo, senza listing/indice...~~ — **risolto** (2026-08-21): un indice (LISTING-PAGE, con HOME-PAGE come suo caso particolare in ordine cronologico) rientra in questo epic — vedi [UC-003](../use-cases/UC-003-visitatore-sfoglia-elenco-articoli.md). Restano aperte, come da UC-003/UC-005: meccanismo di paginazione, e se il layout a slider orizzontale con effetto lente (design alternativo alla lista verticale) entra in questo epic o resta un incremento futuro.
- La ricerca full-text e i facet dinamici sui tag (voce "ricerca" nel menu) sono demandati all'epic EP-004 (motore di ricerca client-side) — vedi Open Issues in [UC-005](../use-cases/UC-005-visitatore-usa-menu-navigazione.md). In EP-001 i tag restano statici/predefiniti.
- Le voci di menu "About Me" e "CV" compaiono come segnaposto non funzionanti, poiché le relative pagine sono esplicitamente fuori scope di questo epic (vedi UC-005).

---

| Updated | What changed |
|---|---|
| 2026-08-19 | Epic created, a partire dall'assessment architetturale docs/analysis/architecture-assessment-lucalorenzon-blog.md (architecture-compass + software-design, agreed 2026-08-18) |
| 2026-08-20 | Aggiornamento dipendenze completato via lane `chore` (non UC/story, per CLAUDE.md thin-lane routing): Leptos 0.6→0.8.x, toolchain nightly→stable, `experimental-islands`→`islands` (ADR-001 applicata, commit `169b30c`); fix hydrate `mark_branches` (commit `9096976`); migrazione Tailwind v3→v4 (commit `08c6030`); bump Playwright + fix tipi end2end (commit `602887c`, `d5cdff5`). Il sito torna a compilare (AC-1 parzialmente soddisfatto, output ancora SSR). Restano da fare: content layer, migrazione SSG, i due ADR mancanti, verifica NFR (Lighthouse/WCAG/GTM) |
| 2026-08-21 | Definite UC-001..UC-005 (pubblicazione articolo, lettura articolo, sfoglio elenco, header adattivo, menu di navigazione). Risolto l'Open Issue sull'indice/listing (in scope, vedi UC-003). Chiariti i confini con EP-004 (ricerca live/facet fuori scope qui) e con le pagine di identità professionale (About Me/CV segnaposto in menu, pagine reali fuori scope) |
| 2026-08-21 | Riaperta UC-001 per risolvere i suoi due Open Issues: trigger build = CI automatico su push (GitHub Actions, coerente con AC-2), canale errori = check fallito GitHub Actions + email di notifica standard. Estensione ex-1a (frontespizio invalido) rinumerata a 3a poiché rilevata da CI dopo il push, non prima; AT-UC-001 aggiornata di conseguenza (celle `?UNKNOWN?` risolte, `push tentato?` corretto a `true`) |
| 2026-08-23 | Ricerca di mercato su CMS git-based confermata: ADR-002 resta valida (Sveltia CMS notato come opzione UI futura non prioritaria). Creata ADR-003 (topologia repo e hosting GitHub Pages): 2 repo, `lucalorenzon-blog` da rendere pubblico invece di GitHub Pro, repo contenuti dedicato può restare privato. Azioni infrastrutturali (cambio visibilità, creazione repo contenuti) non ancora eseguite — previste come step successivo esplicito |
