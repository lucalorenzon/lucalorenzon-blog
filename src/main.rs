/// Dev-only static file server (used when `cargo leptos watch` sets
/// `LEPTOS_WATCH`, see `main()` below): serves whatever `generate()` just
/// wrote under `site_root`, with the same "clean URL" resolution GitHub
/// Pages applies in production (`/articles/hello-world` → the file
/// `articles/hello-world.html`) — no live rendering, exact same output a
/// real static host would serve.
#[cfg(feature = "ssr")]
async fn serve_generated_site(
    req: actix_web::HttpRequest,
    site_root: actix_web::web::Data<std::sync::Arc<str>>,
) -> actix_web::Result<actix_files::NamedFile> {
    let path = req.path().trim_start_matches('/');
    let candidate = if path.is_empty() {
        "index.html".to_string()
    } else {
        format!("{path}.html")
    };

    for name in [path.to_string(), candidate] {
        if name.is_empty() {
            continue;
        }
        let full_path = std::path::Path::new(site_root.get_ref().as_ref()).join(&name);
        if let Ok(file) = actix_files::NamedFile::open(&full_path) {
            return Ok(file);
        }
    }

    Err(actix_web::error::ErrorNotFound("not found"))
}

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use blog_start::app::App;
    use leptos::prelude::*;
    use leptos_actix::generate_route_list_with_ssg;

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;

    let (_routes, static_route_generator) =
        generate_route_list_with_ssg(|| view! { <App/> });

    static_route_generator.generate(&leptos_options).await;

    // Production/CI build path: generation above already wrote every real
    // .html file under site_root — no HttpServer, no live rendering, per
    // ADR-004. The process exits here.
    if std::env::var("LEPTOS_WATCH").is_err() {
        return Ok(());
    }

    // Dev convenience only (`cargo leptos watch` sets LEPTOS_WATCH): serve
    // the just-generated static output so the existing hot-reload dev loop
    // keeps working, without reintroducing live rendering.
    use actix_web::{App as ActixApp, HttpServer, web};

    let addr = leptos_options.site_addr;
    let site_root = leptos_options.site_root.clone();

    HttpServer::new(move || {
        ActixApp::new()
            .app_data(web::Data::new(site_root.clone()))
            .default_service(web::route().to(serve_generated_site))
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
