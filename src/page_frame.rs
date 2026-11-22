use leptos::either::{Either, EitherOf3};
use leptos::prelude::*;
use std::sync::LazyLock;
use unicode_segmentation::UnicodeSegmentation;

/// The class of the `<main>`: the text column's look, with the text growing with the page's width.
/// It is in `style/input.css`.
const MAIN_CLASSES: &str = "long-text-column";

/// The widths, in pixels, of the logo's round icon files: `assets/logo/logo-<width>.webp`, each
/// sharpened at its own size. The browser picks one by the icon's size and the screen's density.
pub const LOGO_WIDTHS: [u32; 4] = [48, 64, 96, 128];

/// The file the icon shows when the browser reads no `srcset`, and the size its `width` and
/// `height` attributes give (the largest it is drawn at, from 1366px of page).
const LOGO_DEFAULT_WIDTH: u32 = 64;

/// The icon's width in the page, as the stylesheet writes it (`logo-link` in `style/input.css`:
/// 48px at 320px of page, 64px from 1366px, a straight line between), in `vw` because container
/// units have no container in `sizes`. It differs from the page's width only by the scrollbar.
const LOGO_SIZES: &str = "clamp(48px, calc(48px + (100vw - 320px) * 16 / 1046), 64px)";

/// The `srcset` of the logo's icon: the files of [`LOGO_WIDTHS`], each with its width.
fn logo_srcset(address_prefix: &str) -> String {
    LOGO_WIDTHS
        .iter()
        .map(|width| format!("{address_prefix}logo/logo-{width}.webp {width}w"))
        .collect::<Vec<_>>()
        .join(", ")
}

/// The licence phrase a page shows in its footer, as the link to the about
/// page. Each page chooses its own; the frame renders it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FooterLicence {
    /// Everything but the curriculum: reuse is allowed, citing the author.
    ReuseWithAttribution,
    /// The curriculum, shown on the curriculum page: copying yes, changing no.
    CurriculumCopyNotChange,
}

impl FooterLicence {
    fn phrase(self) -> &'static str {
        match self {
            Self::ReuseWithAttribution => "Reuse with attribution",
            Self::CurriculumCopyNotChange => "This CV may be copied, not changed",
        }
    }
}

/// The site's name, as in the tab titles; shown by link previews (`og:site_name`).
pub const SITE_NAME: &str = "Luca Lorenzon";

/// The language of the text, in the form the Open Graph protocol asks for. The site spells in
/// British English; the pages say `lang="en"`, of which this is a case.
pub const LOCALE: &str = "en_GB";

/// What an indexed page is, for the `og:type` tag and the tags that go with it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PageKind {
    /// A page of the site that is neither of the others: the home page, the licence page.
    Website,
    /// A page that speaks of the author: the curriculum, the about page.
    Profile,
    /// An article, with the day it was published in the form `2023-04-30`.
    Article { published: String },
}

/// What an indexed page says about itself to search engines and link previews.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IndexedPage {
    /// The page's full address, however it is reached.
    pub canonical: String,
    /// One text for the search result and the preview alike, at most 160 characters.
    pub description: String,
    pub kind: PageKind,
}

/// What a page tells search engines about indexing. Every page says exactly one thing: a page
/// that says both "this is the page" (canonical) and "do not index" would send contradicting
/// signals. The description and the preview tags exist only for indexed pages, so they live in
/// the indexed variant: no page has to leave them out.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Indexing {
    /// The page is indexed.
    Indexed(IndexedPage),
    /// The page is kept out of the index. For the "page not found", which GitHub also serves
    /// with status 200 when asked for directly.
    NoIndex,
}

/// A preview tag, `<meta property=".." content="..">`. The `view!` macro takes `property` for a
/// DOM property, not an attribute, so the attribute is spelled out here.
fn preview_tag(property: &'static str, content: impl Into<String>) -> impl IntoView {
    leptos::html::meta()
        .attr("property", property)
        .attr("content", content.into())
}

/// The configuration file that holds the heading's length limit. One file is the single source
/// of the figure: the build includes it when it is compiled, and the header's browser test
/// (`end2end/tests/header-layout.spec.ts`) reads the same file from the disk.
const HEADING_MAX_GRAPHEMES_FILE: &str = include_str!("../config/heading-max-graphemes.txt");

/// The most graphemes a [`Heading`] may have. With that many, a heading beside the logo takes
/// at most two lines from 1280px wide (decided 2026-10-29).
static HEADING_MAX_GRAPHEMES: LazyLock<usize> = LazyLock::new(|| {
    parse_limit(HEADING_MAX_GRAPHEMES_FILE).expect("config/heading-max-graphemes.txt")
});

/// Reads the limit from the configuration file's text: a positive whole number.
fn parse_limit(text: &str) -> Result<usize, String> {
    match text.trim().parse::<usize>() {
        Ok(0) => Err("the heading limit must be more than zero".to_owned()),
        Ok(limit) => Ok(limit),
        Err(error) => Err(format!("`{}` is not a whole number: {error}", text.trim())),
    }
}

/// The most graphemes a heading may have, as the configuration file says.
pub fn heading_max_graphemes() -> usize {
    *HEADING_MAX_GRAPHEMES
}

/// The text the frame shows as the page's heading (the `<h1>` and the link preview's title).
/// Trimmed, not empty and short enough to fit the header: at most
/// [`heading_max_graphemes`] graphemes. It belongs to the view, not to the content: an article's
/// title may differ from the heading its page shows.
///
/// Length is counted in graphemes (extended grapheme clusters), what a reader sees as one
/// character, so an accent written as a letter plus a combining mark, or an emoji made of
/// several code points, counts once. The messages say "characters" because the author of an
/// article reads them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Heading(String);

/// Why a text is not a [`Heading`].
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum HeadingError {
    #[error("the title is empty")]
    Empty,
    #[error("the title has {graphemes} characters, at most {max} fit in the page heading")]
    TooLong { graphemes: usize, max: usize },
}

impl TryFrom<&str> for Heading {
    type Error = HeadingError;

    fn try_from(text: &str) -> Result<Self, HeadingError> {
        let text = text.trim();
        if text.is_empty() {
            return Err(HeadingError::Empty);
        }
        let graphemes = text.graphemes(true).count();
        let max = heading_max_graphemes();
        if graphemes > max {
            return Err(HeadingError::TooLong { graphemes, max });
        }
        Ok(Self(text.to_owned()))
    }
}

impl Heading {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// The tags every indexed page has, whatever its kind.
fn common_tags(page: IndexedPage, heading: String, og_type: &'static str) -> impl IntoView {
    view! {
        <link rel="canonical" href=page.canonical.clone() />
        <meta name="description" content=page.description.clone() />
        {preview_tag("og:title", heading)}
        {preview_tag("og:type", og_type)}
        {preview_tag("og:url", page.canonical)}
        {preview_tag("og:description", page.description)}
        {preview_tag("og:site_name", SITE_NAME)}
        {preview_tag("og:locale", LOCALE)}
    }
}

/// The frame every page shares: the `<html>` element with its `<head>`, the
/// header (logo, title, reserved menu slot), the `<main>` (the text column: 60 characters wide
/// at most and centred, its text growing from 18px to 20px with the page's width) and the footer. The page supplies its
/// tab title, its `<h1>` and what goes in the `<main>`.
///
/// The `<main>` holds either the HTML made from a Markdown file (`html`, written as it is) or
/// other components (`children`); the page gives one of the two.
///
/// `address_prefix` is put in front of the addresses of the stylesheet, of
/// the favicon and of the logo's images: `""` for the pages that write addresses relative to themselves
/// (the rule of ADR-002), the path of the site's address, such as `"/"`, for
/// the "page not found", which the host serves at any depth. The footer's
/// link to the about page follows the same rule.
///
/// `indexing` puts at the end of the `<head>` either the canonical link with the description and
/// the preview tags, or the `noindex` meta.
#[component]
pub fn PageFrame(
    #[prop(into)] title: String,
    heading: Heading,
    #[prop(into)] address_prefix: String,
    footer_licence: FooterLicence,
    indexing: Indexing,
    #[prop(optional, into)] html: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let about_page = format!("{address_prefix}about.html");
    let home_page = if address_prefix.is_empty() {
        "./".to_owned()
    } else {
        address_prefix.clone()
    };
    let icon = format!("{address_prefix}favicon.ico");
    let logo_src = format!("{address_prefix}logo/logo-{LOGO_DEFAULT_WIDTH}.webp");
    let logo_srcset = logo_srcset(&address_prefix);
    let stylesheet = format!("{address_prefix}style.css");
    let indexing_tags = match indexing {
        Indexing::Indexed(page) => match page.kind.clone() {
            PageKind::Website => Either::Left(EitherOf3::A(common_tags(
                page,
                heading.as_str().to_owned(),
                "website",
            ))),
            PageKind::Profile => Either::Left(EitherOf3::B((
                common_tags(page, heading.as_str().to_owned(), "profile"),
                view! {
                    {preview_tag("profile:first_name", "Luca")}
                    {preview_tag("profile:last_name", "Lorenzon")}
                },
            ))),
            PageKind::Article { published } => Either::Left(EitherOf3::C((
                common_tags(page, heading.as_str().to_owned(), "article"),
                preview_tag("article:published_time", published),
            ))),
        },
        Indexing::NoIndex => Either::Right(view! { <meta name="robots" content="noindex" /> }),
    };
    let main = match html {
        Some(html) => view! { <main class=MAIN_CLASSES inner_html=html></main> }.into_any(),
        None => view! { <main class=MAIN_CLASSES>{children.map(|children| children())}</main> }
            .into_any(),
    };
    view! {
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <title>{title}</title>
                <link rel="icon" href=icon />
                <link rel="stylesheet" href=stylesheet />
                {indexing_tags}
            </head>
            <body>
                <div id="top" class="page-fills-screen">
                    <header class="header-two-rows xl:header-one-row">
                        <a
                            href=home_page
                            aria-label="Lvk@73r, home page"
                            class="logo-link"
                        >
                            <img
                                class="logo-icon"
                                src=logo_src
                                srcset=logo_srcset
                                sizes=LOGO_SIZES
                                width=LOGO_DEFAULT_WIDTH
                                height=LOGO_DEFAULT_WIDTH
                                alt=""
                            />
                            <span class="logo-name"><span>"Lvk@73r"</span></span>
                        </a>
                        <h1 class="title-below-line xl:title-beside-logo">{heading.as_str().to_owned()}</h1>
                        <div
                            data-testid="menu-slot"
                            aria-hidden="true"
                            class="reserved-menu-slot"
                        ></div>
                    </header>
                    {main}
                    <footer class="footer-sticky-bottom">
                        <p>
                            "© 2026 Luca Lorenzon · "
                            <a href=about_page class="footer-link">{footer_licence.phrase()}</a>
                        </p>
                    </footer>
                </div>
            </body>
        </html>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The class the `<main>` must carry, written out here so that the test does not depend on
    /// the constant of the component.
    const EXPECTED_CLASSES: &str = "long-text-column";

    fn render(view: impl IntoView + 'static) -> String {
        let owner = Owner::new();
        owner.with(|| view.to_html())
    }

    #[test]
    fn with_html_it_writes_the_main_with_its_class_holding_the_html_unchanged() {
        let html = render(view! {
            <PageFrame
                title="Title"
                heading=Heading::try_from("Heading").unwrap()
                address_prefix=""
                footer_licence=FooterLicence::ReuseWithAttribution
                indexing=Indexing::NoIndex
                html="<h2>Title</h2><p>Text &amp; more</p>"
            />
        });
        let expected = format!(
            "<main class=\"{EXPECTED_CLASSES}\"><h2>Title</h2><p>Text &amp; more</p></main>"
        );
        assert!(html.contains(&expected), "{html}");
    }

    #[test]
    fn with_children_it_writes_the_same_main_holding_the_children() {
        let html = render(view! {
            <PageFrame
                title="Title"
                heading=Heading::try_from("Heading").unwrap()
                address_prefix=""
                footer_licence=FooterLicence::ReuseWithAttribution
                indexing=Indexing::NoIndex
            >
                <p>"Hello"</p>
            </PageFrame>
        });
        let expected = format!("<main class=\"{EXPECTED_CLASSES}\"><p>Hello</p></main>");
        assert!(html.contains(&expected), "{html}");
    }

    fn render_frame() -> String {
        render(view! {
            <PageFrame
                title="Title"
                heading=Heading::try_from("Heading").unwrap()
                address_prefix=""
                footer_licence=FooterLicence::ReuseWithAttribution
                indexing=Indexing::NoIndex
            />
        })
    }

    #[test]
    fn the_logo_is_one_link_to_the_home_page_with_one_name() {
        let html = render_frame();
        assert!(
            html.contains("<a href=\"./\" aria-label=\"Lvk@73r, home page\" class=\"logo-link\">")
        );
        assert_eq!(html.matches("aria-label=\"Lvk@73r, home page\"").count(), 1);
    }

    /// The icon is not announced: the link has its own name (acceptance test 6 of the card
    /// docs/board/cards/review-the-logo.md).
    #[test]
    fn the_icon_has_an_empty_alternative_text() {
        let html = render_frame();
        let img = &html[html.find("<img").expect("the icon")..];
        let img = &img[..img.find('>').unwrap()];
        assert!(img.contains("alt=\"\""), "{img}");
        assert!(!html.contains("role=\"img\""));
        assert_eq!(html.matches("<img").count(), 1);
    }

    #[test]
    fn the_icon_offers_the_four_files_with_their_widths_and_the_size_it_is_drawn_at() {
        let html = render_frame();
        assert!(html.contains(
            "srcset=\"logo/logo-48.webp 48w, logo/logo-64.webp 64w, logo/logo-96.webp 96w, logo/logo-128.webp 128w\""
        ));
        assert!(html.contains("src=\"logo/logo-64.webp\""));
        assert!(html.contains(&format!("sizes=\"{LOGO_SIZES}\"")));
        assert!(html.contains("width=\"64\""));
        assert!(html.contains("height=\"64\""));
    }

    #[test]
    fn the_logo_files_follow_the_address_prefix() {
        let html = render(view! {
            <PageFrame
                title="Title"
                heading=Heading::try_from("Heading").unwrap()
                address_prefix="/"
                footer_licence=FooterLicence::ReuseWithAttribution
                indexing=Indexing::NoIndex
            />
        });
        assert!(html.contains("src=\"/logo/logo-64.webp\""));
        assert!(html.contains("/logo/logo-128.webp 128w"));
    }

    /// The name is text beside the icon, in every width; nothing is drawn in characters
    /// (Orca 51 did not speak the name of a `<pre>`, card
    /// docs/board/cards/logo-name-not-read-by-orca-51.md).
    #[test]
    fn the_name_is_text_and_nothing_is_drawn_in_characters() {
        let html = render_frame();
        assert!(html.contains("<span class=\"logo-name\"><span>Lvk@73r</span></span>"));
        assert!(!html.contains("<pre"));
        assert!(!html.contains("ascii-logo"));
        assert!(!html.contains("art-logo"));
        assert!(!html.contains("text-logo"));
        assert!(!html.contains("____"));
        assert!(!html.contains("&lt;"));
    }

    /// A text of exactly `count` graphemes, each one made of `unit`.
    fn of(count: usize, unit: &str) -> String {
        unit.repeat(count)
    }

    #[test]
    fn the_configuration_file_holds_a_positive_number() {
        assert!(parse_limit(HEADING_MAX_GRAPHEMES_FILE).is_ok());
        assert!(heading_max_graphemes() > 0);
    }

    #[test]
    fn the_limit_is_read_from_a_whole_positive_number_only() {
        assert_eq!(parse_limit(" 12\n"), Ok(12));
        assert!(parse_limit("0").is_err());
        assert!(parse_limit("").is_err());
        assert!(parse_limit("fifty").is_err());
        assert!(parse_limit("-3").is_err());
    }

    #[test]
    fn a_heading_at_the_limit_is_accepted() {
        let text = of(heading_max_graphemes(), "a");
        assert_eq!(Heading::try_from(text.as_str()).unwrap().as_str(), text);
    }

    #[test]
    fn one_grapheme_more_is_refused_with_both_numbers() {
        let max = heading_max_graphemes();
        let text = of(max + 1, "a");
        assert_eq!(
            Heading::try_from(text.as_str()),
            Err(HeadingError::TooLong {
                graphemes: max + 1,
                max
            })
        );
    }

    #[test]
    fn an_empty_or_blank_heading_is_refused() {
        assert_eq!(Heading::try_from(""), Err(HeadingError::Empty));
        assert_eq!(Heading::try_from(" \t\n "), Err(HeadingError::Empty));
    }

    #[test]
    fn the_heading_is_trimmed_and_the_spaces_do_not_count() {
        let text = format!("  {}  ", of(heading_max_graphemes(), "a"));
        assert!(Heading::try_from(text.as_str()).is_ok());
        assert_eq!(Heading::try_from("  Hello ").unwrap().as_str(), "Hello");
    }

    #[test]
    fn a_letter_with_a_combining_accent_counts_once() {
        let text = of(heading_max_graphemes(), "e\u{301}");
        assert!(text.chars().count() > heading_max_graphemes());
        assert!(Heading::try_from(text.as_str()).is_ok());
    }

    #[test]
    fn an_emoji_made_of_several_code_points_counts_once() {
        let family = "\u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F467}\u{200D}\u{1F466}";
        let text = of(heading_max_graphemes(), family);
        assert!(text.chars().count() > heading_max_graphemes());
        assert!(Heading::try_from(text.as_str()).is_ok());
        let one_more = format!("{text}a");
        assert!(matches!(
            Heading::try_from(one_more.as_str()),
            Err(HeadingError::TooLong { .. })
        ));
    }

    #[test]
    fn the_messages_speak_to_the_author_of_an_article() {
        assert_eq!(
            HeadingError::TooLong {
                graphemes: 57,
                max: 50
            }
            .to_string(),
            "the title has 57 characters, at most 50 fit in the page heading"
        );
        assert_eq!(HeadingError::Empty.to_string(), "the title is empty");
    }
}

/// The logo's image files (acceptance tests 8 and 9 of the card
/// docs/board/cards/review-the-logo.md), read as the site publishes them.
#[cfg(test)]
mod logo_files {
    use super::{LOGO_WIDTHS, logo_srcset};
    use std::path::{Path, PathBuf};

    fn folder() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/logo")
    }

    /// The chunks of a RIFF file: their four-letter names and contents.
    fn chunks(file: &[u8]) -> Vec<(String, &[u8])> {
        assert_eq!(&file[0..4], b"RIFF");
        let size = u32::from_le_bytes(file[4..8].try_into().unwrap()) as usize;
        assert_eq!(size + 8, file.len(), "the RIFF size is the file's");
        assert_eq!(&file[8..12], b"WEBP");
        let mut found = Vec::new();
        let mut at = 12;
        while at < file.len() {
            let name = String::from_utf8_lossy(&file[at..at + 4]).into_owned();
            let length = u32::from_le_bytes(file[at + 4..at + 8].try_into().unwrap()) as usize;
            found.push((name, &file[at + 8..at + 8 + length]));
            at += 8 + length + (length & 1);
        }
        assert_eq!(at, file.len());
        found
    }

    #[test]
    fn the_folder_holds_exactly_the_four_icons_and_their_notice() {
        let mut names: Vec<String> = std::fs::read_dir(folder())
            .unwrap()
            .map(|entry| entry.unwrap().file_name().into_string().unwrap())
            .collect();
        names.sort();
        assert_eq!(
            names,
            [
                "README.md",
                "logo-128.webp",
                "logo-48.webp",
                "logo-64.webp",
                "logo-96.webp"
            ]
        );
    }

    #[test]
    fn the_srcset_names_exactly_those_files() {
        let srcset = logo_srcset("");
        let named: Vec<String> = srcset
            .split(", ")
            .map(|entry| entry.split(' ').next().unwrap().to_owned())
            .collect();
        let files: Vec<String> = LOGO_WIDTHS
            .iter()
            .map(|width| format!("logo/logo-{width}.webp"))
            .collect();
        assert_eq!(named, files);
        for name in named {
            assert!(
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("assets")
                    .join(&name)
                    .is_file(),
                "{name}"
            );
        }
    }

    #[test]
    fn each_file_is_a_webp_of_its_width_with_transparency() {
        for width in LOGO_WIDTHS {
            let bytes = std::fs::read(folder().join(format!("logo-{width}.webp"))).unwrap();
            let chunks = chunks(&bytes);
            let (name, extended) = &chunks[0];
            assert_eq!(name, "VP8X", "logo-{width}.webp");
            assert_eq!(extended.len(), 10);
            assert!(extended[0] & 0x10 != 0, "the alpha flag, logo-{width}.webp");
            let canvas = |at: usize| {
                u32::from_le_bytes([extended[at], extended[at + 1], extended[at + 2], 0]) + 1
            };
            assert_eq!((canvas(4), canvas(7)), (width, width), "logo-{width}.webp");
        }
    }

    #[test]
    fn each_file_holds_the_notice_in_its_metadata() {
        for width in LOGO_WIDTHS {
            let bytes = std::fs::read(folder().join(format!("logo-{width}.webp"))).unwrap();
            let chunks = chunks(&bytes);
            let xmp = chunks
                .iter()
                .find(|(name, _)| name == "XMP ")
                .unwrap_or_else(|| panic!("logo-{width}.webp has no XMP chunk"));
            let text = std::str::from_utf8(xmp.1).expect("the XMP packet is UTF-8");
            assert!(
                text.contains("\u{a9} 2011 Gioia Greifenberg. All rights reserved."),
                "logo-{width}.webp"
            );
        }
    }
}
