mod about_page;
mod article_page;
mod home_page;
mod licence_page;
mod markdown;
mod not_found_page;
mod page_frame;
mod site_address;

use site_address::SiteAddress;
use std::{env, error::Error, fs, path::PathBuf, process::ExitCode};

/// The site's full address when `SITE_ADDRESS` is not set (ADR-002).
const DEFAULT_SITE_ADDRESS: &str = "https://www.lucalorenzon.com/";

fn build() -> Result<(), Box<dyn Error>> {
    let out_dir = env::args_os()
        .nth(1)
        .map_or_else(|| PathBuf::from("target/site"), PathBuf::from);

    let configured = env::var("SITE_ADDRESS").unwrap_or_default();
    let address = SiteAddress::parse(if configured.trim().is_empty() {
        DEFAULT_SITE_ADDRESS
    } else {
        &configured
    })?;

    fs::create_dir_all(&out_dir)?;
    fs::write(out_dir.join("index.html"), home_page::render_home_page())?;
    fs::write(out_dir.join("about.html"), about_page::render_about_page())?;
    fs::create_dir_all(out_dir.join("articles"))?;
    fs::write(
        out_dir.join("articles/after-save-event.html"),
        article_page::render_after_save_event_page(),
    )?;
    fs::write(
        out_dir.join("articles/how-to-generate-hybris-pk-values-without-hybris.html"),
        article_page::render_hybris_pk_page(),
    )?;
    fs::write(
        out_dir.join("licence.html"),
        licence_page::render_licence_page(),
    )?;
    fs::write(
        out_dir.join("404.html"),
        not_found_page::render_not_found_page(&address),
    )?;
    Ok(())
}

fn main() -> ExitCode {
    match build() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}
