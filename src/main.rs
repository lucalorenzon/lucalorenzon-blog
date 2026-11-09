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

/// Why a generated input, named by an environment variable, could not be had.
#[derive(Debug, thiserror::Error)]
enum InputError {
    #[error("{variable} is not set")]
    NotSet { variable: &'static str },
    #[error("cannot read `{path}`: {reason}")]
    Unreadable {
        path: String,
        reason: std::io::Error,
    },
}

/// The text of the file the environment variable points to. Unset or empty means not set.
fn read_input(variable: &'static str) -> Result<String, InputError> {
    match env::var(variable) {
        Ok(path) if !path.is_empty() => {
            fs::read_to_string(&path).map_err(|reason| InputError::Unreadable { path, reason })
        }
        _ => Err(InputError::NotSet { variable }),
    }
}

#[derive(Debug, thiserror::Error)]
#[error("no blocked robots list: {0}")]
struct MissingBlockedRobots(InputError);

/// What the build does with a problem it can live with. In the pipeline (`is_ci`) what is
/// built is what gets published, so the problem stops the build; anywhere else the build goes
/// on, and the warning says the problem and what the build does instead.
fn tolerate<E>(problem: E, is_ci: bool, instead: &str) -> Result<String, E>
where
    E: std::fmt::Display,
{
    if is_ci {
        Err(problem)
    } else {
        Ok(format!("{problem}; {instead}"))
    }
}

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
    let dates_file = read_input("CONTENT_DATES_FILE").map_err(|error| error.to_string());
    let sitemap = sitemap::build_sitemap(&pages, &address, dates_file, is_ci)?;
    for warning in &sitemap.warnings {
        eprintln!("warning: {warning}");
    }
    fs::write(out_dir.join(sitemap::SITEMAP_FILE), sitemap.xml)?;
    let blocked = match read_input("BLOCKED_ROBOTS_FILE") {
        Ok(text) => robots_txt::parse_blocked_robots(&text)?,
        Err(problem) => {
            let warning = tolerate(
                MissingBlockedRobots(problem),
                is_ci,
                "the robots.txt blocks only the training signal",
            )?;
            eprintln!("warning: {warning}");
            Vec::new()
        }
    };
    fs::write(
        out_dir.join(robots_txt::ROBOTS_FILE),
        robots_txt::robots_txt(&address, &blocked),
    )?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_problem_stops_the_build_in_the_pipeline() {
        let error = tolerate("it broke".to_owned(), true, "going on").unwrap_err();
        assert_eq!(error, "it broke");
    }

    #[test]
    fn a_problem_is_a_warning_elsewhere() {
        let warning = tolerate("it broke".to_owned(), false, "going on").unwrap();
        assert_eq!(warning, "it broke; going on");
    }

    #[test]
    fn an_unset_input_is_not_set() {
        // A name no other test or program uses, so the process environment is not raced.
        let error = read_input("LUCALORENZON_BLOG_TEST_UNSET_INPUT").unwrap_err();
        assert_eq!(
            error.to_string(),
            "LUCALORENZON_BLOG_TEST_UNSET_INPUT is not set"
        );
    }

    #[test]
    fn the_missing_list_error_reads_as_before() {
        let error = MissingBlockedRobots(InputError::NotSet {
            variable: "BLOCKED_ROBOTS_FILE",
        });
        assert_eq!(
            error.to_string(),
            "no blocked robots list: BLOCKED_ROBOTS_FILE is not set"
        );
    }
}
