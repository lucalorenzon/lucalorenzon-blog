use leptos::*;
use leptos_meta::{Html, Meta, Stylesheet, Body, Title, provide_meta_context};
use leptos_router::{Router, Routes, Route};
use crate::layout::{Layout, ArticleTitle, ArticleAbstract, ArticleContent};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Html lang="en" dir="ltr" class="dark antialiased h-full" />
        <Meta name="viewport" content="width=device-width,initial-scale=1.0" />
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <Body class="bg-white text-blue dark:bg-black dark:text-blue-100 flex flex-col h-screen overflow-y-auto" />
        // content for this welcome page
        <Router>
                <Routes>
                    <Route path="" view=HomePage />
                    <Route path="/*any" view=NotFound/>
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
