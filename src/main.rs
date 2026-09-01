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
    use blog_start::app::{content_repo_path, shell};
    use leptos::prelude::*;
    use leptos_actix::generate_route_list_with_ssg;

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;

    // Minimal fix (2026-09-01, Luca): copy the content repo's own images
    // into the generated site so ARTICLE-PAGE/LISTING-PAGE's <img> tags
    // (view_model::effective_image_path, rooted at ARTICLE_IMAGES_BASE_PATH)
    // actually resolve to a real file — confirmed broken otherwise via a
    // real cargo leptos build. Shares a directory with this app's own
    // fallback SVG asset; a real asset-storage design (its own location,
    // no collision risk) is a future epic, not solved here.
    let content_images_dir =
        std::path::Path::new(&content_repo_path()).join("assets/images");
    // `/images`, not `/assets/images`: cargo-leptos flattens `assets-dir`
    // straight into `site-root` (confirmed against a real build's output,
    // not assumed) — same real location this app's own fallback SVG
    // already lives at.
    let site_images_dir = std::path::Path::new(leptos_options.site_root.as_ref()).join("images");
    if content_images_dir.is_dir() {
        std::fs::create_dir_all(&site_images_dir)?;
        for entry in std::fs::read_dir(&content_images_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                std::fs::copy(entry.path(), site_images_dir.join(entry.file_name()))?;
            }
        }
    }

    // `shell(...)`, not bare `<App/>`: generation renders whatever this
    // closure returns, and leptos_meta needs a literal `<head>` (from the
    // shell) to inject `<Title>`/`<Meta>` into — verified empirically
    // (a bare `<App/>` panics with "without a </head> tag"), matching the
    // official examples/static_routing pattern.
    let (_routes, static_route_generator) = generate_route_list_with_ssg({
        let leptos_options = leptos_options.clone();
        move || shell(leptos_options.clone())
    });

    static_route_generator.generate(&leptos_options).await;

    // HomePage is generated at /_home (see app.rs's comment on that
    // route — the real root path can't go through this generation
    // mechanism, a leptos_actix limitation). Move its file into place as
    // the real index.html.
    std::fs::rename(
        format!("{}/_home.html", leptos_options.site_root),
        format!("{}/index.html", leptos_options.site_root),
    )?;

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
