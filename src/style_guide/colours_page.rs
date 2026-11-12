//! The colours page of the style guide: the text of `content/style-guide/colours.md`, and after
//! each of its sections the samples of that section's colour, drawn from the palette.
//!
//! Every foreground colour is shown in the light and in the dark scheme, side by side,
//! whatever scheme the reader's browser uses: the box of a sample carries `scheme-light` or
//! `scheme-dark` (written by the build, see `colours::css`), so what is inside it is the
//! site's own styles in that scheme. The class names are written out in full in this file on
//! purpose: Tailwind finds the classes it must emit by scanning the sources.

use super::FOLDER;
use super::colours::{ALLOWED_PAIRS, AllowedPair, PALETTE, Role, Scheme, contrast_ratio};
use crate::markdown;
use crate::page_frame::{FooterLicence, Heading, IndexedPage, Indexing, PageFrame, PageKind};
use crate::site_address::SiteAddress;
use leptos::prelude::*;

/// The page's file in the guide's folder, as the index links to it.
pub const FILE: &str = "colours.html";

/// The text of the page, written in Markdown: an introduction, then one section for each
/// foreground colour.
pub(crate) const COLOURS: &str = include_str!("../../content/style-guide/colours.md");

/// What search engines and link previews show of the page.
const DESCRIPTION: &str = "The colours of this site in the light and the dark scheme: what each one is for, and the contrast each pair reaches.";

/// Why the page's text does not match the palette. The build stops with it: the text and the
/// colours must say the same thing.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SectionError {
    #[error(
        "content/style-guide/colours.md: the section `{0}` names no foreground colour (the titles are {titles})",
        titles = foreground_titles()
    )]
    UnknownColour(String),
    #[error("content/style-guide/colours.md: the section `{0}` appears more than once")]
    Duplicate(&'static str),
    #[error("content/style-guide/colours.md: the colour `{0}` has no section")]
    MissingSection(&'static str),
}

fn foreground_titles() -> String {
    ALLOWED_PAIRS
        .iter()
        .map(|pair| pair.foreground.title())
        .collect::<Vec<_>>()
        .join(", ")
}

/// A background, as the samples show it: the label above the sample and the variant that
/// paints it.
struct Ground {
    label: &'static str,
    class: &'static str,
}

fn ground(background: Role) -> Ground {
    match background {
        Role::Page => Ground {
            label: "On the page",
            class: "sample-on-page",
        },
        Role::CodeSurface => Ground {
            label: "On the code surface",
            class: "sample-on-code-surface",
        },
        other => unreachable!("{} is not a background", other.name()),
    }
}

/// What the sample of a colour shows: the colour at work, as the site uses it.
fn example(foreground: Role) -> AnyView {
    match foreground {
        Role::Ink => view! { <p>"Running text, written in the ink."</p> }.into_any(),
        Role::AccentCode => view! {
            <p>"Code in a sentence: " <code>"render_page()"</code></p>
        }
        .into_any(),
        // Level 3: the page's outline stays h1, h2 (the colour's section), h3.
        Role::AccentHeading => view! { <h3>"Titles and headings"</h3> }.into_any(),
        Role::AccentLine => view! {
            <p>"A section above"</p>
            <hr />
            <p>"A section below"</p>
        }
        .into_any(),
        other => unreachable!("{} is not a foreground", other.name()),
    }
}

/// One sample: its label, the box in the scheme and on the background, and the numbers of the
/// colour shown (for the code accent, the code and not the ink around it).
#[component]
fn Sample(scheme: Scheme, pair: AllowedPair, background: Role) -> impl IntoView {
    let colours = PALETTE.scheme(scheme);
    let (foreground_colour, background_colour) =
        (colours.of(pair.foreground), colours.of(background));
    let scheme_class = match scheme {
        Scheme::Light => "scheme-light",
        Scheme::Dark => "scheme-dark",
    };
    let Ground { label, class } = ground(background);
    let numbers = format!(
        "{foreground_colour} on {background_colour} · {:.2}:1, needs {}:1",
        contrast_ratio(foreground_colour, background_colour),
        pair.usage.threshold()
    );
    // The frame has no scheme: it is drawn in the reader's line accent, so a light box on a
    // light page and a dark box on a dark page still show their edge.
    view! {
        <div class="sample-label">{label}</div>
        <div class="sample-frame">
            <div class=format!("{scheme_class} sample-box {class}")>{example(pair.foreground)}</div>
        </div>
        <div class="sample-numbers">{numbers}</div>
    }
}

/// The samples of one colour: the light scheme, then the dark, one sample for each background
/// the colour may be used on. A forbidden pair never appears.
#[component]
fn Samples(pair: AllowedPair) -> impl IntoView {
    let column = move |scheme: Scheme, name: &'static str| {
        let samples = pair
            .backgrounds
            .iter()
            .map(|&background| view! { <Sample scheme=scheme pair=pair background=background /> })
            .collect_view();
        view! {
            <div class="sample-column">
                <div class="scheme-name">{name}</div>
                {samples}
            </div>
        }
    };
    view! {
        <div class="sample-stacked md:sample-side-by-side">
            {column(Scheme::Light, "Light")}
            {column(Scheme::Dark, "Dark")}
        </div>
    }
}

/// Puts the samples of each foreground colour after its section: after the section's text,
/// before the next section. The HTML is split at `<h2>`, which the converter writes for a
/// level-2 heading only (the same characters in a code block are escaped).
fn with_samples(
    html: &str,
    samples: impl Fn(AllowedPair) -> String,
) -> Result<String, SectionError> {
    let mut sections = html.split("<h2>");
    let mut out = sections.next().unwrap_or_default().to_owned();
    let mut seen: Vec<Role> = Vec::new();
    for section in sections {
        let title = section.split("</h2>").next().unwrap_or_default();
        let pair = ALLOWED_PAIRS
            .iter()
            .find(|pair| pair.foreground.title() == title)
            .ok_or_else(|| SectionError::UnknownColour(title.to_owned()))?;
        if seen.contains(&pair.foreground) {
            return Err(SectionError::Duplicate(pair.foreground.title()));
        }
        seen.push(pair.foreground);
        out.push_str("<h2>");
        out.push_str(section);
        out.push_str(&samples(*pair));
    }
    if let Some(pair) = ALLOWED_PAIRS
        .iter()
        .find(|pair| !seen.contains(&pair.foreground))
    {
        return Err(SectionError::MissingSection(pair.foreground.title()));
    }
    Ok(out)
}

/// The page's main content: the converted Markdown with the samples placed in it.
fn body(markdown: &str) -> Result<String, SectionError> {
    let owner = Owner::new();
    with_samples(&markdown::to_html(markdown), |pair| {
        owner.with(|| view! { <Samples pair=pair /> }.to_html())
    })
}

#[component]
pub fn ColoursPage(canonical: String, body: String) -> impl IntoView {
    view! {
        <PageFrame
            title="Colours - Style guide - Luca Lorenzon"
            heading=Heading::try_from("Colours").expect("the colours page's heading fits the frame")
            address_prefix="../"
            footer_licence=FooterLicence::ReuseWithAttribution
            indexing=Indexing::Indexed(IndexedPage {
                canonical,
                description: DESCRIPTION.to_owned(),
                kind: PageKind::Website,
            })
        >
            <main class="long-text-column md:long-text-large" inner_html=body></main>
        </PageFrame>
    }
}

/// Renders the colours page to a complete HTML document string, or says why the text does not
/// match the palette.
pub fn render_colours_page(site: &SiteAddress) -> Result<String, SectionError> {
    let body = body(COLOURS)?;
    let owner = Owner::new();
    let canonical = site.join(&format!("{FOLDER}{FILE}"));
    let page = owner.with(|| view! { <ColoursPage canonical=canonical body=body /> }.to_html());
    Ok(format!("<!DOCTYPE html>{page}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn site() -> SiteAddress {
        SiteAddress::parse("https://www.lucalorenzon.com/").unwrap()
    }

    fn render_colours_page() -> String {
        super::render_colours_page(&site()).unwrap()
    }

    /// A text with the four sections, in the order given by the titles.
    fn text_with(titles: &[&str]) -> String {
        let mut text = String::from("Introduction.\n");
        for title in titles {
            text.push_str(&format!("\n## {title}\n\nWhy.\n"));
        }
        text
    }

    const ALL_TITLES: [&str; 4] = ["Ink", "Code accent", "Heading accent", "Line accent"];

    #[test]
    fn the_samples_follow_their_section_and_come_before_the_next_one() {
        let html = body(&text_with(&ALL_TITLES)).unwrap();
        let mut at = html.find("Introduction.").unwrap();
        for title in ALL_TITLES {
            let section = html.find(&format!("<h2>{title}</h2>")).unwrap();
            let samples = html[section..].find("sample-stacked").unwrap() + section;
            assert!(at < section && section < samples, "{title}");
            at = samples;
        }
        assert_eq!(html.matches("sample-stacked").count(), 4);
        let heading_accent = html.find("<h2>Heading accent</h2>").unwrap();
        let line_accent = html.find("<h2>Line accent</h2>").unwrap();
        assert!(html[heading_accent..line_accent].contains("sample-stacked"));
    }

    #[test]
    fn the_sections_may_come_in_any_order() {
        let mut titles = ALL_TITLES;
        titles.reverse();
        assert!(body(&text_with(&titles)).is_ok());
    }

    #[test]
    fn a_section_that_names_no_colour_is_an_error() {
        let mut titles = ALL_TITLES.to_vec();
        titles.push("Page");
        let error = body(&text_with(&titles)).unwrap_err();
        assert_eq!(error, SectionError::UnknownColour("Page".to_owned()));
        assert_eq!(
            error.to_string(),
            "content/style-guide/colours.md: the section `Page` names no foreground colour (the titles are Ink, Code accent, Heading accent, Line accent)"
        );
    }

    #[test]
    fn a_colour_without_a_section_is_an_error() {
        let error = body(&text_with(&["Ink", "Code accent", "Line accent"])).unwrap_err();
        assert_eq!(error, SectionError::MissingSection("Heading accent"));
        assert_eq!(
            error.to_string(),
            "content/style-guide/colours.md: the colour `Heading accent` has no section"
        );
        assert_eq!(
            body("Only an introduction.\n").unwrap_err(),
            SectionError::MissingSection("Ink")
        );
    }

    #[test]
    fn a_section_written_twice_is_an_error() {
        let mut titles = ALL_TITLES.to_vec();
        titles.push("Ink");
        let error = body(&text_with(&titles)).unwrap_err();
        assert_eq!(error, SectionError::Duplicate("Ink"));
        assert_eq!(
            error.to_string(),
            "content/style-guide/colours.md: the section `Ink` appears more than once"
        );
    }

    #[test]
    fn the_real_text_matches_the_palette() {
        assert!(body(COLOURS).is_ok());
    }

    #[test]
    fn the_real_text_has_four_sections_and_no_values() {
        let html = markdown::to_html(COLOURS);
        assert_eq!(html.matches("<h2>").count(), 4);
        // The samples print the codes and the ratios, so the text does not repeat them.
        assert!(!html.contains("on #"));
    }

    /// The samples of one colour, as the page renders them.
    fn samples_of(foreground: Role) -> String {
        let html = body(&text_with(&ALL_TITLES)).unwrap();
        let start = html
            .find(&format!("<h2>{}</h2>", foreground.title()))
            .unwrap();
        let end = html[start + 1..]
            .find("<h2>")
            .map_or(html.len(), |at| at + start + 1);
        html[start..end].to_owned()
    }

    #[test]
    fn each_colour_shows_light_then_dark_with_one_sample_for_each_allowed_background() {
        for pair in ALLOWED_PAIRS {
            let samples = samples_of(pair.foreground);
            let light = samples.find("Light").unwrap();
            let dark = samples.find("Dark").unwrap();
            assert!(light < dark);
            let boxes = pair.backgrounds.len();
            assert_eq!(samples.matches("scheme-light sample-box").count(), boxes);
            assert_eq!(samples.matches("scheme-dark sample-box").count(), boxes);
        }
    }

    #[test]
    fn the_heading_accent_never_shows_on_the_code_surface() {
        let samples = samples_of(Role::AccentHeading);
        assert!(samples.contains("sample-on-page"));
        assert!(samples.contains(">On the page<"));
        assert!(!samples.contains("sample-on-code-surface"));
        assert!(!samples.contains("On the code surface"));
        assert!(samples_of(Role::Ink).contains("sample-on-code-surface"));
    }

    #[test]
    fn the_numbers_come_from_the_palette() {
        let ink = samples_of(Role::Ink);
        assert!(ink.contains("#1c1a17 on #fbfaf7 · 16.63:1, needs 7:1"));
        assert!(ink.contains("#1c1a17 on #ebe7df · 14.08:1, needs 7:1"));
        assert!(ink.contains("#e6e6e6 on #000000 · 16.83:1, needs 7:1"));
        let code = samples_of(Role::AccentCode);
        assert!(code.contains("#6f3f09 on #ebe7df · 7.12:1, needs 7:1"));
        let heading = samples_of(Role::AccentHeading);
        assert!(heading.contains("#a9600d on #fbfaf7 · 4.61:1, needs 4.5:1"));
        assert!(!heading.contains("#ebe7df"));
        let line = samples_of(Role::AccentLine);
        assert!(line.contains("#c06d0f on #fbfaf7 · 3.70:1, needs 3:1"));
        assert!(line.contains("#dd9911 on #1a1a1a · 7.16:1, needs 3:1"));
    }

    #[test]
    fn each_colour_is_shown_at_work() {
        assert!(samples_of(Role::Ink).contains("<p>Running text, written in the ink.</p>"));
        assert!(
            samples_of(Role::AccentCode)
                .contains("<p>Code in a sentence: <code>render_page()</code></p>")
        );
        assert!(samples_of(Role::AccentHeading).contains("<h3>Titles and headings</h3>"));
        assert!(
            samples_of(Role::AccentLine)
                .contains("<p>A section above</p><hr><p>A section below</p>")
        );
    }

    #[test]
    fn the_frame_has_no_scheme_of_its_own() {
        let samples = samples_of(Role::Ink);
        assert!(samples.contains("<div class=\"sample-frame\"><div class=\"scheme-light "));
    }

    /// ADR-001: the page carries only named variants, of the stylesheet or of the colours the
    /// build writes.
    #[test]
    fn every_class_in_the_page_is_a_named_variant_of_the_stylesheet() {
        crate::style_guide::assert_only_named_variants(&render_colours_page());
    }

    #[test]
    fn its_heading_fits_the_frame() {
        assert!(render_colours_page().contains("<h1 "));
    }

    #[test]
    fn has_its_description_and_preview_tags() {
        let html = render_colours_page();
        let url = "https://www.lucalorenzon.com/style-guide/colours.html";
        assert!(html.contains(&format!(
            "<meta name=\"description\" content=\"{DESCRIPTION}\">"
        )));
        assert!(html.contains("<meta property=\"og:title\" content=\"Colours\">"));
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
        assert!(render_colours_page().contains(
            "<link rel=\"canonical\" href=\"https://www.lucalorenzon.com/style-guide/colours.html\">"
        ));
    }

    #[test]
    fn sets_tab_title_and_has_colours_as_only_h1() {
        let html = render_colours_page();
        assert!(html.contains("<title>Colours - Style guide - Luca Lorenzon</title>"));
        assert_eq!(html.matches("<h1").count(), 1);
        assert!(html.contains(">Colours</h1>"));
    }

    #[test]
    fn addresses_are_relative_to_the_guides_folder() {
        let html = render_colours_page();
        assert!(html.contains("href=\"../style.css\""));
        assert!(html.contains("<link rel=\"icon\" href=\"../favicon.ico\""));
        assert!(html.contains("<a href=\"../about.html\""));
        assert!(html.contains("<a href=\"../\" aria-label=\"Lvk@73r, home page\""));
        assert!(!html.contains("href=\"/"));
        assert!(!html.contains("src=\"/"));
    }

    #[test]
    fn footer_says_reuse_with_attribution() {
        let html = render_colours_page();
        let footer = &html[html.find("<footer").unwrap()..];
        assert!(footer.contains(">Reuse with attribution</a>"));
    }
}
