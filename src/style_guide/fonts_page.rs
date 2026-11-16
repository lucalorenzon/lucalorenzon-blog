//! The fonts page of the style guide: the text of `content/style-guide/fonts.md`, and after each
//! of its sections the samples of that font, one for each style the site serves.
//!
//! Every sample shows the same text, so that only the font changes from one to the next. A
//! sample never names a font: it carries a variant of the stylesheet (`style/input.css`) that
//! takes the font from Tailwind's `font-text`, `font-code` or `font-logo`, so a change of font
//! in the stylesheet shows here too. The variants' names are written out in full in this file
//! on purpose: Tailwind finds the classes it must emit by scanning the sources.

use super::FOLDER;
use super::navigation::GuideMain;
use crate::markdown;
use crate::page_frame::{FooterLicence, Heading, IndexedPage, Indexing, PageFrame, PageKind};
use crate::site_address::SiteAddress;
use leptos::prelude::*;

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

/// A font of the site: the title of its section and its samples, each a label and the variant
/// of the stylesheet that draws it.
struct Font {
    title: &'static str,
    samples: &'static [(&'static str, &'static str)],
}

const FONTS_SHOWN: [Font; 3] = [
    Font {
        title: "Literata",
        samples: &[
            ("Regular", "sample-literata"),
            ("Italic", "sample-literata-italic"),
            ("Bold", "sample-literata-bold"),
            ("Bold italic", "sample-literata-bold-italic"),
        ],
    },
    Font {
        title: "Hack",
        samples: &[("Regular", "sample-hack")],
    },
    Font {
        title: "IBM VGA 8x16",
        samples: &[("Regular", "sample-pixel")],
    },
];

/// Why the page's text does not match the fonts. The build stops with it: the text and the
/// fonts must say the same thing.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SectionError {
    #[error(
        "content/style-guide/fonts.md: the section `{0}` names no font (the titles are {titles})",
        titles = titles()
    )]
    UnknownFont(String),
    #[error("content/style-guide/fonts.md: the section `{0}` appears more than once")]
    Duplicate(&'static str),
    #[error("content/style-guide/fonts.md: the font `{0}` has no section")]
    MissingSection(&'static str),
}

fn titles() -> String {
    FONTS_SHOWN
        .iter()
        .map(|font| font.title)
        .collect::<Vec<_>>()
        .join(", ")
}

/// The samples of one font: for each style, the text drawn in it and, under it as a caption,
/// the style's name (above the text it would read as the caption of the paragraph before).
#[component]
fn Samples(font: &'static Font) -> impl IntoView {
    font.samples
        .iter()
        .map(|&(label, class)| {
            view! {
                <p class=class>{SAMPLE_TEXT}</p>
                <div class="sample-label">{label}</div>
            }
        })
        .collect_view()
}

/// Puts the samples of each font after its section: after the section's text, before the next
/// section. The HTML is split at `<h2>`, which the converter writes for a level-2 heading only
/// (the same characters in a code block are escaped).
fn with_samples(
    html: &str,
    samples: impl Fn(&'static Font) -> String,
) -> Result<String, SectionError> {
    let mut sections = html.split("<h2>");
    let mut out = sections.next().unwrap_or_default().to_owned();
    let mut seen: Vec<&str> = Vec::new();
    for section in sections {
        let title = section.split("</h2>").next().unwrap_or_default();
        let font = FONTS_SHOWN
            .iter()
            .find(|font| font.title == title)
            .ok_or_else(|| SectionError::UnknownFont(title.to_owned()))?;
        if seen.contains(&font.title) {
            return Err(SectionError::Duplicate(font.title));
        }
        seen.push(font.title);
        out.push_str("<h2>");
        out.push_str(section);
        out.push_str(&samples(font));
    }
    if let Some(font) = FONTS_SHOWN.iter().find(|font| !seen.contains(&font.title)) {
        return Err(SectionError::MissingSection(font.title));
    }
    Ok(out)
}

/// The page's main content: the converted Markdown with the samples placed in it.
fn body(markdown: &str) -> Result<String, SectionError> {
    let owner = Owner::new();
    with_samples(&markdown::to_html(markdown), |font| {
        owner.with(|| view! { <Samples font=font /> }.to_html())
    })
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
            <GuideMain title="Fonts" body=body />
        </PageFrame>
    }
}

/// Renders the fonts page to a complete HTML document string, or says why the text does not
/// match the fonts.
pub fn render_fonts_page(site: &SiteAddress) -> Result<String, SectionError> {
    let body = body(FONTS)?;
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

    fn render_fonts_page() -> String {
        super::render_fonts_page(&site()).unwrap()
    }

    /// A text with the sections, in the order given by the titles.
    fn text_with(titles: &[&str]) -> String {
        let mut text = String::from("Introduction.\n");
        for title in titles {
            text.push_str(&format!("\n## {title}\n\nWhy.\n"));
        }
        text
    }

    const ALL_TITLES: [&str; 3] = ["Literata", "Hack", "IBM VGA 8x16"];

    #[test]
    fn a_section_that_names_no_font_is_an_error() {
        let mut titles = ALL_TITLES.to_vec();
        titles.push("Meslo LG");
        let error = body(&text_with(&titles)).unwrap_err();
        assert_eq!(error, SectionError::UnknownFont("Meslo LG".to_owned()));
        assert_eq!(
            error.to_string(),
            "content/style-guide/fonts.md: the section `Meslo LG` names no font (the titles are Literata, Hack, IBM VGA 8x16)"
        );
    }

    #[test]
    fn a_font_without_a_section_is_an_error() {
        let error = body(&text_with(&["Literata", "IBM VGA 8x16"])).unwrap_err();
        assert_eq!(error, SectionError::MissingSection("Hack"));
        assert_eq!(
            error.to_string(),
            "content/style-guide/fonts.md: the font `Hack` has no section"
        );
    }

    #[test]
    fn a_section_written_twice_is_an_error() {
        let mut titles = ALL_TITLES.to_vec();
        titles.push("Hack");
        assert_eq!(
            body(&text_with(&titles)).unwrap_err(),
            SectionError::Duplicate("Hack")
        );
    }

    #[test]
    fn the_real_text_matches_the_fonts() {
        assert!(body(FONTS).is_ok());
    }

    /// The page's main content, as rendered: the introduction, then each section with its
    /// samples.
    fn main_html() -> String {
        body(FONTS).unwrap()
    }

    #[test]
    fn the_page_holds_the_introduction_and_one_section_for_each_font() {
        let html = main_html();
        assert!(
            html.starts_with(
                "<p>These are the fonts this site uses, each in every style it serves."
            )
        );
        assert_eq!(html.matches("<h2>").count(), 3);
        let literata = html.find("<h2>Literata</h2>").unwrap();
        let hack = html.find("<h2>Hack</h2>").unwrap();
        let vga = html.find("<h2>IBM VGA 8x16</h2>").unwrap();
        assert!(0 < literata && literata < hack && hack < vga);
    }

    /// The samples of a font: what the page holds between its section and the next.
    fn samples_of(title: &str) -> String {
        let html = main_html();
        let start = html.find(&format!("<h2>{title}</h2>")).unwrap();
        let end = html[start + 1..]
            .find("<h2>")
            .map_or(html.len(), |at| at + start + 1);
        html[start..end].to_owned()
    }

    /// The labels and the classes of the samples of a font, in order.
    fn labelled_samples(title: &str) -> Vec<(String, String)> {
        let samples = samples_of(title);
        let mut found = Vec::new();
        for part in samples.split("<p class=\"").skip(1) {
            let class = part.split('"').next().unwrap().to_owned();
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

    fn expected(samples: &[(&str, &str)]) -> Vec<(String, String)> {
        samples
            .iter()
            .map(|&(label, class)| (label.to_owned(), class.to_owned()))
            .collect()
    }

    #[test]
    fn each_font_is_followed_by_its_samples_named_by_their_style() {
        assert_eq!(
            labelled_samples("Literata"),
            expected(&[
                ("Regular", "sample-literata"),
                ("Italic", "sample-literata-italic"),
                ("Bold", "sample-literata-bold"),
                ("Bold italic", "sample-literata-bold-italic"),
            ])
        );
        assert_eq!(
            labelled_samples("Hack"),
            expected(&[("Regular", "sample-hack")])
        );
        assert_eq!(
            labelled_samples("IBM VGA 8x16"),
            expected(&[("Regular", "sample-pixel")])
        );
        assert_eq!(main_html().matches("sample-label").count(), 6);
    }

    #[test]
    fn the_samples_come_after_the_explanation_of_their_font() {
        let samples = samples_of("Hack");
        assert!(samples.find("Meslo LG itself").unwrap() < samples.find("sample-label").unwrap());
    }

    #[test]
    fn every_sample_shows_the_same_text() {
        let html = main_html();
        assert_eq!(html.matches(&format!(">{SAMPLE_TEXT}</p>")).count(), 6);
    }

    /// Acceptance test 3: a sample never names a font. The stylesheet's variants take it from
    /// the font variables, which this test reads, so a change of font there shows on the page.
    #[test]
    fn no_sample_names_a_font_and_each_uses_its_fonts_variable_in_the_stylesheet() {
        let html = render_fonts_page();
        for family in ["font-family", "Literata\"", "Hack\"", "VGA", "style=\""] {
            let body = &html[html.find("<main").unwrap()..html.find("</main>").unwrap()];
            let body = body.replace("IBM VGA 8x16", "");
            assert!(!body.contains(family), "{family}");
        }
        let css = include_str!("../../style/input.css");
        let variant = |name: &str| {
            let start = css.find(&format!("@utility {name} {{")).unwrap();
            let rule = &css[start..];
            rule[..rule.find('}').unwrap()].to_owned()
        };
        for (name, classes) in [
            ("sample-literata", ["font-text"].as_slice()),
            ("sample-literata-italic", ["font-text", "italic"].as_slice()),
            (
                "sample-literata-bold",
                ["font-text", "font-bold"].as_slice(),
            ),
            (
                "sample-literata-bold-italic",
                ["font-text", "font-bold", "italic"].as_slice(),
            ),
            ("sample-hack", ["font-code"].as_slice()),
            (
                "sample-pixel",
                ["font-logo", "text-[16px]", "max-w-[80ch]"].as_slice(),
            ),
        ] {
            let rule = variant(name);
            assert!(!rule.contains("font-family"), "{name}");
            for class in classes {
                assert!(rule.contains(class), "{name} lacks {class}");
            }
        }
        // Literata and Hack keep the page's size.
        assert!(!variant("sample-hack").contains("text-"));
        assert!(!variant("sample-literata").contains("text-"));
    }

    /// The stylesheet's text without its comments, and without the `@theme` block and the
    /// `@font-face` rules, the only places that may name a font.
    fn stylesheet_outside_the_fonts_places(css: &str) -> String {
        let mut without_comments = String::new();
        let mut rest = css;
        while let Some(start) = rest.find("/*") {
            without_comments.push_str(&rest[..start]);
            rest = &rest[start + 2..];
            rest = &rest[rest.find("*/").map_or(rest.len(), |end| end + 2)..];
        }
        without_comments.push_str(rest);

        let mut kept = String::new();
        let mut rest = without_comments.as_str();
        while !rest.is_empty() {
            let next = ["@theme", "@font-face"]
                .iter()
                .filter_map(|at| rest.find(at))
                .min();
            let Some(start) = next else {
                kept.push_str(rest);
                break;
            };
            kept.push_str(&rest[..start]);
            rest = &rest[start..];
            let open = rest.find('{').expect("an at-rule without a block");
            let mut depth = 0;
            let mut end = rest.len();
            for (index, character) in rest[open..].char_indices() {
                match character {
                    '{' => depth += 1,
                    '}' => {
                        depth -= 1;
                        if depth == 0 {
                            end = open + index + 1;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            rest = &rest[end..];
        }
        kept
    }

    /// What names a font in the text given: a `font-family` whose value is not one of the
    /// fonts' variables, or the name of one of the three fonts.
    fn font_names_in(text: &str) -> Vec<String> {
        let mut found = Vec::new();
        for declaration in text.split("font-family:").skip(1) {
            let value = declaration.split(';').next().unwrap().trim();
            if !value.starts_with("var(--font-") {
                found.push(format!("font-family: {value}"));
            }
        }
        for name in ["Literata", "Hack", "IBM VGA"] {
            if text.contains(name) {
                found.push(name.to_owned());
            }
        }
        found
    }

    /// Acceptance test 3 of the card on the fonts' three roles: outside `@theme` and
    /// `@font-face`, the stylesheet names no font, so changing a role's value changes the site.
    #[test]
    fn the_stylesheet_names_no_font_outside_the_theme_and_the_font_face_rules() {
        let css = include_str!("../../style/input.css");
        let outside = stylesheet_outside_the_fonts_places(css);
        assert!(
            outside.contains("@utility sample-literata"),
            "the helper cut too much"
        );
        assert!(
            outside.contains("font-family: var(--font-text)"),
            "html and body"
        );
        assert_eq!(font_names_in(&outside), Vec::<String>::new());
    }

    #[test]
    fn a_font_written_by_hand_outside_the_theme_is_found() {
        let css = include_str!("../../style/input.css");
        let by_hand = css.replace(
            "font-family: var(--font-text);",
            "font-family: \"Literata\", serif;",
        );
        assert_ne!(
            by_hand, css,
            "html and body no longer set font-family: var(--font-text)"
        );
        let found = font_names_in(&stylesheet_outside_the_fonts_places(&by_hand));
        assert_eq!(found, ["font-family: \"Literata\", serif", "Literata"]);
        // A name in a comment, in the theme or in a font face is no violation.
        let outside = stylesheet_outside_the_fonts_places(css);
        assert!(css.contains("\"Hack\"") && !outside.contains("Hack"));
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
        assert_eq!(html.matches("<h2>").count(), 3);
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
