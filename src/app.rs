use crate::layout::{ArticleAbstract, ArticleContent, ArticleTitle, Layout};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Body, Html, Meta, MetaTags, Stylesheet, Title};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

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

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Html {..} lang="en" dir="ltr" class="dark antialiased h-full" />
        <Meta name="viewport" content="width=device-width,initial-scale=1.0" />
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <Body {..} class="bg-white text-blue dark:bg-black dark:text-blue-100 flex flex-col h-screen overflow-y-auto" />
        // content for this welcome page
        <Router>
                <Routes fallback=NotFound>
                    <Route path=path!("") view=HomePage />
                    <Route path=path!("/*any") view=NotFound/>
                </Routes>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        // sets the document title
        <Title text="Lvk@73r Blog | Homepage" />
        <Layout>
            <ArticleTitle>ARTICLE TITLE</ArticleTitle>
            <ArticleAbstract>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer maximus nibh sit amet mattis elementum. Fusce vel maximus orci, ut lobortis tellus. Pellentesque malesuada quis arcu eu rutrum.</ArticleAbstract>
            <ArticleContent>
                <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer maximus nibh sit amet mattis elementum. Fusce vel maximus orci, ut lobortis tellus. Pellentesque malesuada quis arcu eu rutrum. Vivamus imperdiet massa et mollis ullamcorper. Phasellus semper, augue at condimentum accumsan, nisl velit volutpat risus, et feugiat libero mauris id mi. Nullam a efficitur nunc, id ultricies turpis. Vestibulum vestibulum nulla vel tempus iaculis. Donec semper facilisis ultrices. Suspendisse tincidunt sagittis lectus ut efficitur. Praesent ac accumsan neque.</p>
                <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer maximus nibh sit amet mattis elementum. Fusce vel maximus orci, ut lobortis tellus. Pellentesque malesuada quis arcu eu rutrum. Vivamus imperdiet massa et mollis ullamcorper. Phasellus semper, augue at condimentum accumsan, nisl velit volutpat risus, et feugiat libero mauris id mi. Nullam a efficitur nunc, id ultricies turpis. Vestibulum vestibulum nulla vel tempus iaculis. Donec semper facilisis ultrices. Suspendisse tincidunt sagittis lectus ut efficitur. Praesent ac accumsan neque.</p>
            </ArticleContent>
        </Layout>
    }
}

/// 404 - Not Found
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
