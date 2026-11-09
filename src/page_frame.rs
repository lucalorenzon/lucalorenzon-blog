use leptos::either::{Either, EitherOf3};
use leptos::prelude::*;
use std::sync::LazyLock;
use unicode_segmentation::UnicodeSegmentation;

/// The ASCII-art logo (narrow version of the old site's `AsciiArtLogo`).
pub const LOGO: &str = r#"    __        __   ______  __________
   / / _   __/ /__/ ____ \/__  /__  /_____
  / / | | / / //_/ / __ `/  / / /_ </ ___/
 / /__| |/ / ,< / / /_/ /  / /___/ / /
/_____/___/_/\_|\ \__,_/  /_//____/_/
                 \____/"#;

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
/// header (logo, title, reserved menu slot) and the footer. The page supplies
/// its tab title, its `<h1>` and its `<main>` element.
///
/// `address_prefix` is put in front of the addresses of the stylesheet and of
/// the icon: `""` for the pages that write addresses relative to themselves
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
    children: Children,
) -> impl IntoView {
    let about_page = format!("{address_prefix}about.html");
    let home_page = if address_prefix.is_empty() {
        "./".to_owned()
    } else {
        address_prefix.clone()
    };
    let icon = format!("{address_prefix}favicon.ico");
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
                <div class="page-fills-screen">
                    <header class="header-two-rows xl:header-one-row">
                        <a
                            href=home_page
                            aria-label="Lvk@73r, home page"
                            class="logo-link"
                        >
                            <div class="text-logo-large md:not-shown">"Lvk@73r"</div>
                            <div
                                data-testid="ascii-logo"
                                aria-hidden="true"
                                class="not-shown md:art-logo-left"
                            >{LOGO}</div>
                        </a>
                        <h1 class="title-below-line md:title-below-line-large xl:title-beside-logo">{heading.as_str().to_owned()}</h1>
                        <div
                            data-testid="menu-slot"
                            aria-hidden="true"
                            class="reserved-menu-slot"
                        ></div>
                    </header>
                    {children()}
                    <footer class="footer-sticky-bottom md:footer-text-large">
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
