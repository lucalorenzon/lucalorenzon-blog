# AT-UC-001: Autore pubblica un nuovo articolo

Epic: EP-001 | UC: UC-001

---

## Happy path

> Source: UC-001 main scenario, passi 1-5

| frontmatter | push esito | build esito | deploy esito | ARTICLE-PAGE(slug)? | LISTING-PAGE/HOME-PAGE? | ref |
|---|---|---|---|---|---|---|
| valido (data, slug univoco, tag, titolo, abstract, immagine di sintesi tutti presenti e ben formati) | accettato | riuscita | riuscita | `{ reachable: true, metadata: <frontmatter>, content: <corpo markdown> }` | include voce per lo slug pubblicato | 1-5 |

---

## Extensions — frontespizio mancante o malformato

> Source: UC-001 ext. 1a

| frontmatter | campo invalido | push tentato? | esito pubblicazione? | messaggio errore? | ref |
|---|---|---|---|---|---|
| data mancante o malformata | data | false | non pubblicato | `?UNKNOWN?` | 1a |
| slug mancante o malformato | slug | false | non pubblicato | `?UNKNOWN?` | 1a |
| tag mancante o malformato | tag | false | non pubblicato | `?UNKNOWN?` | 1a |

---

## Extensions — push respinto

> Source: UC-001 ext. 2a

| frontmatter | condizione push | push esito? | stato pubblicazione? | ref |
|---|---|---|---|---|
| valido | conflitto sul repo contenuti | respinto | non pubblicato; autore deve risolvere il conflitto e ripetere il push | 2a |
| valido | permessi di scrittura mancanti | respinto | non pubblicato; autore deve ottenere i permessi e ripetere il push | 2a |

---

## Extensions — slug duplicato

> Source: UC-001 ext. 3a

| frontmatter | slug | slug già esistente? | esito pubblicazione? | messaggio errore? | ref |
|---|---|---|---|---|---|
| valido altrimenti | uguale a uno slug di un articolo già pubblicato | true | non pubblicato | `?UNKNOWN?` | 3a |

---

## Extensions — build fallisce

> Source: UC-001 ext. 3b

| frontmatter | slug | build esito | ARTICLE-PAGE(slug) pubblicata? | sito precedentemente distribuito? | messaggio errore? | ref |
|---|---|---|---|---|---|---|
| valido | univoco | fallita | false | invariato (resta la versione distribuita prima del tentativo) | `?UNKNOWN?` | 3b |

---

## Extensions — distribuzione fallisce

> Source: UC-001 ext. 4a

| frontmatter | slug | build esito | deploy esito | sito servito al Visitatore? | autore informato dell'esito? | ref |
|---|---|---|---|---|---|---|
| valido | univoco | riuscita | fallita | ultima versione distribuita con successo (non la nuova build) | `?UNKNOWN?` (canale non specificato) — ma esito comunicato e autore può ripetere la pubblicazione | 4a |

---

## Open Issues

- Tutte le celle `?UNKNOWN?` dipendono dai due Open Issues già registrati in UC-001, non ancora decisi:
  - **Canale/formato con cui l'autore vede errori di validazione o di build** (righe 1a, 3a, 3b, 4a) — demandato alla progettazione tecnica della story che implementerà la validazione/build/deploy.
  - **Meccanismo di trigger della build** (CI su push vs. comando manuale) — non influisce sui valori attesi di queste righe, ma condiziona come "push accettato" si traduce operativamente in "build avviata".
- Riga 3a (slug duplicato): non è specificato in UC-001 se il controllo di unicità avviene prima o durante la build — la tabella assume solo l'esito osservabile (pubblicazione non avviene), non il punto esatto del fallimento.
