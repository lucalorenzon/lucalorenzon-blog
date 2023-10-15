use leptos::*;
use leptos_meta::{Html, Meta, Stylesheet, Body, Title, provide_meta_context};
use leptos_router::{Route, Router, Routes};

use crate::layout::Layout;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Html lang="en" dir="ltr" class="dark antialiased h-full" />
        <Meta name="viewport" content="width=device-width,initial-scale=1.0" />
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <Body class="bg-white text-blue dark:bg-black dark:text-blue-100 flex flex-col min-h-full" />
        // content for this welcome page
        <Router>
                <Routes>
                    <Route path="" view=HomePage/>
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
            <article class="h-[100vh] overflow-hidden overflow-y-auto items-center justify-center">
                <div class="bg-[url('/assets/images/ostia_sea_top_image.webp')] bg-cover pt-[100px] h-[100vh] bg-fixed" />
                <h1 class="-transform-y-px">Pinned Article: Lorem Ipsum</h1>
                <p>sdfs sdf lkjls j fllkjlkjlkjkljklsdf sdf lkjljlksdf sdflklkjsdf  lkjlkjl j sd fsdf </p>
            </article>
            <ul class="grid gap-10 ">
                <li>
                    <h2>articolo 1</h2>
                </li>
                <li>
                    <h2>articolo 2</h2>
                </li>
                <li>
                    <h2>articolo 3</h2>
                </li>
                <li>
                    <h2>articolo 4</h2>
                </li>
                <li>
                    <h2>articolo 5</h2>
                </li>
                <li>
                     <h2>articolo 6</h2>
                </li>
            </ul>
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
