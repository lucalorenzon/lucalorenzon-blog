#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::prelude::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use blog_start::app::*;
    use leptos::prelude::*;
    use leptos_actix::{LeptosRoutes, generate_route_list};

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(|| view! { <App/> });

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", site_root.to_string()))
            // serve the favicon from /favicon.ico
            .service(favicon)
            .leptos_routes(routes.to_owned(), {
                let leptos_options = leptos_options.to_owned();
                move || shell(leptos_options.clone())
            })
            .app_data(web::Data::new(leptos_options.to_owned()))
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // No client-side main function: the wasm bundle ships via
    // wasm-bindgen's own entrypoint (lib.rs's `hydrate()`), invoked
    // directly by the generated JS glue, not through this binary's main.
}
