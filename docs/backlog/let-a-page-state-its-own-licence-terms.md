# Let a page state its own licence terms

- **Opened:** 2026-11-08, by the user, while reviewing the code of the
  card on the articles' front matter
  ([card](../wip/read-article-fields-from-front-matter.md)).
- **Status:** noticed, not started.

## Why

The licence page (`content/licence.md`) holds the licences the author
manages: the articles under CC BY 4.0, their code samples under MIT-0,
the curriculum under CC BY-ND 4.0. Its first paragraph already says
"Unless a page says otherwise": a page may depart from those licences,
towards more restrictive terms or less restrictive ones.

Today a page says otherwise in two ways, neither of which the build
knows as data:

- **in the code,** the licence line of the footer, chosen page by page
  in `FooterLicence` (`src/page_frame.rs`), which knows two phrases:
  "Reuse with attribution" for everything but the curriculum, and the
  curriculum's "This CV may be copied, not changed";
- **in the text,** a closing note: both articles on SAP Commerce end
  saying that the interfaces, classes and configuration quoted from SAP
  Commerce belong to SAP, and that MIT-0 covers only the author's code
  samples ([card](../done/2026-11-03_add-after-save-event-markdown.md),
  "The licence of that code";
  [card](../done/2026-11-03_add-generate-pk-article-markdown.md)). The
  licence page does not make that distinction.

The user first thought of an optional field in the front matter, for
articles too, and then judged it more complex than a single field: what
a page says otherwise may be a distinction within the page, as in the
SAP articles, not one licence in place of another.

## Known from the start

- **Without the field, the licences of the licence page apply,** as
  today.
- **The licence is already written in three places:** `content/README.md`,
  `content/licence.md` and the code's `FooterLicence`. A field must not
  become a fourth source that can contradict them.
- **The footer shows a short phrase per licence;** a licence the build
  does not know has no phrase to show.
- **It concerns two models:** the articles' (`Article`) and the
  information pages', to be unified in a
  [card of its own](unify-the-information-pages-model.md), which
  probably comes first.

## Questions

- What a page states: one licence in place of the default, terms per
  kind of material (text, code samples, material by others), or a free
  note.
- Whether the licences a page may choose are a list the site knows,
  which grows when a page needs one more, or anything the author writes.
- Whether the SAP articles' closing note stays in the text or is
  written by the build from the front matter.
- Whether the licence page lists the pages that depart from it.
