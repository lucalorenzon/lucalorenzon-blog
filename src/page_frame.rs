use leptos::either::{Either, EitherOf3};
use leptos::prelude::*;

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
    pub description: &'static str,
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

/// The tags every indexed page has, whatever its kind.
fn common_tags(page: IndexedPage, heading: &'static str, og_type: &'static str) -> impl IntoView {
    view! {
        <link rel="canonical" href=page.canonical.clone() />
        <meta name="description" content=page.description />
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
    title: &'static str,
    heading: &'static str,
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
            PageKind::Website => Either::Left(EitherOf3::A(common_tags(page, heading, "website"))),
            PageKind::Profile => Either::Left(EitherOf3::B((
                common_tags(page, heading, "profile"),
                view! {
                    {preview_tag("profile:first_name", "Luca")}
                    {preview_tag("profile:last_name", "Lorenzon")}
                },
            ))),
            PageKind::Article { published } => Either::Left(EitherOf3::C((
                common_tags(page, heading, "article"),
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
                        <h1 class="title-below-line md:title-below-line-large xl:title-beside-logo">{heading}</h1>
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
