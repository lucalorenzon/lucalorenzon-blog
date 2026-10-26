## EP-001-UC-001-S006 — Servire il sito sotto una context root configurabile (`CONTEXT_ROOT`)

> Creata nel replan del 2026-10-26. **Ordine di consegna: 1° di 7** (**S006** → S005 → S007 → S008 → S004 → S009 → S010).
> Nata come "`base_url` configurabile": rinominata `CONTEXT_ROOT` il 2026-10-26 (vedi Decisions Log). Il nome del file resta invariato per non rompere i riferimenti.

**Decision:** Come ogni percorso prodotto dal sito (route, CSS, wasm/js in `/pkg`, immagini, asset, link interni — compresi quelli scritti dall'autore nel markdown degli articoli) deriva da una `CONTEXT_ROOT` sempre definita e configurabile, invece di percorsi assoluti dalla radice. È un principio permanente, non un adattamento temporaneo all'indirizzo di GitHub Pages: un sito deve essere rilocabile, e spostarlo (es. al dominio personale) è un cambio di configurazione più una nuova build, mai una revisione dei link. Decisione trasversale → richiede un ADR.

As Visitatore del sito, voglio che stili, immagini, link e interattività funzionino a qualunque indirizzo il sito sia pubblicato, so that vedo sempre un sito completo e navigabile.

| Field | Value |
|---|---|
| **Epic** | EP-001 — Rilancio del blog personale come presenza professionale |
| **UC** | UC-001 — Autore pubblica un nuovo articolo |
| **Pattern** | Major effort isolation |
| **AT rows** | AT-UC-001 righe: 1-5 (success guarantee "articolo raggiungibile pubblicamente"); epic AC-1 |

### Acceptance criteria
- Given `CONTEXT_ROOT` = `/lucalorenzon-blog/`, when il sito generato è servito sotto quel prefisso, then CSS, wasm/js, immagini, asset (logo e favicon compresi) e link interni risolvono tutti, senza 404
- Given `CONTEXT_ROOT` = `/` (dominio personale), when il sito generato è servito alla radice, then vale lo stesso
- Given `CONTEXT_ROOT` non definita, when si lancia la build, then la build si ferma con un errore che la nomina
- Given un articolo il cui markdown contiene un link o un'immagine con percorso dalla radice (`/articles/altro`, `/images/x.webp`), when il sito è generato con un prefisso, then nell'HTML il percorso include la `CONTEXT_ROOT`; link esterni (`https://…`, `//…`) e ancore (`#…`) restano invariati
- Given il sito servito sotto un prefisso, when il visitatore usa le islands (cambio tema, header), then l'hydration funziona
- Given lo sviluppo locale (`cargo leptos watch`) con `CONTEXT_ROOT` = `/` dichiarata esplicitamente nella configurazione di sviluppo, when il sito è servito in locale, then continua a funzionare

Acceptance tests: [AT-EP-001-UC-001-S006](../acceptance-tests/AT-EP-001-UC-001-S006-servire-sito-context-root-configurabile.md)

### Design pipeline
Before any implementation, complete in order:
- [ ] `/adr`                    — `CONTEXT_ROOT` configurabile come principio trasversale
- [ ] `/residuality`            — Step 0 decide se si applica; poi ricontrollo `/story-size`
- [ ] `/software-design`        — coupling, ownership, accidental complexity
- [ ] `/hexagonal-architecture` — ports, adapters, composition root
- [ ] `/parse-dont-validate`    — domain types and invariants
- [ ] `/sw-practices`           — naming, error handling, bootstrap

### Next steps after agreement
- [ ] `/acceptance-tests EP-001-UC-001-S006` — story-level AT table (narrower than UC level)
- [ ] `/story-size EP-001-UC-001-S006`       — assign XS / S / M / L / XL / XXL

### Open questions
- **Meccanismo Leptos (design, non requisito).** Verificato sul sorgente (leptos 0.8.22, leptos_router 0.8.17): `HydrationScripts` ha la prop `root` ("a base url, not including a trailing slash"), `Router` ha la prop `base`; l'islands router usa gli `href` così come sono (il controllo sul `basePath` è commentato in `islands_routing.js`), quindi i link devono già contenere il prefisso. **Da verificare:** se `Router base` fa scrivere alla generazione statica i file sotto `site_root/<prefisso>/…` (sbagliato per Pages, dove il prefisso corrisponde già alla radice dell'artefatto). Candidato a uno `/spike` breve in `/adr` o `/software-design`.
- **Lettura in compilazione o in generazione** (vedi [[EP-001-UC-001-S008]]): accettato che spostare il sito richieda una nuova Release, quindi la lettura in compilazione è ammessa; la scelta concreta resta a `/adr`. Se si sceglie la generazione, wasm/js/CSS della Release non devono contenere percorsi (oggi è così: nessun `url()` nel CSS, nessun percorso nelle islands).

### Dependencies
- Nessuna: è la prima storia dell'ordine di consegna.

### Decisions Log
| Date | Decision | Reasoning | Alternatives Considered |
|---|---|---|---|
| 2026-10-26 | `CONTEXT_ROOT` obbligatoria: se manca la build si ferma; nessun valore di default, anche in sviluppo (lì `/` è dichiarata esplicitamente) | Un sito deve essere rilocabile; un default `/` dimenticato in CI pubblicherebbe sotto prefisso un sito rotto senza errore — proprio il guasto che la storia previene. Esperienza dell'utente: "tanto non lo si sposta mai" ⇒ lo si sposta | Default `/` (più comodo in sviluppo, ma fallisce in silenzio) |
| 2026-10-26 | Spostare il sito = cambio di configurazione + nuova build (una nuova Release, vedi S008); i link non si toccano mai | Basta che lo spostamento non richieda di rivedere i link; un output che funzioni ovunque senza ricompilare non è richiesto | Output rilocabile senza rebuild (percorsi relativi per pagina); rigenerazione con gli artefatti della Release esistente senza ricompilare |
| 2026-10-26 | La context root è solo il percorso (`/lucalorenzon-blog/`, `/`), non l'origine | L'origine (`https://…`) servirà a canonical/sitemap, oggi fuori scope; arriverà come configurazione separata | URL completo come unico valore |
| 2026-10-26 | Nome: `CONTEXT_ROOT`, non `BASE_URL` | "url" suggerisce un indirizzo completo; il concetto è il percorso da cui il sito è servito | `BASE_URL`, `base_url` |
| 2026-10-26 | I link interni scritti dall'autore nel markdown sono in scope: la conversione markdown→HTML aggiunge da sola la `CONTEXT_ROOT` ai link e alle immagini con percorso dalla radice (opzione A). Ripiego su segnaposto `$CONTEXT_ROOT/…` scritto dall'autore (opzione B) solo se A risulta complicata in design | A lascia all'autore la regola più semplice ("scrivi come se il sito fosse alla radice") e resta solo codice. Fattibilità: `pulldown-cmark` 0.13 (già usato) espone `Tag::Link`/`Tag::Image` con `dest_url`. Limite accettato: HTML grezzo nel markdown (`<a href>`, `<img src>`) non viene riscritto | B — segnaposto: sostituzione banale, ma diventa un contratto del contenuto da documentare e un link dimenticato si rompe in silenzio |
| 2026-10-26 | Logo (`/assets/logoIcon.png`, bug preesistente: gli asset sono copiati alla radice del sito) e favicon rientrano in S006, non in un `fix` separato | L'AC "asset senza 404" li copre già; correggerli altrove duplicherebbe lo stesso lavoro sui percorsi | `fix` dedicato per il logo |
| 2026-10-26 | La riscrittura dei link markdown non è una seconda decisione: stesso principio applicato a un punto in più, nessuno split | 1 storia = 1 decisione resta rispettato; il rischio di crescita lo misura `/story-size` (hard stop oltre L) | Storia separata per i link nel contenuto |

INVEST: I✓ N✓ V✓ E✓ S✓ T✓  |  1 Decision: ✓  |  Coherence: ✓
