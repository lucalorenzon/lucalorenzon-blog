use leptos::prelude::*;

/// The ASCII-art logo (narrow version of the old site's `AsciiArtLogo`).
pub const LOGO: &str = r#"    __        __   ______  __________
   / / _   __/ /__/ ____ \/__  /__  /_____
  / / | | / / //_/ / __ `/  / / /_ </ ___/
 / /__| |/ / ,< / / /_/ /  / /___/ / /
/_____/___/_/\_|\ \__,_/  /_//____/_/
                 \____/"#;

/// The licence phrase a page shows in its footer, as the link to the licence
/// page. Each page chooses its own; the frame renders it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FooterLicence {
    /// Everything but the curriculum: reuse is allowed, citing the author.
    ReuseWithAttribution,
    /// The curriculum, shown on the home page: copying yes, changing no.
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

/// The frame every page shares: the `<html>` element with its `<head>`, the
/// header (logo, title, reserved menu slot) and the footer. The page supplies
/// its tab title, its `<h1>` and its `<main>` element.
///
/// `address_prefix` is put in front of the addresses of the stylesheet and of
/// the icon: `""` for the pages that write addresses relative to themselves
/// (the rule of ADR-002), the path of the site's address, such as `"/"`, for
/// the "page not found", which the host serves at any depth. The footer's
/// link to the licence page follows the same rule.
#[component]
pub fn PageFrame(
    title: &'static str,
    heading: &'static str,
    #[prop(into)] address_prefix: String,
    footer_licence: FooterLicence,
    children: Children,
) -> impl IntoView {
    let licence_page = format!("{address_prefix}licence.html");
    let home_page = if address_prefix.is_empty() {
        "./".to_owned()
    } else {
        address_prefix.clone()
    };
    let icon = format!("{address_prefix}favicon.ico");
    let stylesheet = format!("{address_prefix}style.css");
    view! {
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <title>{title}</title>
                <link rel="icon" href=icon />
                <link rel="stylesheet" href=stylesheet />
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
                    <footer class="footer-sticky-bottom">
                        <p>
                            "© 2026 Luca Lorenzon · "
                            <a href=licence_page class="footer-link">{footer_licence.phrase()}</a>
                        </p>
                    </footer>
                </div>
            </body>
        </html>
    }
}
