# UC-001: Autore pubblica un nuovo articolo

| Field | Value |
|---|---|
| **Epic** | EP-001 — Rilancio del blog personale come presenza professionale |
| **Goal level** | ⚡ User-Goal |
| **Scope** | Sistema di pubblicazione del blog (repo contenuti dedicato + sito statico generato) |
| **Primary actor** | Autore/Editore |
| **Preconditions** | L'autore ha accesso in scrittura al repo contenuti dedicato; il sito è già distribuito da una build precedente |
| **Success guarantee** | L'articolo è raggiungibile pubblicamente sul sito generato (ARTICLE-PAGE), con contenuto e metadati (data, slug, tag) corretti, ed elencato nella LISTING-PAGE/HOME-PAGE pertinente |
| **Trigger** | L'autore vuole pubblicare un nuovo articolo; la build è avviata automaticamente da CI (GitHub Actions) al push sul repo contenuti, non da un comando manuale dell'autore |

## Main Success Scenario

1. Autore scrive un file markdown con frontespizio nel repo contenuti dedicato: campi obbligatori data, slug, uno o più tag, titolo; campi opzionali abstract e immagine di sintesi (calcolati automaticamente se assenti, vedi passo 3)
2. Autore fa push del file sul repo contenuti
3. Sistema rileva la nuova pubblicazione e genera il sito statico aggiornato (ARTICLE-PAGE del nuovo articolo, LISTING-PAGE e HOME-PAGE aggiornate); se abstract o immagine di sintesi sono assenti, il sistema li calcola automaticamente (abstract dal corpo dell'articolo, immagine di sintesi con un'immagine di fallback predefinita)
4. Sistema distribuisce l'output generato
5. Sistema rende l'articolo raggiungibile dal Visitatore tramite la ARTICLE-PAGE e la LISTING-PAGE/HOME-PAGE

<!-- Verb duration check: scrive, fa push, rileva/genera, distribuisce, rende raggiungibile — tutti eventi puntuali, stessa granularità -->

## Extensions (alternative and failure paths)

**2a. Push respinto (conflitto, permessi mancanti):**
  1. Repo respinge il push
  2. Autore risolve il conflitto o ottiene i permessi necessari, poi ripete il passo 2

**3a. Frontespizio mancante o malformato (data, slug, tag o titolo non validi — abstract e immagine di sintesi non rientrano qui: sono opzionali, vedi passo 3):**
  1. CI (GitHub Actions), avviata dal push, rileva l'errore e segnala all'autore tramite check fallito, con dettaglio nel log del check, più email di notifica standard di GitHub per il check fallito
  2. La pubblicazione non procede; l'autore corregge il frontespizio e ripete il push (torna al passo 2)

**3b. Slug duplicato (già usato da un articolo esistente):**
  1. Sistema segnala il conflitto di slug tramite check fallito sul push in GitHub Actions, con dettaglio dell'errore nel log del check, più email di notifica standard di GitHub per il check fallito
  2. Autore corregge lo slug e ripete la pubblicazione

**3c. Build fallisce (errore nel processo di generazione del sito):**
  1. Sistema segnala l'errore di build tramite check fallito sul push in GitHub Actions, con dettaglio dell'errore nel log del check, più email di notifica standard di GitHub per il check fallito
  2. L'articolo non viene pubblicato; il sito precedentemente distribuito resta invariato

**4a. Distribuzione fallisce:**
  1. Sistema mantiene attiva l'ultima versione distribuita con successo
  2. Autore viene informato dell'esito tramite check fallito sul push in GitHub Actions (stesso canale di 3a/3b/3c, essendo il deploy uno step della stessa pipeline) e può ripetere la pubblicazione

<!-- Each extension = one acceptance test case -->

## Open Issues

- **Risolto (2026-08-21, discussione Decision di [EP-001-UC-001-S001](../stories/EP-001-UC-001-S001-modellare-articolo-dominio-content-source.md)):** obbligatorietà dei campi del frontespizio non era specificata a livello di UC. Decisione: data/slug/tag/titolo obbligatori (rifiuto se mancanti o malformati, estensione 3a); abstract e immagine di sintesi opzionali, calcolati automaticamente in fase di generazione se assenti (passo 3 del main scenario) — non generano un errore. `tag` è una lista di uno o più valori, non un valore singolo.
- ~~Meccanismo di trigger della build (CI su push vs. comando manuale dell'autore) non deciso qui...~~ — **risolto** (2026-08-21): CI automatico su push (GitHub Actions), coerente con AC-2 dell'epic ("pushando...senza modificare codice applicativo") — nessun comando manuale dell'autore. Vedi campo Trigger. **Conseguenza:** la validazione del frontespizio (ex-estensione 1a) è rilevata da CI *dopo* il push, non prima — l'estensione è stata rinumerata da 1a a 3a (e le successive 3a/3b spostate a 3b/3c) per riflettere il punto reale in cui la deviazione viene osservata nello scenario principale.
- ~~Canale/formato con cui l'autore vede errori di validazione o di build (passi 1a, 3b) non specificato in questa UC~~ — **risolto** (2026-08-21): check fallito su GitHub Actions con dettaglio nel log + email di notifica standard di GitHub. Applicato anche a 3b (ex-3a) e 4a per coerenza, essendo validazione/build/deploy step della stessa pipeline.

## Acceptance Tests
[AT-UC-001-autore-pubblica-articolo](../acceptance-tests/AT-UC-001-autore-pubblica-articolo.md)

## Stories
| Story ID | Title | Status |
|---|---|---|
| EP-001-UC-001-S001 | Modellare l'articolo e la porta ContentSource | AT defined (2026-08-21) — pending sizing |
| EP-001-UC-001-S002 | Verificare l'unicità dello slug rispetto agli articoli già pubblicati | Pending discussion |
| EP-001-UC-001-S003 | Generare il sito statico da un Article (ARTICLE-PAGE, LISTING-PAGE, HOME-PAGE) | Pending discussion |
| EP-001-UC-001-S004 | Automatizzare la pipeline CI: trigger su push e segnalazione errori | Pending discussion |
| EP-001-UC-001-S005 | Distribuire l'output generato e gestire il fallimento di deploy | Pending discussion |

**Estensione 2a (push respinto per conflitto o permessi mancanti) non ha una story dedicata**: è comportamento nativo di git/GitHub (il repo respinge il push, l'autore risolve conflitto/permessi fuori dal sistema che stiamo costruendo) — nessuno sviluppo applicativo la copre. Segnalato qui per non assorbirla silenziosamente in un'altra story (Step 7 di `sw-story-split`).

## Sequence Diagram

```mermaid
sequenceDiagram
    actor Autore as Autore/Editore
    participant Repo as Repo Contenuti
    participant System as Sito (build + deploy)
    actor Visitatore

    Autore->>Repo: push articolo (markdown + frontespizio)
    Repo->>System: CI (GitHub Actions) avviata automaticamente sul push
    alt frontespizio non valido
        System-->>Autore: check fallito su GitHub Actions + email di notifica (errore di validazione)
    else push accettato
        System->>System: genera output statico aggiornato
        alt build fallisce
            System-->>Autore: check fallito su GitHub Actions + email di notifica (errore di build)
        else build riuscita
            System->>System: distribuisce l'output generato
            alt distribuzione fallisce
                System-->>Autore: check fallito su GitHub Actions + email di notifica (errore di deploy); ultima versione distribuita resta attiva
            else distribuzione riuscita
                System-->>Visitatore: articolo raggiungibile (ARTICLE-PAGE, LISTING-PAGE/HOME-PAGE)
            end
        end
    end
```
