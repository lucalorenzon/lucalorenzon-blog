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
- Aggiornamento delle dipendenze Cargo e del toolchain a versioni correnti mantenute (sblocca la build, oggi rotta)
- Dominio `Article` (parse-dont-validate sul frontmatter: data, slug, tag) e porta `ContentSource` con adapter filesystem sul repo contenuti dedicato
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
| Regulatory | Google Tag Manager raccoglie dati di navigazione; il consenso/cookie-banner non è gestito in questo epic (vedi Open Issues) |

## Acceptance Criteria

| ID | Criterion | Verified by |
|---|---|---|
| AC-1 | Il sito compila e genera un output statico deployabile senza processo server sempre attivo | Esecuzione della build e verifica dell'assenza di un processo server in produzione |
| AC-2 | È possibile pubblicare un nuovo articolo scrivendo un file markdown+frontespizio nel repo contenuti dedicato e pushando, senza modificare codice applicativo | Pubblicazione end-to-end di un articolo di prova |
| AC-3 | Il sito generato ottiene punteggio Lighthouse >90 (performance) | Audit Lighthouse sull'output di build |
| AC-4 | Il layout rispetta WCAG 2.2 AA | Audit di accessibilità (es. axe / Lighthouse a11y) |
| AC-5 | Google Tag Manager è attivo e traccia le visite | Verifica in GTM/GA che gli eventi di visita arrivano |

## ADRs

| ADR | Title | Date | Status |
|---|---|---|---|
| — | — | — | — |

> Da creare con `/adr`: "SSG + islands al posto di SSR" e "Content come markdown-in-git via porta ContentSource, nessun CMS per ora" — entrambe raccomandate dall'assessment architetturale collegato, non ancora formalizzate come ADR.

## Use Cases

| UC ID | Title | Goal level | Status |
|---|---|---|---|
| — | — | — | — |

## Open Issues

- GTM è in scope di questo epic ma il consenso/cookie-banner è esplicitamente fuori scope — rischio concreto di non conformità UE se GTM va in produzione prima che il consenso sia gestito. Da risolvere prima del deploy in produzione, qui o in un epic compliance separato — non ancora deciso.
- Il dominio `Article` raccomandato dall'assessment copre un singolo articolo, senza listing/indice. Un epic che "pubblica articoli" ma può mostrarne solo uno alla volta rischia di non essere realmente utilizzabile a fine epic. Da decidere se un indice minimale rientra in questo epic o resta nell'epic content-strategy/SEO separato che copre gli altri gap dichiarati fuori scope.

---

| Updated | What changed |
|---|---|
| 2026-08-19 | Epic created, a partire dall'assessment architetturale docs/analysis/architecture-assessment-lucalorenzon-blog.md (architecture-compass + software-design, agreed 2026-08-18) |
