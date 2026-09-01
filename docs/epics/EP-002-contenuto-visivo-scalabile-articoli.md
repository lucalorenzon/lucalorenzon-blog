# EP-002: Contenuto visivo scalabile per gli articoli

> Disaccoppiare le immagini degli articoli dal versionamento git e renderle correttamente responsive, così che il repo contenuti non degradi in dimensione/tempi di build e i lettori vedano sempre l'immagine giusta per il proprio dispositivo.

---

## Motivation

Il sito è responsive con breakpoint dichiarati (`sm/md/lg/xl` in `input.css`), ma il modello immagine di oggi (`Article.image`: singolo path opzionale, fallback SVG se assente/rotto, fix minimo introdotto in EP-001-UC-001-S003) non regge né una vera separazione degli asset né un adattamento per viewport. Il problema è emerso concretamente durante EP-001-UC-001-S003, con bug reali trovati solo con una build reale (path immagine sbagliato, file mai copiato nel sito generato) — non è un rischio ipotetico.

Un articolo raramente è solo testo: oltre a un'immagine che lo rappresenta visivamente (con varianti potenzialmente diverse per breakpoint), il corpo stesso contiene tipicamente altre immagini — diagrammi, esempi, riferimenti — scritte dall'autore nello stesso punto in cui scrive il testo, non caricate altrove.

Se queste immagini restano versionate su git insieme al markdown (come oggi), il repo contenuti cresce in dimensione nel tempo, le build rallentano, e si rischia di superare i limiti di spazio del piano gratuito GitHub. Va fatto ora, prima che il volume di articoli/immagini cresca e renda il problema più costoso da risolvere.

## Context

Oggi: un'unico campo immagine opzionale per articolo, nessun resize, nessuna variante per breakpoint, nessuna gestione delle immagini citate nel corpo del testo, storage delle immagini nello stesso repo contenuti del markdown (fix temporaneo di EP-001-UC-001-S003: `images_dir` copiata in `site-root/images` a build time, senza convenzioni di naming né gestione collisioni).

Il meccanismo previsto per risolvere questo è una singola sorgente di asset (`AssetSource`, per analogia con `ContentSource` di EP-001), responsabile di due cose: salvare un'immagine dato il suo nome, e fornire un URL SEO-friendly con cui l'articolo la referenzia. La stessa sorgente può servire sia l'immagine di copertina sia le immagini nel corpo — eventualmente su cartelle o prefissi diversi — ma non è un vincolo che debbano essere due sorgenti separate. L'autore continua a scrivere le immagini nella stessa cartella del file markdown dell'articolo (flusso di authoring naturale, invariato rispetto a oggi); un automatismo si occupa di spostarle/pubblicarle sullo storage dedicato e di generare l'URL con cui il markdown le referenzia, in modo che lo stesso link funzioni sia in locale (`cargo leptos watch`) sia online.

## Business Outcome

- L'immagine di copertina di un articolo si presenta in modo appropriato (non tagliata male, non sovradimensionata) su ciascun breakpoint del sito
- Le immagini nel corpo di un articolo (diagrammi, esempi, riferimenti) sono referenziabili nel markdown come se vivessero nella stessa cartella del file, senza passi manuali aggiuntivi per l'autore
- Il link a un'immagine funziona identico in sviluppo locale e in produzione, senza istruzioni diverse per l'autore nei due ambienti
- Le immagini pubblicate non restano versionate nel repo contenuti dopo la pubblicazione: dimensione e tempi di build del repo restano indipendenti dal volume di immagini pubblicate nel tempo
- Il repo contenuti resta entro i limiti di spazio del piano gratuito GitHub anche con la crescita del numero di articoli/immagini

## Actors

| Actor | Type | Role in this epic |
|---|---|---|
| Autore/Editore | Primary | Scrive articoli in markdown con immagini nella stessa cartella del file, senza gestire storage o URL a mano |
| Visitatore del sito | Primary | Vede immagini corrette e appropriate al proprio dispositivo, sia in copertina sia nel corpo dell'articolo |

## Scope

### In scope
- Modello "immagine di copertina" dell'articolo con varianti per breakpoint (`sm/md/lg/xl`)
- Modello "immagini nel corpo articolo" (lista, potenzialmente multiple, distinte dalla copertina)
- Convenzione di authoring: le immagini vivono nella stessa cartella del file markdown nel repo contenuti
- `AssetSource`: sorgente unica (eventualmente su cartelle/prefissi diversi per copertina/corpo) che salva un'immagine dato il nome e fornisce l'URL SEO-friendly con cui l'articolo la referenzia
- Automatismo che sposta/pubblica le immagini dal punto di authoring allo storage dedicato, senza passi manuali per l'autore
- Parità di comportamento dell'URL tra ambiente locale e produzione
- ADR sulla scelta del meccanismo/servizio di storage concreto (non decisa qui)

### Out of scope
- Come viene generata o scelta l'immagine di copertina (fotografia, illustrazione, IA...) — resta responsabilità editoriale dell'autore
- Editor visuale di crop/ritaglio per l'autore
- Elaborazione automatica del contenuto delle immagini (OCR, alt-text automatico, ecc.)
- Scelta del servizio/backend di storage concreto (object storage esterno, branch dedicato, servizio di image hosting...) — materia di ADR dedicata durante il design

## Constraints

| Type | Constraint |
|---|---|
| Business | Priorità media: non blocca la pubblicazione articoli, che oggi funziona con il fix minimo di EP-001-UC-001-S003 |
| Technical | Sito SSG (ADR-004): nessuna elaborazione immagine a runtime lato server always-on; repo contenuti su piano gratuito GitHub (limiti di spazio); build time non deve degradare con la crescita del volume immagini; l'URL di un'immagine deve risolversi correttamente sia in sviluppo locale (`cargo leptos watch`) sia in produzione (GitHub Pages + dominio custom), senza divergenza di comportamento tra i due ambienti |
| Regulatory | Nessuno noto |

## Acceptance Criteria

| ID | Criterion | Verified by |
|---|---|---|
| AC-1 | L'immagine di copertina di un articolo mostra una variante appropriata su ciascun breakpoint del sito | Verifica visiva/audit su build reale a ciascun breakpoint |
| AC-2 | Un'immagine referenziata nel corpo del markdown (stessa cartella del file) viene pubblicata correttamente e raggiungibile dall'HTML generato | Build reale + verifica che il link risolva (no 404) |
| AC-3 | Lo stesso link immagine funziona sia in ambiente locale sia in produzione | Verifica in `cargo leptos watch` e sul sito pubblicato |
| AC-4 | Le immagini pubblicate non restano versionate nel repo contenuti dopo la pubblicazione | Ispezione della storia/dimensione del repo contenuti dopo una pubblicazione di prova |
| AC-5 | Il repo contenuti resta entro i limiti di spazio del piano gratuito GitHub con un volume di articoli/immagini realistico | Verifica dimensione repo contro i limiti noti del piano |

## ADRs

| ADR | Title | Date | Status |
|---|---|---|---|
| — | — | — | — |

## Use Cases

| UC ID | Title | Goal level | Status |
|---|---|---|---|
| — | — | — | — |

## Open Issues

- Granularità del responsive: l'immagine di copertina avrà varianti per breakpoint (art-direction); non deciso se le immagini nel corpo dell'articolo necessitano dello stesso trattamento o di un semplice resize responsivo — da risolvere in fase di use-case/story
- Servizio/backend concreto per lo storage disaccoppiato da git non ancora scelto — materia di ADR dedicata
- Convenzione esatta dell'URL SEO-friendly (come il path locale nel markdown si traduce nell'URL servito, sia in locale sia online) non ancora decisa
- Confine con l'automazione S004 (pipeline CI di EP-001): dove vive l'automatismo che sposta le immagini dal punto di authoring allo storage dedicato (step della stessa CI, processo separato) — da chiarire quando si disegna S004 o l'UC di questo epic

---

| Updated | What changed |
|---|---|
| 2026-09-01 | Epic created — emerso come residuo #7 dello Stressor Analysis di EP-001-UC-001-S003 (30/08), ampliato da Luca il 01/09 con il tema breakpoint responsive e la framing `AssetSource` (salvataggio + URL SEO-friendly, un'unica sorgente per copertina e corpo) |
