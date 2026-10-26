## EP-001-UC-001-S006 — Pubblicare una homepage "in costruzione" sotto una context root configurabile (`CONTEXT_ROOT`)

> Creata nel replan del 2026-10-26 come "`base_url` configurabile", rinominata `CONTEXT_ROOT` e poi divisa per decisione lo stesso giorno (dimensionata XL, hard stop): questa storia è il primo pezzo (D1). Gli altri: [[EP-001-UC-001-S011]] (islands), [[EP-001-UC-001-S012]] (pagine di esempio dal repo contenuti), [[EP-001-UC-001-S013]] (link nel markdown). Il nome del file resta invariato per non rompere i riferimenti.
> **Ordine di consegna: 1° di 10** (**S006** → S005 → S011 → S012 → S013 → S007 → S008 → S004 → S009 → S010).

**Decision:** Come si definisce `CONTEXT_ROOT` (contratto: obbligatoria, forma del valore, dove si dichiara in sviluppo e in CI) e come le risorse del codice (CSS, favicon) e la struttura dell'output ne derivano, provato su una homepage temporanea "in costruzione" scritta nel codice. Principio trasversale di tutta la serie: ogni percorso prodotto dal sito deriva da `CONTEXT_ROOT`, spostare il sito è un cambio di configurazione più una nuova build, mai una revisione dei link. Richiede un ADR.

Per mettere online questa homepage senza percorsi rotti, la storia **sospende temporaneamente** due cose, ciascuna riattivata da una storia nominata:
- le **islands** (menu, cambio tema, header dinamico) → riattivate da [[EP-001-UC-001-S011]]; deviazione temporanea da ADR-004 da annotare;
- la **generazione dei contenuti** (route di articoli ed elenco, lettura del repo contenuti, copia delle immagini) → riattivata da [[EP-001-UC-001-S012]].

As Visitatore del sito, voglio vedere all'indirizzo pubblico del sito una homepage "in costruzione" completa di stili, logo e favicon, so that capisco che il sito esiste e sta arrivando, senza pagine rotte.

| Field | Value |
|---|---|
| **Epic** | EP-001 — Rilancio del blog personale come presenza professionale |
| **UC** | UC-001 — Autore pubblica un nuovo articolo |
| **Pattern** | Major effort isolation (split per decisione da XL) |
| **AT rows** | AT-UC-001 righe: 1-5 (success guarantee "raggiungibile pubblicamente", limitatamente alla homepage); epic AC-1 |
| **Size** | L — estimated 2026-10-26 dopo lo split (25 righe AT, ~5 componenti, precedente `CONTENT_REPO_PATH`/`Slug::parse`); l'utente la considera fattibile in una sessione. Prima dello split: XL (41 righe AT) |

### Acceptance criteria
- Given `CONTEXT_ROOT` = `/lucalorenzon-blog/`, when il sito generato è servito sotto quel prefisso, then la homepage "in costruzione" si carica con CSS e favicon, senza 404
- Given `CONTEXT_ROOT` = `/` (dominio personale), when il sito generato è servito alla radice, then vale lo stesso
- Given `CONTEXT_ROOT` non definita, when si lancia la build, then la build si ferma con un errore che la nomina
- Given due build con `CONTEXT_ROOT` diverse, when si confronta l'output, then l'elenco dei file è identico (nessuna sottocartella col nome del prefisso)
- Given la build di questa storia, when termina, then l'output non contiene pagine di articoli né l'elenco, e la build non legge il repo contenuti
- Given la homepage generata, when la si apre, then l'header contiene solo logo e titolo (nessun menu, nessuna icona di ricerca), c'è il footer statico e l'HTML non contiene script di hydration
- Given lo sviluppo locale (`cargo leptos watch`) con `CONTEXT_ROOT` = `/` dichiarata esplicitamente nella configurazione di sviluppo, when il sito è servito in locale, then la homepage si carica senza 404

Acceptance tests: [AT-EP-001-UC-001-S006](../acceptance-tests/AT-EP-001-UC-001-S006-servire-sito-context-root-configurabile.md) — righe del pezzo D1 approvate il 2026-10-26, riviste lo stesso giorno (righe delle sospensioni aggiunte, righe dei PNG del logo tolte).

### Design pipeline
Before any implementation, complete in order:
- [ ] `/adr`                    — `CONTEXT_ROOT` come principio trasversale; nota in ADR-004 sulla sospensione temporanea delle islands
- [ ] `/residuality`            — Step 0 decide se si applica; poi ricontrollo `/story-size`
- [ ] `/software-design`        — coupling, ownership, accidental complexity
- [ ] `/hexagonal-architecture` — ports, adapters, composition root
- [ ] `/parse-dont-validate`    — domain types and invariants (forma del valore di `CONTEXT_ROOT`)
- [ ] `/sw-practices`           — naming, error handling, bootstrap

### Next steps after agreement
- [x] `/acceptance-tests EP-001-UC-001-S006` — revisione: righe delle sospensioni, homepage "in costruzione" (`ca9f825`)
- [x] `/story-size EP-001-UC-001-S006`       — rimisurare dopo lo split: L (`b94dea3`)

### Open questions
- **Meccanismo Leptos (design, non requisito).** Verificato sul sorgente (leptos 0.8.22, leptos_router 0.8.17): `HydrationScripts` ha la prop `root` ("a base url, not including a trailing slash"), `Router` ha la prop `base`. **Da verificare:** se `Router base` fa scrivere alla generazione statica i file sotto `site_root/<prefisso>/…` (sbagliato per Pages, dove il prefisso corrisponde già alla radice dell'artefatto). Candidato a uno `/spike` breve in `/adr` o `/software-design`: riguarda questa storia per la riga "struttura dell'output identica".
- **Lettura in compilazione o in generazione** (vedi [[EP-001-UC-001-S008]]): accettato che spostare il sito richieda una nuova Release, quindi la lettura in compilazione è ammessa; la scelta concreta resta a `/adr`. **Dato in più (2026-10-26):** il CSS generato *non* è privo di percorsi: `src/layout.rs:20` usa `bg-[url('/images/ostia_sea_top_image.webp')]` (sfondo delle pagine articolo), e Tailwind lo scrive nel CSS come `url("/images/ostia_sea_top_image.webp")`. Con la lettura in generazione questo percorso resterebbe sbagliato nella Release; riguarda le pagine articolo ([[EP-001-UC-001-S012]]).
- **`CONTENT_REPO_PATH` durante la sospensione dei contenuti** (design): resta obbligatoria pur senza lettori, o torna con [[EP-001-UC-001-S012]]?

### Dependencies
- Nessuna: è la prima storia dell'ordine di consegna. [[EP-001-UC-001-S005]] (deploy su push di codice) la segue subito, per portarla online.

### Decisions Log
| Date | Decision | Reasoning | Alternatives Considered |
|---|---|---|---|
| 2026-10-26 | `CONTEXT_ROOT` obbligatoria: se manca la build si ferma; nessun valore di default, anche in sviluppo (lì `/` è dichiarata esplicitamente) | Un sito deve essere rilocabile; un default `/` dimenticato in CI pubblicherebbe sotto prefisso un sito rotto senza errore — proprio il guasto che la storia previene. Esperienza dell'utente: "tanto non lo si sposta mai" ⇒ lo si sposta | Default `/` (più comodo in sviluppo, ma fallisce in silenzio) |
| 2026-10-26 | Spostare il sito = cambio di configurazione + nuova build (una nuova Release, vedi S008); i link non si toccano mai | Basta che lo spostamento non richieda di rivedere i link; un output che funzioni ovunque senza ricompilare non è richiesto | Output rilocabile senza rebuild (percorsi relativi per pagina); rigenerazione con gli artefatti della Release esistente senza ricompilare |
| 2026-10-26 | La context root è solo il percorso (`/lucalorenzon-blog/`, `/`), non l'origine | L'origine (`https://…`) servirà a canonical/sitemap, oggi fuori scope; arriverà come configurazione separata | URL completo come unico valore |
| 2026-10-26 | Nome: `CONTEXT_ROOT`, non `BASE_URL` | "url" suggerisce un indirizzo completo; il concetto è il percorso da cui il sito è servito | `BASE_URL`, `base_url` |
| 2026-10-26 | Favicon rientra qui. Il "bug del logo" (`/assets/logoIcon.png`) non esiste nel sito servito: è nel componente `SmallImageIcon`, definito ma non usato (il logo dell'header è testo `Lvk@73r` e ASCII art). Il codice morto resta fuori da questa storia | Rilevato in revisione AT confrontando codice e output reale; la decisione precedente ("logo e favicon in S006") si basava su un fatto errato | Rimuovere `SmallImageIcon` qui (sarebbe un `refactor` separato) |
| 2026-10-26 | Dimensionata XL (41 righe AT) → split per decisione, non per numero di righe, in quattro storie in base al valore visibile: D1 homepage "in costruzione" (questa), islands ([[EP-001-UC-001-S011]]), pagine di esempio ([[EP-001-UC-001-S012]]), link nel markdown ([[EP-001-UC-001-S013]]) | Le storie dividono le decisioni; molte righe sotto una sola decisione si affrontano in una sessione dedicata. Ogni pezzo ha valore visibile da solo | Tagli per ridurre righe (es. solo configurazione: nessun valore visibile) |
| 2026-10-26 | D1 online subito: il deploy su push di codice ([[EP-001-UC-001-S005]]) viene subito dopo questa storia | L'obiettivo è vedere la homepage sul sito, non solo in locale | Verifica solo in locale sotto prefisso, deploy dopo tutta la serie |
| 2026-10-26 | Niente contenuto online fino a [[EP-001-UC-001-S012]]: questa storia sospende la generazione dei contenuti; la pagina "in costruzione" è scritta nel codice | Pubblicare pagine di articoli con immagini e link rotti, anche se non linkate, non è accettabile | Pubblicare anche le pagine degli articoli generate dal repo contenuti |
| 2026-10-26 | Testo della homepage temporanea: `Site under construction`, in inglese | Il sito è dichiarato `lang="en"` e l'interfaccia è in inglese; un testo italiano sarebbe incoerente per screen reader e SEO | "Sito in costruzione" in italiano |
| 2026-10-26 | Islands sospese fino a [[EP-001-UC-001-S011]]; nell'header restano solo logo e titolo: niente menu (il selettore tema sarebbe inerte) e niente icona di ricerca (inerte già oggi, nessuna funzione di ricerca) | Nessun comando visibile senza effetto | Menu visibile con selettore inerte |

INVEST: I✓ N✓ V✓ E✓ S✓ T✓  |  1 Decision: ✓  |  Coherence: ✓
