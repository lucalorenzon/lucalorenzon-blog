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

> Source: UC-001 ext. 3a

| frontmatter | campo invalido | push tentato? | CI (GitHub Actions) avviata? | esito pubblicazione? | messaggio errore? | ref |
|---|---|---|---|---|---|---|
| data mancante o malformata | data | true | true (fallisce nel check di validazione) | non pubblicato | check fallito su GitHub Actions (dettaglio nel log) + email di notifica standard GitHub | 3a |
| slug mancante o malformato | slug | true | true (fallisce nel check di validazione) | non pubblicato | check fallito su GitHub Actions (dettaglio nel log) + email di notifica standard GitHub | 3a |
| tag mancante o malformato | tag | true | true (fallisce nel check di validazione) | non pubblicato | check fallito su GitHub Actions (dettaglio nel log) + email di notifica standard GitHub | 3a |

---

## Extensions — push respinto

> Source: UC-001 ext. 2a

| frontmatter | condizione push | push esito? | stato pubblicazione? | ref |
|---|---|---|---|---|
| valido | conflitto sul repo contenuti | respinto | non pubblicato; autore deve risolvere il conflitto e ripetere il push | 2a |
| valido | permessi di scrittura mancanti | respinto | non pubblicato; autore deve ottenere i permessi e ripetere il push | 2a |

---

## Extensions — slug duplicato

> Source: UC-001 ext. 3b

| frontmatter | slug | slug già esistente? | esito pubblicazione? | messaggio errore? | ref |
|---|---|---|---|---|---|
| valido altrimenti | uguale a uno slug di un articolo già pubblicato | true | non pubblicato | check fallito su GitHub Actions (dettaglio nel log) + email di notifica standard GitHub | 3b |

---

## Extensions — build fallisce

> Source: UC-001 ext. 3c

| frontmatter | slug | build esito | ARTICLE-PAGE(slug) pubblicata? | sito precedentemente distribuito? | messaggio errore? | ref |
|---|---|---|---|---|---|---|
| valido | univoco | fallita | false | invariato (resta la versione distribuita prima del tentativo) | check fallito su GitHub Actions (dettaglio nel log) + email di notifica standard GitHub | 3c |

---

## Extensions — distribuzione fallisce

> Source: UC-001 ext. 4a

| frontmatter | slug | build esito | deploy esito | sito servito al Visitatore? | autore informato dell'esito? | ref |
|---|---|---|---|---|---|---|
| valido | univoco | riuscita | fallita | ultima versione distribuita con successo (non la nuova build) | check fallito su GitHub Actions (dettaglio nel log) + email di notifica standard GitHub — esito comunicato e autore può ripetere la pubblicazione | 4a |

---

## Open Issues

- ~~Tutte le celle `?UNKNOWN?` dipendono dai due Open Issues già registrati in UC-001, non ancora decisi...~~ — **risolto** (2026-08-21), coerentemente con la risoluzione degli Open Issues in UC-001:
  - **Canale/formato errori**: check fallito su GitHub Actions (dettaglio nel log) + email di notifica standard GitHub — applicato a tutte le righe ex-`?UNKNOWN?` (ora 3a, 3b, 3c, 4a).
  - **Meccanismo di trigger della build**: CI automatico su push (GitHub Actions), non comando manuale. **Conseguenza sulla tabella:** la sezione "frontespizio mancante o malformato" presupponeva `push tentato? false` (validazione prima del push) — corretto a `true`, con colonna aggiuntiva `CI avviata?`, poiché con CI-on-push la validazione avviene *dopo* il push, dentro la pipeline (vedi UC-001 ext. 3a, rinumerata da 1a).
- Riga 3b (slug duplicato, ex-3a): non è specificato in UC-001 se il controllo di unicità avviene in uno step CI dedicato o durante il build stesso — la tabella assume solo l'esito osservabile (pubblicazione non avviene), non il punto esatto del fallimento all'interno della pipeline.
