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

#[derive(Debug, thiserror::Error)]
#[error("no content dates: {0}")]
struct MissingContentDates(InputError);

/// What the build does with a problem it can live with. In the pipeline (`is_ci`) what is
/// built is what gets published, so the problem stops the build; anywhere else the build goes
/// on.
fn tolerate<E>(problem: E, is_ci: bool) -> Result<E, E> {
    if is_ci { Err(problem) } else { Ok(problem) }
}

/// A warning: the problem, then what the build does instead.
fn warning(problem: &impl std::fmt::Display, instead: &str) -> String {
    format!("{problem}; {instead}")
}

/// The list of the articles left out, shown at the end of the build.
fn skipped_summary(skipped: &article_page::ArticleProblems) -> String {
    let lines: Vec<String> = skipped
        .0
        .iter()
        .map(|error| format!("  - {error}"))
        .collect();
    format!(
        "{} article(s) skipped, not in the site:\n{}",
        skipped.0.len(),
        lines.join("\n")
    )
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
    let loaded = article_page::load_articles(&article_page::ARTICLE_FILES);
    let skipped = match loaded.problems {
        Some(problems) => Some(tolerate(problems, is_ci)?),
        None => None,
    };
    for error in skipped.iter().flat_map(|problems| &problems.0) {
        eprintln!(
            "warning: {}",
            warning(error, "the article is left out of the site")
        );
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
    let dates = match read_input("CONTENT_DATES_FILE") {
        Ok(text) => Some(sitemap::ContentDates::parse(&text)?),
        Err(problem) => {
            let problem = tolerate(MissingContentDates(problem), is_ci)?;
            eprintln!(
                "warning: {}",
                warning(&problem, "the sitemap has no <lastmod>")
            );
            None
        }
    };
    let sitemap = sitemap::build_sitemap(&pages, &address, dates.as_ref());
    if let Some(missing) = sitemap.missing {
        let missing = tolerate(missing, is_ci)?;
        eprintln!(
            "warning: {}",
            warning(&missing, "written without <lastmod>")
        );
    }
    fs::write(out_dir.join(sitemap::SITEMAP_FILE), sitemap.xml)?;
    let blocked = match read_input("BLOCKED_ROBOTS_FILE") {
        Ok(text) => robots_txt::parse_blocked_robots(&text)?,
        Err(problem) => {
            let problem = tolerate(MissingBlockedRobots(problem), is_ci)?;
            eprintln!(
                "warning: {}",
                warning(&problem, "the robots.txt blocks only the training signal")
            );
            Vec::new()
        }
    };
    fs::write(
        out_dir.join(robots_txt::ROBOTS_FILE),
        robots_txt::robots_txt(&address, &blocked),
    )?;
    write_code_classes()?;
    if let Some(skipped) = &skipped {
        eprintln!("{}", skipped_summary(skipped));
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
        let error = tolerate("it broke".to_owned(), true).unwrap_err();
        assert_eq!(error, "it broke");
    }

    #[test]
    fn a_problem_is_tolerated_elsewhere() {
        let problem = tolerate("it broke".to_owned(), false).unwrap();
        assert_eq!(problem, "it broke");
    }

    #[test]
    fn a_warning_is_the_problem_then_what_the_build_does_instead() {
        assert_eq!(warning(&"it broke", "going on"), "it broke; going on");
    }

    #[test]
    fn the_summary_lists_the_articles_left_out() {
        let skipped = article_page::ArticleProblems(vec![
            article_page::ArticleError {
                file: "content/articles/a.md",
                error: article_page::FrontMatterError::Missing,
            },
            article_page::ArticleError {
                file: "content/articles/b.md",
                error: article_page::FrontMatterError::Unclosed,
            },
        ]);
        let summary = skipped_summary(&skipped);
        assert_eq!(
            summary,
            "2 article(s) skipped, not in the site:\n  \
             - content/articles/a.md: the file does not start with a front matter (a line `---`)\n  \
             - content/articles/b.md: the front matter has no closing line `---`"
        );
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

    #[test]
    fn the_missing_dates_error_reads_as_before() {
        let error = MissingContentDates(InputError::NotSet {
            variable: "CONTENT_DATES_FILE",
        });
        assert_eq!(
            error.to_string(),
            "no content dates: CONTENT_DATES_FILE is not set"
        );
    }
}
