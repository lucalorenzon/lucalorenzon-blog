//! The fonts page of the style guide: the text of `content/style-guide/fonts.md`, and after each
//! of its sections the samples of that font, one for each file the site serves.
//!
//! The fonts, their names and their samples come from the configuration (`config/`, read by
//! `fonts.rs`): a section of the text is matched to a font by its family name, and the font's
//! files give the samples, each labelled with its `name`. Every sample shows the same text, so
//! that only the font changes from one to the next. A sample never names a font: it carries a
//! variant the build writes in `target/generated/fonts.css` (`sample-<role>-<style>-<weight>`),
//! which takes the font from the role's variable, so a change of font in the configuration
//! shows here too, and a file added to it gets its sample with nothing written by hand.

use super::FOLDER;
use super::navigation::GuideMain;
use crate::markdown;
use crate::page_frame::{FooterLicence, Heading, IndexedPage, Indexing, PageFrame, PageKind};
use crate::site_address::SiteAddress;
use crate::style_guide::fonts::{Fonts, ShownFont};
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

/// Why the page's text does not match the fonts. The build stops with it: the text and the
/// fonts must say the same thing.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SectionError {
    #[error(
        "content/style-guide/fonts.md: the section `{title}` names no font (the titles are {titles})"
    )]
    UnknownFont { title: String, titles: String },
    #[error("content/style-guide/fonts.md: the section `{0}` appears more than once")]
    Duplicate(String),
    #[error("content/style-guide/fonts.md: the font `{0}` has no section")]
    MissingSection(String),
}

fn titles(fonts: &[ShownFont]) -> String {
    fonts
        .iter()
        .map(|font| font.family.as_str())
        .collect::<Vec<_>>()
        .join(", ")
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

/// Puts the samples of each font after its section: after the section's text, before the next
/// section. The HTML is split at `<h2>`, which the converter writes for a level-2 heading only
/// (the same characters in a code block are escaped).
fn with_samples(
    html: &str,
    fonts: &[ShownFont],
    samples: impl Fn(&ShownFont) -> String,
) -> Result<String, SectionError> {
    let mut sections = html.split("<h2>");
    let mut out = sections.next().unwrap_or_default().to_owned();
    let mut seen: Vec<&str> = Vec::new();
    for section in sections {
        let title = section.split("</h2>").next().unwrap_or_default();
        let font = fonts
            .iter()
            .find(|font| font.family == title)
            .ok_or_else(|| SectionError::UnknownFont {
                title: title.to_owned(),
                titles: titles(fonts),
            })?;
        if seen.contains(&font.family.as_str()) {
            return Err(SectionError::Duplicate(font.family.clone()));
        }
        seen.push(&font.family);
        out.push_str("<h2>");
        out.push_str(section);
        out.push_str(&samples(font));
    }
    if let Some(font) = fonts
        .iter()
        .find(|font| !seen.contains(&font.family.as_str()))
    {
        return Err(SectionError::MissingSection(font.family.clone()));
    }
    Ok(out)
}

/// The page's main content: the converted Markdown with the samples placed in it.
fn body(markdown: &str, fonts: &Fonts) -> Result<String, SectionError> {
    let owner = Owner::new();
    with_samples(&markdown::to_html(markdown), &fonts.shown(), |font| {
        owner.with(|| view! { <Samples font=font.clone() /> }.to_html())
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
        let error = body(&text_with(&titles), &fonts()).unwrap_err();
        assert_eq!(
            error,
            SectionError::UnknownFont {
                title: "Meslo LG".to_owned(),
                titles: "Literata, Hack, IBM VGA 8x16".to_owned()
            }
        );
        assert_eq!(
            error.to_string(),
            "content/style-guide/fonts.md: the section `Meslo LG` names no font (the titles are Literata, Hack, IBM VGA 8x16)"
        );
    }

    #[test]
    fn a_font_without_a_section_is_an_error() {
        let error = body(&text_with(&["Literata", "IBM VGA 8x16"]), &fonts()).unwrap_err();
        assert_eq!(error, SectionError::MissingSection("Hack".to_owned()));
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
            body(&text_with(&titles), &fonts()).unwrap_err(),
            SectionError::Duplicate("Hack".to_owned())
        );
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
        assert_eq!(html.matches("<h2>").count(), 3);
        let literata = html.find("<h2>Literata</h2>").unwrap();
        let hack = html.find("<h2>Hack</h2>").unwrap();
        let vga = html.find("<h2>IBM VGA 8x16</h2>").unwrap();
        assert!(0 < literata && literata < hack && hack < vga);
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
        assert_eq!(
            labelled_samples("IBM VGA 8x16"),
            expected(&[("Regular", "sample-logo-normal-400")])
        );
        assert_eq!(main_html().matches("sample-label").count(), 6);
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
            "text=literata\ncode=hack\nlogo=ibm-vga-8x16\n",
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
        assert_eq!(html.matches("sample-label").count(), 7);
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

    /// A sample never names a font: its variant takes it from the role's variable (the variants
    /// are written by `fonts.rs`, whose tests check their rules), so a change of font in the
    /// configuration shows on the page.
    #[test]
    fn no_sample_names_a_font() {
        let html = render_fonts_page();
        let body = &html[html.find("<main").unwrap()..html.find("</main>").unwrap()];
        let body = body.replace("IBM VGA 8x16", "");
        for family in ["font-family", "Literata\"", "Hack\"", "VGA", "style=\""] {
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
