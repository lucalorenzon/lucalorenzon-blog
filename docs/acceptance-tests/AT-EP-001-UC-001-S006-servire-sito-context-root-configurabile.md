# AT-EP-001-UC-001-S006: Servire il sito sotto una context root configurabile (`CONTEXT_ROOT`)

Epic: EP-001 | UC: UC-001 | Story: EP-001-UC-001-S006

Scope: sottoinsieme più fine della success guarantee "articolo raggiungibile
pubblicamente" (righe Happy path 1-5 di
[AT-UC-001](AT-UC-001-autore-pubblica-articolo.md)) ed epic AC-1, limitato a
come il sito costruisce i propri percorsi. Il deploy su GitHub Pages resta
fuori scope ([[EP-001-UC-001-S005]]), così come la pagina 404 statica.

Le righe con percorsi assoluti dalla radice in
[AT-EP-001-UC-001-S003](AT-EP-001-UC-001-S003-generare-sito-statico-ssg.md)
restano valide come caso `CONTEXT_ROOT` = `/`.

Numerazione degli AC, nell'ordine in cui compaiono nella storia:
AC-1 prefisso, AC-2 radice, AC-3 `CONTEXT_ROOT` mancante, AC-4 link nel
markdown, AC-5 islands, AC-6 sviluppo locale.

Valori di riferimento: `output-name` = `blog_start`, slug di esempio
`hello-world`, immagine propria di esempio `cover.webp`.

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

> Source: Story AC-1, AC-2. Valore dell'attributo nell'HTML di ogni pagina generata (HOME-PAGE, LISTING-PAGE, ARTICLE-PAGE).

| elemento | con `CONTEXT_ROOT` = `/lucalorenzon-blog/`? | con `CONTEXT_ROOT` = `/`? | ref |
|---|---|---|---|
| foglio di stile (`<link rel="stylesheet">` `href`) | `/lucalorenzon-blog/pkg/blog_start.css` | `/pkg/blog_start.css` | AC-1, AC-2 |
| script di hydration (`<link rel="modulepreload">` `href`) | `/lucalorenzon-blog/pkg/blog_start.js` | `/pkg/blog_start.js` | AC-1, AC-2 |
| modulo wasm (`<link rel="preload">` `href`) | `/lucalorenzon-blog/pkg/blog_start.wasm` | `/pkg/blog_start.wasm` | AC-1, AC-2 |
| favicon (`<link rel="icon">` `href`) | `/lucalorenzon-blog/favicon.ico` | `/favicon.ico` | AC-1, AC-2 |
| logo grande (`<source>` `srcset`) | `/lucalorenzon-blog/logoIcon.png` | `/logoIcon.png` | AC-1, AC-2 |
| logo piccolo (`<img>` `src`) | `/lucalorenzon-blog/logoIconSmall.png` | `/logoIconSmall.png` | AC-1, AC-2 |

## Percorsi generati: derivati dai contenuti

> Source: Story AC-1, AC-2

| elemento | con `CONTEXT_ROOT` = `/lucalorenzon-blog/`? | con `CONTEXT_ROOT` = `/`? | ref |
|---|---|---|---|
| link all'articolo in LISTING-PAGE (`<a>` `href`) | `/lucalorenzon-blog/articles/hello-world` | `/articles/hello-world` | AC-1, AC-2 |
| immagine propria dell'articolo (`<img>` `src`) | `/lucalorenzon-blog/images/cover.webp` | `/images/cover.webp` | AC-1, AC-2 |
| immagine di fallback (`<img>` `src`) | `/lucalorenzon-blog/images/article-image-not-found.svg` | `/images/article-image-not-found.svg` | AC-1, AC-2 |

## Proprietà globali dell'output

> Source: Story AC-1, AC-2. Valutate su tutti gli attributi `href`/`src`/`srcset` di tutte le pagine HTML generate, esclusi i link esterni (`https://…`, `//…`), le ancore (`#…`) e `mailto:`.

| proprietà | con `CONTEXT_ROOT` = `/lucalorenzon-blog/`? | con `CONTEXT_ROOT` = `/`? | ref |
|---|---|---|---|
| numero di percorsi che non iniziano con la `CONTEXT_ROOT` | `0` | `0` | AC-1, AC-2 |
| numero di percorsi che contengono `//` | `0` | `0` | AC-1, AC-2 |
| elenco dei file dell'output, relativo a `site-root` | identico a quello con `/` (`index.html`, `articles/<slug>.html`, `articles/page/1.html`, `pkg/…`, `images/…`, `favicon.ico`, `logoIcon.png`, `logoIconSmall.png` — nessuna cartella `lucalorenzon-blog/`) | identico a quello con `/lucalorenzon-blog/` | AC-1, AC-2 |

---

## Sito servito

> Source: Story AC-1, AC-2. Output servito da un host statico che fa corrispondere la `CONTEXT_ROOT` alla radice dell'output (come GitHub Pages), con risoluzione degli URL senza estensione (`/articles/hello-world` → `articles/hello-world.html`).

| `CONTEXT_ROOT` | URL richiesto | stato HTTP? | richieste con stato `404` durante il caricamento della pagina? | ref |
|---|---|---|---|---|
| `/lucalorenzon-blog/` | `/lucalorenzon-blog/` | `200` (HOME-PAGE) | `0` | AC-1 |
| `/lucalorenzon-blog/` | `/lucalorenzon-blog/articles/hello-world` | `200` (ARTICLE-PAGE) | `0` | AC-1 |
| `/lucalorenzon-blog/` | `/lucalorenzon-blog/articles/page/1` | `200` (LISTING-PAGE) | `0` | AC-1 |
| `/` | `/` | `200` (HOME-PAGE) | `0` | AC-2 |
| `/` | `/articles/hello-world` | `200` (ARTICLE-PAGE) | `0` | AC-2 |
| `/` | `/articles/page/1` | `200` (LISTING-PAGE) | `0` | AC-2 |

---

## Link e immagini scritti dall'autore nel markdown: riscritti

> Source: Story AC-4. Valore dell'attributo nell'HTML del corpo dell'ARTICLE-PAGE.

| markdown nel corpo | `CONTEXT_ROOT` | attributo generato? | ref |
|---|---|---|---|
| `[altro](/articles/altro)` | `/lucalorenzon-blog/` | `href="/lucalorenzon-blog/articles/altro"` | AC-4 |
| `![x](/images/x.webp)` | `/lucalorenzon-blog/` | `src="/lucalorenzon-blog/images/x.webp"` | AC-4 |
| `[altro](/articles/altro)` | `/` | `href="/articles/altro"` | AC-4 |

## Link e immagini scritti dall'autore nel markdown: non riscritti

> Source: Story AC-4 + Decisions Log (si riscrivono solo i percorsi dalla radice; limite accettato: HTML grezzo)

| markdown nel corpo | `CONTEXT_ROOT` | attributo generato? | ref |
|---|---|---|---|
| `[ext](https://example.com/p)` | `/lucalorenzon-blog/` | `href="https://example.com/p"` | AC-4 |
| `[cdn](//cdn.example.com/x)` | `/lucalorenzon-blog/` | `href="//cdn.example.com/x"` | AC-4 |
| `[sez](#sezione)` | `/lucalorenzon-blog/` | `href="#sezione"` | AC-4 |
| `[mail](mailto:a@example.com)` | `/lucalorenzon-blog/` | `href="mailto:a@example.com"` | AC-4 |
| `[rel](altro)` | `/lucalorenzon-blog/` | `href="altro"` | AC-4 |
| `<a href="/articles/altro">raw</a>` (HTML grezzo) | `/lucalorenzon-blog/` | `href="/articles/altro"` | AC-4 |

---

## Islands sotto prefisso

> Source: Story AC-5. Browser reale, sito servito con `CONTEXT_ROOT` = `/lucalorenzon-blog/`, pagina `/lucalorenzon-blog/`.

| azione del visitatore | `<html>` ha la classe `dark`? | testo del selettore tema? | `<header>` ha la classe `bg-white`? | ref |
|---|---|---|---|---|
| pagina appena caricata, in cima | `true` | `Enable light mode` | `false` | AC-5 |
| clic sul selettore tema | `false` | `Enable dark mode` | `false` | AC-5 |
| secondo clic sul selettore tema | `true` | `Enable light mode` | `false` | AC-5 |
| scroll oltre la cima della pagina | `true` | `Enable light mode` | `true` | AC-5 |

---

## Sviluppo locale

> Source: Story AC-6, AC-3

| `CONTEXT_ROOT` in `cargo leptos watch` | URL richiesto | stato HTTP? | richieste con stato `404` durante il caricamento? | ref |
|---|---|---|---|---|
| `/` (dichiarata nella configurazione di sviluppo) | `http://localhost:3000/` | `200` (HOME-PAGE) | `0` | AC-6 |
| `/` (dichiarata nella configurazione di sviluppo) | `http://localhost:3000/articles/hello-world` | `200` (ARTICLE-PAGE) | `0` | AC-6 |
| non definita | — | build interrotta, il messaggio d'errore contiene `CONTEXT_ROOT` | — | AC-3, AC-6 |

---

## Open Issues

- **Forma del valore** (4 righe `?UNKNOWN?`): cosa fa la build con `""`,
  con il `/` finale o iniziale mancante, con un'origine `https://…`. Si può
  normalizzare o rifiutare: è una scelta da fare con l'utente in
  `/parse-dont-validate`, non da inventare qui.
- **Link relativi nel markdown** (`[rel](altro)`): restano invariati per la
  regola concordata (si riscrivono solo i percorsi dalla radice), ma il
  browser li risolve rispetto all'URL della pagina
  (`/lucalorenzon-blog/articles/` per un articolo). La riga fissa il
  comportamento; se sia accettabile per l'autore, o vada vietato o
  segnalato (es. da [[EP-001-UC-001-S007]] o [[EP-001-UC-001-S009]]), è da
  decidere fuori da questa storia.
- **Host statico per le righe "Sito servito" e "Islands"**: servono un
  server che faccia corrispondere il prefisso alla radice dell'output e un
  browser reale. Restano controlli manuali finché non c'è l'infrastruttura
  Playwright di [[EP-001-S010]], oppure il design di S006 decide di
  adattare il server di sviluppo (`src/main.rs`) per servire anche sotto
  prefisso.
- **Righe dell'header island**: dipendono dal fatto che la pagina sia
  abbastanza lunga da scorrere; nel test va fissata un'altezza del
  viewport minore della pagina.
