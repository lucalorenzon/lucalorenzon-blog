//! The fonts page of the style guide: the text of `content/style-guide/fonts.md`, and after each
//! of its sections the samples of that font, one for each file the site serves.
//!
//! The fonts, their names, their roles and their samples come from the configuration
//! (`config/`, read by `fonts.rs`). The author's text is cut in pieces, each opened by a
//! placeholder heading `## <folder> | <role>`; the build composes the page: one section for each
//! distinct font, with its family name, its roles, the pieces of its roles and its samples, and
//! the font's files give the samples, each labelled with its `name`. Every sample shows the same
//! text, so that only the font changes from one to the next. A sample never names a font: it carries a
//! variant the build writes in `target/generated/fonts.css` (`sample-<role>-<style>-<weight>`),
//! which takes the font from the role's variable, so a change of font in the configuration
//! shows here too, and a file added to it gets its sample with nothing written by hand.

use super::FOLDER;
use super::navigation::GuideBody;
use crate::markdown;
use crate::page_frame::{FooterLicence, Heading, IndexedPage, Indexing, PageFrame, PageKind};
use crate::site_address::SiteAddress;
use crate::style_guide::fonts::{Fonts, Role, ShownFont};
use leptos::prelude::*;
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};

/// The page's file in the guide's folder, as the index links to it.
pub const FILE: &str = "fonts.html";

/// The text of the page, written in Markdown: an introduction, then one section for each font.
pub(crate) const FONTS: &str = include_str!("../../content/style-guide/fonts.md");

/// What search engines and link previews show of the page.
const DESCRIPTION: &str =
    "The fonts of this site, each in every style it serves, and why each one was chosen.";

/// The text every sample shows: English, with every letter and every digit.
const SAMPLE_TEXT: &str =
    "The quick brown fox jumps over the lazy dog. Sphinx of black quartz, judge my vow. 0123456789";

/// Why the page's text does not match the fonts. The build stops with it: the text and the
/// fonts must say the same thing.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SectionError {
    #[error(
        "content/style-guide/fonts.md: the heading `## {0}` is not in the form `## <font> | <role>`, \
         with the font and the role written in small letters, digits and hyphens"
    )]
    NotAPlaceholder(String),
    #[error("content/style-guide/fonts.md: the heading `## {0}` appears more than once")]
    Duplicate(String),
    #[error(
        "content/style-guide/fonts.md: the font `{folder}` in the role `{role}` has no piece (write a heading `## {folder} | {role}`)"
    )]
    MissingPiece { folder: String, role: String },
}

/// A piece of the author's text: what follows one placeholder heading, up to the next level-2
/// heading or the end.
struct Piece {
    folder: String,
    role: String,
    markdown: String,
}

/// Whether a part of a placeholder is made only of small letters, digits and hyphens.
fn is_plain(part: &str) -> bool {
    !part.is_empty()
        && part
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// Cuts the text at its level-2 headings, as the converter sees them (the Markdown parser's
/// own, so an indented heading, an underlined one or one inside a code block are told apart
/// exactly as the page will render them): the introduction before the first one, then one piece
/// for each heading, which must be a placeholder `## <folder> | <role>`.
fn cut(markdown: &str) -> Result<(String, Vec<Piece>), SectionError> {
    let mut headings: Vec<(std::ops::Range<usize>, String)> = Vec::new();
    let mut open: Option<(std::ops::Range<usize>, String)> = None;
    for (event, range) in Parser::new_ext(markdown, markdown::PARSER_OPTIONS).into_offset_iter() {
        match event {
            Event::Start(Tag::Heading {
                level: HeadingLevel::H2,
                ..
            }) => open = Some((range, String::new())),
            Event::Text(text) | Event::Code(text) => {
                if let Some((_, title)) = open.as_mut() {
                    title.push_str(&text);
                }
            }
            Event::End(TagEnd::Heading(HeadingLevel::H2)) => {
                headings.extend(open.take());
            }
            _ => {}
        }
    }
    let introduction_end = headings.first().map_or(markdown.len(), |h| h.0.start);
    let introduction = markdown[..introduction_end].to_owned();
    let mut pieces: Vec<Piece> = Vec::new();
    for (at, (range, title)) in headings.iter().enumerate() {
        let heading = title.trim();
        let (folder, role) = heading
            .split_once(" | ")
            .filter(|(folder, role)| is_plain(folder) && is_plain(role))
            .ok_or_else(|| SectionError::NotAPlaceholder(heading.to_owned()))?;
        if pieces
            .iter()
            .any(|piece| piece.folder == folder && piece.role == role)
        {
            return Err(SectionError::Duplicate(heading.to_owned()));
        }
        let end = headings.get(at + 1).map_or(markdown.len(), |h| h.0.start);
        pieces.push(Piece {
            folder: folder.to_owned(),
            role: role.to_owned(),
            markdown: markdown[range.end..end].to_owned(),
        });
    }
    Ok((introduction, pieces))
}

/// The samples of one font: for each file, the text drawn with it and, under it as a caption,
/// the file's name (above the text it would read as the caption of the paragraph before).
#[component]
fn Samples(font: ShownFont) -> impl IntoView {
    font.samples
        .into_iter()
        .map(|sample| {
            view! {
                <p class=sample.class>{SAMPLE_TEXT}</p>
                <div class="sample-label">{sample.name}</div>
            }
        })
        .collect_view()
}

/// The start of a font's section: its title, then the line that names the roles it plays.
#[component]
fn SectionStart(family: String, roles: String) -> impl IntoView {
    view! {
        <h2>{family}</h2>
        <p class="roles-line">{roles}</p>
    }
}

/// "Role: text" or "Roles: code, logo", in the roles' order.
fn roles_line(roles: &[Role]) -> String {
    let names: Vec<&str> = roles.iter().map(|role| role.name()).collect();
    let label = if names.len() == 1 { "Role" } else { "Roles" };
    format!("{label}: {}", names.join(", "))
}

/// The page's main content: the introduction, then one section for each distinct font, in the
/// order of its first role: its title, its roles, the author's piece for each role, its samples.
/// A piece whose font and role the configuration does not hold is dropped.
fn body(markdown: &str, fonts: &Fonts) -> Result<String, SectionError> {
    let (introduction, pieces) = cut(markdown)?;
    let shown = fonts.shown();
    for font in &shown {
        for role in &font.roles {
            if !pieces
                .iter()
                .any(|piece| piece.folder == font.folder && piece.role == role.name())
            {
                return Err(SectionError::MissingPiece {
                    folder: font.folder.clone(),
                    role: role.name().to_owned(),
                });
            }
        }
    }
    let owner = Owner::new();
    let mut out = markdown::to_html(&introduction);
    for font in shown {
        out.push_str(&owner.with(|| {
            view! { <SectionStart family=font.family.clone() roles=roles_line(&font.roles) /> }
                .to_html()
        }));
        for role in &font.roles {
            if let Some(piece) = pieces
                .iter()
                .find(|piece| piece.folder == font.folder && piece.role == role.name())
            {
                out.push_str(&markdown::to_html(&piece.markdown));
            }
        }
        out.push_str(&owner.with(|| view! { <Samples font=font.clone() /> }.to_html()));
    }
    Ok(out)
}

#[component]
pub fn FontsPage(canonical: String, body: String) -> impl IntoView {
    view! {
        <PageFrame
            title="Fonts - Style guide - Luca Lorenzon"
            heading=Heading::try_from("Fonts").expect("the fonts page's heading fits the frame")
            address_prefix="../"
            footer_licence=FooterLicence::ReuseWithAttribution
            indexing=Indexing::Indexed(IndexedPage {
                canonical,
                description: DESCRIPTION.to_owned(),
                kind: PageKind::Website,
            })
        >
            <GuideBody title="Fonts" body=body />
        </PageFrame>
    }
}

/// Renders the fonts page to a complete HTML document string, or says why the text does not
/// match the fonts.
pub fn render_fonts_page(site: &SiteAddress, fonts: &Fonts) -> Result<String, SectionError> {
    let body = body(FONTS, fonts)?;
    let owner = Owner::new();
    let canonical = site.join(&format!("{FOLDER}{FILE}"));
    let page = owner.with(|| view! { <FontsPage canonical=canonical body=body /> }.to_html());
    Ok(format!("<!DOCTYPE html>{page}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn site() -> SiteAddress {
        SiteAddress::parse("https://www.lucalorenzon.com/").unwrap()
    }

    fn fonts() -> Fonts {
        crate::style_guide::fonts::real()
    }

    fn render_fonts_page() -> String {
        super::render_fonts_page(&site(), &fonts()).unwrap()
    }

    /// A text with an introduction and one piece for each placeholder heading given.
    fn text_with(headings: &[&str]) -> String {
        let mut text = String::from("Introduction.\n");
        for heading in headings {
            text.push_str(&format!("\n## {heading}\n\nWhy {heading}.\n"));
        }
        text
    }

    const ALL: [&str; 3] = ["literata | text", "hack | code", "literata | logo"];

    /// Acceptance test 4: a pair of the configuration with no piece stops the build.
    #[test]
    fn a_pair_without_a_piece_is_an_error() {
        let error = body(
            &text_with(&["literata | text", "literata | logo"]),
            &fonts(),
        )
        .unwrap_err();
        assert_eq!(
            error,
            SectionError::MissingPiece {
                folder: "hack".to_owned(),
                role: "code".to_owned()
            }
        );
        assert_eq!(
            error.to_string(),
            "content/style-guide/fonts.md: the font `hack` in the role `code` has no piece (write a heading `## hack | code`)"
        );
    }

    /// Acceptance test 4: a pair written twice stops the build, held by the configuration or not.
    #[test]
    fn a_pair_written_twice_is_an_error() {
        for twice in ["hack | code", "meslo-lg | code"] {
            let mut headings = ALL.to_vec();
            headings.push("meslo-lg | code");
            headings.push(twice);
            let error = body(&text_with(&headings), &fonts()).unwrap_err();
            assert_eq!(error, SectionError::Duplicate(twice.to_owned()));
            assert_eq!(
                error.to_string(),
                format!(
                    "content/style-guide/fonts.md: the heading `## {twice}` appears more than once"
                )
            );
        }
    }

    /// Acceptance test 4: a level-2 heading that is not a placeholder stops the build.
    #[test]
    fn a_heading_that_is_not_a_placeholder_is_an_error() {
        for heading in [
            "Literata",
            "literata|text",
            "literata | ",
            " | text",
            "Literata | text",
            "literata | Text",
            "literata | text | code",
            "lite rata | text",
            "literata | te_xt",
        ] {
            let mut headings = ALL.to_vec();
            headings.push(heading);
            let error = body(&text_with(&headings), &fonts()).unwrap_err();
            assert_eq!(
                error,
                SectionError::NotAPlaceholder(heading.trim().to_owned())
            );
        }
        let error = body("## Literata\n\nText.\n", &fonts()).unwrap_err();
        assert_eq!(
            error.to_string(),
            "content/style-guide/fonts.md: the heading `## Literata` is not in the form `## <font> | <role>`, with the font and the role written in small letters, digits and hyphens"
        );
    }

    /// Acceptance test 5: a well-formed placeholder whose pair the configuration does not hold is
    /// dropped, without an error.
    #[test]
    fn a_piece_whose_pair_the_configuration_does_not_hold_is_ignored() {
        let mut headings = ALL.to_vec();
        headings.push("meslo-lg | code");
        headings.push("literata | heading");
        let html = body(&text_with(&headings), &fonts()).unwrap();
        assert!(!html.contains("Why meslo-lg"));
        assert!(!html.contains("Why literata | heading"));
        assert!(html.contains("Why literata | text."));
        assert!(html.contains("Why literata | logo."));
        assert_eq!(html.matches("<h2>").count(), 2);
    }

    /// Acceptance test 4: an indented level-2 heading is still a heading for the converter.
    #[test]
    fn an_indented_heading_that_is_not_a_placeholder_is_an_error() {
        let mut text = text_with(&ALL);
        text.push_str("\n   ## Notes\n\nText.\n");
        assert_eq!(
            body(&text, &fonts()).unwrap_err(),
            SectionError::NotAPlaceholder("Notes".to_owned())
        );
    }

    /// Acceptance test 4: so is a heading underlined with hyphens.
    #[test]
    fn an_underlined_heading_that_is_not_a_placeholder_is_an_error() {
        let mut text = text_with(&ALL);
        text.push_str("\nNotes\n-----\n\nText.\n");
        assert_eq!(
            body(&text, &fonts()).unwrap_err(),
            SectionError::NotAPlaceholder("Notes".to_owned())
        );
    }

    /// Acceptance test 3: a heading inside a longer fence is not a placeholder either.
    #[test]
    fn a_heading_inside_a_four_backtick_fence_is_not_cut() {
        let mut text = text_with(&ALL);
        text.push_str("\n````\n```\n## not a placeholder\n```\n````\n");
        let html = body(&text, &fonts()).unwrap();
        assert!(html.contains("## not a placeholder"));
        assert_eq!(html.matches("<h2>").count(), 2);
    }

    /// Acceptance test 3: a heading in a fenced code block is not a placeholder.
    #[test]
    fn a_heading_inside_a_code_block_is_not_cut() {
        let mut text = text_with(&ALL);
        text.push_str("\n```\n## not a placeholder\n```\n");
        let html = body(&text, &fonts()).unwrap();
        assert!(html.contains("## not a placeholder"));
    }

    #[test]
    fn the_real_text_matches_the_fonts() {
        assert!(body(FONTS, &fonts()).is_ok());
    }

    /// The page's main content, as rendered: the introduction, then each section with its
    /// samples.
    fn main_html() -> String {
        body(FONTS, &fonts()).unwrap()
    }

    #[test]
    fn the_page_holds_the_introduction_and_one_section_for_each_font() {
        let html = main_html();
        assert!(
            html.starts_with(
                "<p>These are the fonts this site uses, each in every style it serves."
            )
        );
        assert_eq!(html.matches("<h2>").count(), 2);
        let literata = html.find("<h2>Literata</h2>").unwrap();
        let hack = html.find("<h2>Hack</h2>").unwrap();
        assert!(0 < literata && literata < hack);
    }

    /// Acceptance tests 1 and 2: under each title, the roles' line.
    #[test]
    fn each_title_is_followed_by_the_roles_line() {
        let html = main_html();
        for (title, line) in [("Literata", "Roles: text, logo"), ("Hack", "Role: code")] {
            assert!(
                html.contains(&format!(
                    "<h2>{title}</h2><p class=\"roles-line\">{line}</p>"
                )),
                "{title}"
            );
        }
    }

    /// A configuration where the code and the logo share a font, Hack.
    fn shared_fonts() -> Fonts {
        let root = env!("CARGO_MANIFEST_DIR");
        crate::style_guide::fonts::load(
            "text=literata\ncode=hack\nlogo=hack\n",
            &|folder| std::fs::read_to_string(format!("{root}/config/fonts/{folder}.txt")).ok(),
            &|_, _| true,
        )
        .unwrap()
    }

    /// Acceptance tests 1, 2 and 3: a font with two roles has one section, at the place of its
    /// first role, naming both roles and holding both pieces in the roles' order.
    #[test]
    fn a_font_with_two_roles_has_one_section_with_both_pieces() {
        let text = "Introduction.\n\n## hack | logo\n\nLogo piece.\n\n\
             ## literata | text\n\nText piece.\n\n## hack | code\n\nCode piece.\n";
        let html = body(text, &shared_fonts()).unwrap();
        assert!(html.starts_with("<p>Introduction.</p>"));
        assert_eq!(html.matches("<h2>").count(), 2);
        let literata = html.find("<h2>Literata</h2>").unwrap();
        let hack = html.find("<h2>Hack</h2>").unwrap();
        assert!(literata < hack);
        assert!(html.contains("<p class=\"roles-line\">Role: text</p>"));
        assert!(html.contains("<p class=\"roles-line\">Roles: code, logo</p>"));
        let section = &html[hack..];
        let code = section.find("Code piece.").unwrap();
        let logo = section.find("Logo piece.").unwrap();
        assert!(section.find("Roles: code, logo").unwrap() < code);
        assert!(code < logo);
        assert!(logo < section.find("sample-label").unwrap());
    }

    /// Acceptance test 3: no placeholder ever appears on the page.
    #[test]
    fn no_placeholder_appears_on_the_page() {
        let html = render_fonts_page();
        let main = &html[html.find("<main").unwrap()..html.find("</main>").unwrap()];
        assert!(!main.contains(" | "));
        assert!(!main.contains("hack |"));
    }

    /// Acceptance test 3: each piece is in its font's section, in the roles' order.
    #[test]
    fn each_piece_is_in_the_section_of_its_font() {
        let html = main_html();
        assert!(samples_in(&html, "Literata").contains("I chose Literata"));
        assert!(samples_in(&html, "Hack").contains("For the code I chose Hack"));
        assert!(
            samples_in(&html, "Literata")
                .contains("The name in the logo is written in Literata too")
        );
    }

    /// The samples of a font: what the page holds between its section and the next.
    fn samples_in(html: &str, title: &str) -> String {
        let start = html.find(&format!("<h2>{title}</h2>")).unwrap();
        let end = html[start + 1..]
            .find("<h2>")
            .map_or(html.len(), |at| at + start + 1);
        html[start..end].to_owned()
    }

    fn samples_of(title: &str) -> String {
        samples_in(&main_html(), title)
    }

    /// The labels and the classes of the samples of a font, in order.
    fn labelled_samples_in(html: &str, title: &str) -> Vec<(String, String)> {
        let samples = samples_in(html, title);
        let mut found = Vec::new();
        for part in samples.split("<p class=\"sample-").skip(1) {
            let class = format!("sample-{}", part.split('"').next().unwrap());
            let label = part
                .split("<div class=\"sample-label\">")
                .nth(1)
                .unwrap()
                .split("</div>")
                .next()
                .unwrap()
                .to_owned();
            found.push((label, class));
        }
        found
    }

    fn labelled_samples(title: &str) -> Vec<(String, String)> {
        labelled_samples_in(&main_html(), title)
    }

    fn expected(samples: &[(&str, &str)]) -> Vec<(String, String)> {
        samples
            .iter()
            .map(|&(label, class)| (label.to_owned(), class.to_owned()))
            .collect()
    }

    /// Acceptance test 6: each file of a font has a sample, labelled with the file's name and
    /// drawn by the variant the build writes from the role, the style and the weight.
    #[test]
    fn each_font_is_followed_by_one_sample_for_each_of_its_files() {
        assert_eq!(
            labelled_samples("Literata"),
            expected(&[
                ("Regular", "sample-text-normal-400"),
                ("Italic", "sample-text-italic-400"),
                ("Bold", "sample-text-normal-700"),
                ("Bold italic", "sample-text-italic-700"),
            ])
        );
        assert_eq!(
            labelled_samples("Hack"),
            expected(&[("Regular", "sample-code-normal-400")])
        );
        assert_eq!(main_html().matches("sample-label").count(), 5);
    }

    /// Acceptance test 6: a file added to the configuration gets its sample, with no other
    /// change, and the variant the build writes for it is the one the sample carries.
    #[test]
    fn a_file_added_to_the_configuration_gets_its_sample() {
        let literata = include_str!("../../config/fonts/literata.txt");
        let more = format!(
            "{literata}semibold.file=literata-latin-600-normal.woff2\nsemibold.name=Semibold\n\
             semibold.style=normal\nsemibold.weight=600\n"
        );
        let describe = |folder: &str| {
            let path = format!("{}/config/fonts/{folder}.txt", env!("CARGO_MANIFEST_DIR"));
            if folder == "literata" {
                Some(more.clone())
            } else {
                std::fs::read_to_string(path).ok()
            }
        };
        let fonts = crate::style_guide::fonts::load(
            "text=literata\ncode=hack\nlogo=literata\n",
            &describe,
            &|_, _| true,
        )
        .unwrap();
        let html = body(FONTS, &fonts).unwrap();
        let samples = labelled_samples_in(&html, "Literata");
        assert_eq!(samples.len(), 5);
        assert_eq!(
            samples[4],
            ("Semibold".to_owned(), "sample-text-normal-600".to_owned())
        );
        assert!(
            crate::style_guide::fonts::css(&fonts).contains("@utility sample-text-normal-600 {")
        );
        assert_eq!(html.matches("sample-label").count(), 6);
    }

    #[test]
    fn the_samples_come_after_the_explanation_of_their_font() {
        let samples = samples_of("Hack");
        assert!(samples.find("Meslo LG itself").unwrap() < samples.find("sample-label").unwrap());
    }

    #[test]
    fn every_sample_shows_the_same_text() {
        let html = main_html();
        assert_eq!(html.matches(&format!(">{SAMPLE_TEXT}</p>")).count(), 5);
    }

    /// A sample never names a font: its variant takes it from the role's variable (the variants
    /// are written by `fonts.rs`, whose tests check their rules), so a change of font in the
    /// configuration shows on the page.
    #[test]
    fn no_sample_names_a_font() {
        let html = render_fonts_page();
        let body = &html[html.find("<main").unwrap()..html.find("</main>").unwrap()];
        for family in ["font-family", "Literata\"", "Hack\"", "style=\""] {
            assert!(!body.contains(family), "{family}");
        }
    }

    /// ADR-001: the page carries only named variants, of the stylesheet or of the colours the
    /// build writes.
    #[test]
    fn every_class_in_the_page_is_a_named_variant_of_the_stylesheet() {
        crate::style_guide::assert_only_named_variants(&render_fonts_page());
    }

    #[test]
    fn the_outline_is_the_title_then_one_level_two_heading_for_each_font() {
        let html = render_fonts_page();
        assert_eq!(html.matches("<h1").count(), 1);
        assert_eq!(html.matches("<h2>").count(), 2);
        assert_eq!(html.matches("<h3").count(), 0);
    }

    #[test]
    fn its_heading_fits_the_frame() {
        assert!(render_fonts_page().contains("<h1 "));
    }

    #[test]
    fn has_its_description_and_preview_tags() {
        let html = render_fonts_page();
        let url = "https://www.lucalorenzon.com/style-guide/fonts.html";
        assert!(html.contains(&format!(
            "<meta name=\"description\" content=\"{DESCRIPTION}\">"
        )));
        assert!(html.contains("<meta property=\"og:title\" content=\"Fonts\">"));
        assert!(html.contains("<meta property=\"og:type\" content=\"website\">"));
        assert!(html.contains(&format!("<meta property=\"og:url\" content=\"{url}\">")));
        assert!(html.contains(&format!(
            "<meta property=\"og:description\" content=\"{DESCRIPTION}\">"
        )));
        assert!(html.contains("<meta property=\"og:site_name\" content=\"Luca Lorenzon\">"));
        assert!(!html.contains("og:image"));
    }

    #[test]
    fn description_fits_a_search_result() {
        assert!(DESCRIPTION.chars().count() <= 160);
    }

    #[test]
    fn has_its_own_full_address_as_canonical_link() {
        assert!(render_fonts_page().contains(
            "<link rel=\"canonical\" href=\"https://www.lucalorenzon.com/style-guide/fonts.html\">"
        ));
    }

    #[test]
    fn sets_tab_title_and_has_fonts_as_only_h1() {
        let html = render_fonts_page();
        assert!(html.contains("<title>Fonts - Style guide - Luca Lorenzon</title>"));
        assert!(html.contains(">Fonts</h1>"));
    }

    #[test]
    fn addresses_are_relative_to_the_guides_folder() {
        let html = render_fonts_page();
        assert!(html.contains("href=\"../style.css\""));
        assert!(!html.contains("href=\"/"));
        assert!(!html.contains("src=\"/"));
    }

    #[test]
    fn footer_says_reuse_with_attribution() {
        let html = render_fonts_page();
        let footer = &html[html.find("<footer").unwrap()..];
        assert!(footer.contains(">Reuse with attribution</a>"));
    }
}
