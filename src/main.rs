mod about_page;
mod article_page;
mod curriculum_page;
mod home_page;
mod licence_page;
mod literata_widths;
mod markdown;
mod not_found_page;
mod page_frame;
mod robots_txt;
mod site_address;
mod sitemap;
mod table_widths;

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

    let is_ci = env::var("CI").is_ok_and(|value| !value.is_empty());
    let loaded = article_page::settle(
        article_page::load_articles(&article_page::ARTICLE_FILES),
        is_ci,
    )?;
    for warning in &loaded.warnings {
        eprintln!("warning: {warning}");
    }

    fs::create_dir_all(&out_dir)?;
    let pages = sitemap::pages(&address, &loaded.articles);
    for page in &pages {
        let path = out_dir.join(page.file);
        if let Some(folder) = path.parent() {
            fs::create_dir_all(folder)?;
        }
        fs::write(path, &page.html)?;
    }
    let dates_file = match env::var("CONTENT_DATES_FILE") {
        Ok(path) if !path.is_empty() => {
            fs::read_to_string(&path).map_err(|error| format!("cannot read `{path}`: {error}"))
        }
        _ => Err("CONTENT_DATES_FILE is not set".to_owned()),
    };
    let sitemap = sitemap::build_sitemap(&pages, &address, dates_file, is_ci)?;
    for warning in &sitemap.warnings {
        eprintln!("warning: {warning}");
    }
    fs::write(out_dir.join(sitemap::SITEMAP_FILE), sitemap.xml)?;
    let blocked_robots = match env::var("BLOCKED_ROBOTS_FILE") {
        Ok(path) if !path.is_empty() => {
            fs::read_to_string(&path).map_err(|error| format!("cannot read `{path}`: {error}"))
        }
        _ => Err("BLOCKED_ROBOTS_FILE is not set".to_owned()),
    };
    let robots = robots_txt::build_robots_txt(&address, blocked_robots, is_ci)?;
    for warning in &robots.warnings {
        eprintln!("warning: {warning}");
    }
    fs::write(out_dir.join(robots_txt::ROBOTS_FILE), robots.text)?;
    write_code_classes()?;
    if let Some(summary) = &loaded.summary {
        eprintln!("{summary}");
    }
    Ok(())
}

/// Where the build lists the class names of the code and table components, for Tailwind
/// (style/input.css).
/// Outside the published site on purpose.
const CODE_CLASSES_FILE: &str = "target/generated/code-classes.txt";

/// Tailwind must not scan the generated HTML: words of the articles ("table", "block"...) would
/// become unrelated utilities. It scans this exact list instead.
fn write_code_classes() -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(CODE_CLASSES_FILE);
    if let Some(folder) = path.parent() {
        fs::create_dir_all(folder)?;
    }
    let mut list = markdown::take_generated_classes()
        .into_iter()
        .collect::<Vec<_>>()
        .join("\n");
    list.push('\n');
    fs::write(path, list)?;
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
