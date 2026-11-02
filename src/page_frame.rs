use leptos::prelude::*;

/// The ASCII-art logo (narrow version of the old site's `AsciiArtLogo`).
pub const LOGO: &str = r#"    __        __   ______  __________
   / / _   __/ /__/ ____ \/__  /__  /_____
  / / | | / / //_/ / __ `/  / / /_ </ ___/
 / /__| |/ / ,< / / /_/ /  / /___/ / /
/_____/___/_/\_|\ \__,_/  /_//____/_/
                 \____/"#;

/// The frame every page shares: the `<html>` element with its `<head>`, the
/// header (logo, title, reserved menu slot) and the footer. The page supplies
/// its tab title, its `<h1>` and its `<main>` element.
///
/// `address_prefix` is put in front of the addresses of the stylesheet and of
/// the icon: `""` for the pages that write addresses relative to themselves
/// (the rule of ADR-002), the path of the site's address, such as `"/"`, for
/// the "page not found", which the host serves at any depth.
#[component]
pub fn PageFrame(
    title: &'static str,
    heading: &'static str,
    #[prop(into)] address_prefix: String,
    children: Children,
) -> impl IntoView {
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
                        <div class="text-logo-large md:not-shown">"Lvk@73r"</div>
                        <div
                            role="img"
                            aria-label="Lvk@73r"
                            class="not-shown md:art-logo-left"
                        >{LOGO}</div>
                        <h1 class="title-below-line md:title-below-line-large xl:title-beside-logo">{heading}</h1>
                        <div
                            data-testid="menu-slot"
                            aria-hidden="true"
                            class="reserved-menu-slot"
                        ></div>
                    </header>
                    {children()}
                    <footer class="footer-sticky-bottom">
                        <p>"© Luca Lorenzon 2026"</p>
                    </footer>
                </div>
            </body>
        </html>
    }
}
