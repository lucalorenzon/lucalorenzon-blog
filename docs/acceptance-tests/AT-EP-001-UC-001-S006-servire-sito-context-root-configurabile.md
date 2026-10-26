# AT-EP-001-UC-001-S006: Pubblicare una homepage "in costruzione" sotto una context root configurabile (`CONTEXT_ROOT`)

Epic: EP-001 | UC: UC-001 | Story: EP-001-UC-001-S006

Scope: sottoinsieme più fine della success guarantee "articolo raggiungibile
pubblicamente" (righe Happy path 1-5 di
[AT-UC-001](AT-UC-001-autore-pubblica-articolo.md)) ed epic AC-1, limitato
alla homepage "in costruzione" e alle risorse del codice. Il deploy su GitHub
Pages resta fuori scope ([[EP-001-UC-001-S005]]).

> **Split del 2026-10-26.** Questa tabella conteneva 41 righe approvate per
> l'intera S006. Dopo lo split per decisione restano qui le righe del pezzo
> D1; le altre sono state spostate, senza riscriverle, in
> [AT-S011](AT-EP-001-UC-001-S011-reintrodurre-islands-context-root.md)
> (js/wasm, islands),
> [AT-S012](AT-EP-001-UC-001-S012-pubblicare-pagine-esempio-repo-contenuti.md)
> (percorsi dei contenuti, pagine servite) e
> [AT-S013](AT-EP-001-UC-001-S013-riscrivere-link-markdown-context-root.md)
> (link nel markdown). La riga duplicata "`CONTEXT_ROOT` non definita" dello
> sviluppo locale è stata tolta (coperta da Configurazione). Unica modifica
> a una riga approvata: la homepage servita è la pagina "in costruzione".
>
> **Revisione del 2026-10-26** (`/acceptance-tests` dopo lo split): tolte le
> due righe "logo grande / logo piccolo" (PNG), perché il logo dell'header è
> testo e ASCII art e il componente con i PNG (`SmallImageIcon`) non è usato;
> aggiunte le righe delle due sospensioni (contenuti, islands/header).

Le righe con percorsi assoluti dalla radice in
[AT-EP-001-UC-001-S003](AT-EP-001-UC-001-S003-generare-sito-statico-ssg.md)
restano valide come caso `CONTEXT_ROOT` = `/` (quelle sui contenuti tornano
attive con [[EP-001-UC-001-S012]]).

Valori di riferimento: `output-name` = `blog_start`.

---

## Configurazione: valore di `CONTEXT_ROOT` alla build

> Source: Story AC-1, AC-2, AC-3

| `CONTEXT_ROOT` | esito della build? | pagine HTML generate? | ref |
|---|---|---|---|
| `/lucalorenzon-blog/` | completata, exit code `0` | sì | AC-1 |
| `/` | completata, exit code `0` | sì | AC-2 |
| non definita | interrotta, exit code `≠ 0`, il messaggio d'errore contiene `CONTEXT_ROOT` | nessuna | AC-3 |

### Forma del valore (edge case)

> Source: Story AC-3 + Decisions Log ("solo il percorso")

| `CONTEXT_ROOT` | esito della build? | ref |
|---|---|---|
| `""` (definita ma vuota) | ?UNKNOWN? | AC-3 |
| `/lucalorenzon-blog` (senza `/` finale) | ?UNKNOWN? | AC-1 |
| `lucalorenzon-blog/` (senza `/` iniziale) | ?UNKNOWN? | AC-1 |
| `https://lucalorenzon.github.io/lucalorenzon-blog/` (con origine) | ?UNKNOWN? | AC-1 |

---

## Percorsi generati: risorse del codice

> Source: Story AC-1, AC-2. Valore dell'attributo nell'HTML della homepage.

| elemento | con `CONTEXT_ROOT` = `/lucalorenzon-blog/`? | con `CONTEXT_ROOT` = `/`? | ref |
|---|---|---|---|
| foglio di stile (`<link rel="stylesheet">` `href`) | `/lucalorenzon-blog/pkg/blog_start.css` | `/pkg/blog_start.css` | AC-1, AC-2 |
| favicon (`<link rel="icon">` `href`) | `/lucalorenzon-blog/favicon.ico` | `/favicon.ico` | AC-1, AC-2 |

## Proprietà globali dell'output

> Source: Story AC-1, AC-2, AC-4. Valutate su tutti gli attributi `href`/`src`/`srcset` di tutte le pagine HTML generate, esclusi i link esterni (`https://…`, `//…`), le ancore (`#…`) e `mailto:`. Restano valide per ogni storia successiva della serie.

| proprietà | con `CONTEXT_ROOT` = `/lucalorenzon-blog/`? | con `CONTEXT_ROOT` = `/`? | ref |
|---|---|---|---|
| numero di percorsi che non iniziano con la `CONTEXT_ROOT` | `0` | `0` | AC-1, AC-2 |
| numero di percorsi che contengono `//` | `0` | `0` | AC-1, AC-2 |
| elenco dei file dell'output, relativo a `site-root` | identico a quello con `/` (nessuna cartella `lucalorenzon-blog/`) | identico a quello con `/lucalorenzon-blog/` | AC-4 |

---

## Sospensione dei contenuti

> Source: Story AC-5

| `CONTENT_REPO_PATH` | esito della build? | file sotto `articles/` nell'output? | contenuto di `images/` nell'output? | ref |
|---|---|---|---|---|
| percorso inesistente (`/nonexistent`) | completata, exit code `0` | `0` | solo gli asset del sito: `article-image-not-found.svg`, `ostia_sea_top_image.jpg`, `ostia_sea_top_image.webp` | AC-5 |
| non definita | ?UNKNOWN? | — | — | AC-5 |

## Homepage "in costruzione": header e footer statici

> Source: Story AC-6. HTML della homepage generata.

| elemento | valore? | ref |
|---|---|---|
| testo principale della pagina | `Site under construction` | AC-6 |
| testo del logo nell'header | `Lvk@73r` (più la versione ASCII art) | AC-6 |
| elementi `<input id="menu">` (menu) | `0` | AC-6 |
| icone `<svg>` dentro `<header>` (ricerca, apertura menu) | `0` | AC-6 |
| testo del `<footer>` | `@ Luca Lorenzon production` | AC-6 |

## Homepage "in costruzione": nessuna hydration

> Source: Story AC-6. HTML della homepage generata con una build di produzione (non `cargo leptos watch`, che aggiunge il proprio script di auto-reload).

| elemento | numero di occorrenze? | ref |
|---|---|---|
| `<leptos-island>` | `0` | AC-6 |
| `<link rel="modulepreload">` | `0` | AC-6 |
| `<script type="module">` | `0` | AC-6 |

---

## Sito servito

> Source: Story AC-1, AC-2. Output servito da un host statico che fa corrispondere la `CONTEXT_ROOT` alla radice dell'output (come GitHub Pages).

| `CONTEXT_ROOT` | URL richiesto | stato HTTP? | richieste con stato `404` durante il caricamento della pagina? | ref |
|---|---|---|---|---|
| `/lucalorenzon-blog/` | `/lucalorenzon-blog/` | `200` (homepage "in costruzione") | `0` | AC-1 |
| `/` | `/` | `200` (homepage "in costruzione") | `0` | AC-2 |

---

## Sviluppo locale

> Source: Story AC-7

| `CONTEXT_ROOT` in `cargo leptos watch` | URL richiesto | stato HTTP? | richieste con stato `404` durante il caricamento? | ref |
|---|---|---|---|---|
| `/` (dichiarata nella configurazione di sviluppo) | `http://localhost:3000/` | `200` (homepage "in costruzione") | `0` | AC-7 |

---

## Open Issues

- **Forma del valore** (4 righe `?UNKNOWN?`): cosa fa la build con `""`,
  con il `/` finale o iniziale mancante, con un'origine `https://…`. Si può
  normalizzare o rifiutare: è una scelta da fare con l'utente in
  `/parse-dont-validate`, non da inventare qui.
- **Host statico per le righe "Sito servito"**: serve un server che faccia
  corrispondere il prefisso alla radice dell'output. Controllo manuale
  finché non c'è l'infrastruttura Playwright di [[EP-001-S010]], oppure il
  design di S006 decide di adattare il server di sviluppo (`src/main.rs`)
  per servire anche sotto prefisso.
- **`CONTENT_REPO_PATH` non definita** durante la sospensione dei contenuti:
  resta obbligatoria pur senza lettori, o no? Scelta di design (open
  question della storia), non da inventare qui.
- **CSS**: la riga globale "percorsi senza `CONTEXT_ROOT`" guarda solo
  l'HTML. Il CSS generato contiene `url("/images/ostia_sea_top_image.webp")`
  (sfondo delle pagine articolo, `src/layout.rs`): non usato dalla homepage,
  va verificato in [[EP-001-UC-001-S012]].
