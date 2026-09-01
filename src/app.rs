use crate::adapters::secondary::content_source::filesystem::FilesystemContentSource;
use crate::domain::ports::ContentSource;
use crate::pages::{ArticlePage, HomePage, ListingPage};
use leptos::prelude::*;
use leptos_meta::{Body, Html, Meta, MetaTags, Stylesheet, provide_meta_context};
use leptos_router::components::{FlatRoutes, Route, Router};
use leptos_router::static_routes::StaticRoute;
use leptos_router::{SsrMode, path};

/// The document shell rendered around `<App/>` during server-side rendering.
///
/// `leptos_meta`'s SSR pipeline patches head content and `<html>`/`<body>`
/// attributes into this literal markup, so `<head>` and `<MetaTags/>` must
/// be present here even though the actual tags are populated by `<Meta>`,
/// `<Title>`, `<Html>` and `<Body>` further down in `<App/>`.
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options islands=true islands_router=true />
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

/// The content repo checkout `FilesystemContentSource` reads from — no
/// default, no silent fallback: a missing value aborts the build
/// immediately rather than reading from an unintended path. `pub`: also
/// used by `main.rs` to locate `images_dir` for the minimal image-copy
/// fix (2026-09-01).
pub fn content_repo_path() -> String {
    std::env::var("CONTENT_REPO_PATH")
        .expect("CONTENT_REPO_PATH must be set to the dedicated content repo's checkout path")
}

/// Every published article's slug, for ARTICLE-PAGE's `prerender_params` —
/// one `StaticRoute` per published slug, per `ContentSource`'s port
/// contract (S001/S003).
async fn published_slugs() -> Vec<String> {
    FilesystemContentSource::new(content_repo_path())
        .list_published()
        .expect(
            "a malformed article must abort the build, not silently exclude it \
             — AT-EP-001-UC-001-S003",
        )
        .iter()
        .map(|article| article.slug().as_str().to_string())
        .collect()
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_context(FilesystemContentSource::new(content_repo_path()));

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Html {..} lang="en" dir="ltr" class="dark antialiased h-full" />
        <Meta name="viewport" content="width=device-width,initial-scale=1.0" />
        <Stylesheet id="leptos" href="/pkg/blog_start.css"/>
        <Body {..} class="bg-white text-blue dark:bg-black dark:text-blue-100 flex flex-col h-screen overflow-y-auto" />
        <Router>
                <FlatRoutes fallback=NotFound>
                    // Not `path!("")`/`path!("/")`: leptos_actix 0.8.7's
                    // StaticRouteGenerator fabricates a mock HTTP request per
                    // route to render it (test::TestRequest::with_uri), and
                    // the root path resolves to an empty string internally —
                    // an empty string isn't a valid URI, so generation panics
                    // (InvalidUri(Empty)), confirmed empirically against both
                    // spellings; this doesn't affect live request handling,
                    // only this build-time mechanism, which the old
                    // always-on server never exercised. `/_home` is a
                    // literal segment so the resolved path is never empty;
                    // main.rs renames the resulting `_home.html` to
                    // `index.html` after generation — not a real page to
                    // visit directly.
                    <Route
                        path=path!("/_home")
                        view=HomePage
                        ssr=SsrMode::Static(StaticRoute::new())
                    />
                    <Route
                        path=path!("/articles/:slug")
                        view=ArticlePage
                        ssr=SsrMode::Static(
                            StaticRoute::new().prerender_params(|| async move {
                                [("slug".into(), published_slugs().await)].into_iter().collect()
                            }),
                        )
                    />
                    <Route
                        path=path!("/articles/page/:page")
                        view=ListingPage
                        ssr=SsrMode::Static(
                            // `:page` reserved for future pagination (story's Open
                            // questions) — not read yet, always "1" for now.
                            StaticRoute::new().prerender_params(|| async move {
                                [("page".into(), vec!["1".to_string()])].into_iter().collect()
                            }),
                        )
                    />
                    <Route path=path!("/*any") view=NotFound/>
                </FlatRoutes>
        </Router>
    }
}

/// 404 - Not Found. Not included in the SSG-generated static routes (a
/// wildcard path can't be enumerated via `prerender_params`) — writing a
/// real `404.html` for the deployed static site is [[EP-001-UC-001-S005]]'s
/// concern (deploy), not this story's.
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
