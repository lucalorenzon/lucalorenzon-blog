//! The site's fonts: which font plays each of the three roles, and how each font is made of
//! files. Both are written in `config/`, read by the build, and written out for the stylesheet
//! to `target/generated/fonts.css`, which `style/input.css` imports.
//!
//! The stylesheet cannot read a configuration file by itself, and a font written there would
//! be a second place to change. So the stylesheet names no font: the roles' values (`@theme`)
//! and the `@font-face` rules both come from here, and a change of font is a change of one
//! name in `config/fonts.txt`.
//!
//! - `config/fonts.txt` has one line `role=folder` for each role (`text`, `code`, `logo`),
//!   `folder` being the font's folder in `assets/fonts/`.
//! - `config/fonts/<folder>.txt` describes that font, one `key=value` per line. The keys
//!   without a prefix are for the whole font: `family` (the CSS family name), `fallback` (a
//!   generic family), `display` (`font-display`) and, optional, `size` (the font's native size
//!   in pixels, a positive whole number: the fonts page draws its samples at that size, and a
//!   font without one keeps the page's size). The keys with a prefix `<label>.` describe
//!   one file, all four required: `file`, `name` (the label the fonts page will show), `style`
//!   and `weight`. Style and weight are written, not guessed from the label, so a new weight
//!   needs no change to the build.
//! - Blank lines and lines that start with `#` are ignored.
//!
//! The fonts page shows one sample for each file of each font. The build writes, for each file
//! of each role's font, the variant that draws it (`sample-<role>-<style>-<weight>`, for example
//! `sample-text-italic-700`), and lists the names for Tailwind with `@source inline(...)`, since
//! the page's HTML is generated and Tailwind could not find them by scanning the sources.
//!
//! The parsing is pure (text in, value or error out) so every refusal has a test that needs no
//! disk; the build program reads the files and passes their text in.
//!
//! About `display`: the text and the code use `swap`, so that they are readable at once in a
//! fallback font, even before the font arrives; the logo's pixel font uses `block`, because a
//! fallback would flash before the pixel font replaced it.

use std::fmt::{self, Write};

/// The three jobs a font can have on the site.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Text,
    Code,
    Logo,
}

impl Role {
    pub const ALL: [Role; 3] = [Role::Text, Role::Code, Role::Logo];

    /// The name in `config/fonts.txt` and in the theme (`--font-<name>`).
    pub const fn name(self) -> &'static str {
        match self {
            Role::Text => "text",
            Role::Code => "code",
            Role::Logo => "logo",
        }
    }

    fn from_name(name: &str) -> Option<Role> {
        Role::ALL.into_iter().find(|role| role.name() == name)
    }
}

/// Where a problem was found: a configuration file, and the line in it when the problem is in
/// one line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Place {
    file: String,
    line: Option<usize>,
}

impl fmt::Display for Place {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.line {
            Some(line) => write!(formatter, "{}:{line}", self.file),
            None => write!(formatter, "{}", self.file),
        }
    }
}

/// What is wrong in a configuration file.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Problem {
    #[error("`{0}` is not in the form key=value")]
    NotKeyValue(String),
    #[error("`{0}` is not a role (the roles are text, code and logo)")]
    UnknownRole(String),
    #[error("the role `{0}` is given more than once")]
    RepeatedRole(&'static str),
    #[error("the role `{0}` is missing")]
    MissingRole(&'static str),
    #[error("the font `{folder}` has no description: `{path}` is missing or unreadable")]
    NoDescription { folder: String, path: String },
    #[error("`{0}` is not a key of a font's description")]
    UnknownKey(String),
    #[error("the key `{0}` is given more than once")]
    RepeatedKey(String),
    #[error("the key `{0}` is missing")]
    MissingKey(&'static str),
    #[error("the file `{label}` lacks the key `{key}`")]
    MissingFileKey { label: String, key: &'static str },
    #[error("the font describes no file")]
    NoFiles,
    #[error("`{key}` is `{value}`, which is not allowed: {expected}")]
    NotAllowed {
        key: String,
        value: String,
        expected: &'static str,
    },
    #[error("`{file}` is not in `{folder}`")]
    FileMissing { file: String, folder: String },
    #[error(
        "`{label}` and `{other}` have the same style and weight, so the browser could not choose"
    )]
    SameStyleAndWeight { label: String, other: String },
}

/// A problem, with where it was found.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{place}: {problem}")]
pub struct FontsError {
    pub place: Place,
    pub problem: Problem,
}

fn error(file: &str, line: Option<usize>, problem: Problem) -> FontsError {
    FontsError {
        place: Place {
            file: file.to_owned(),
            line,
        },
        problem,
    }
}

/// The lines of a configuration file that carry a setting, as (line number, key, value):
/// blank lines and `#` comments are left out.
fn settings<'a>(
    file: &str,
    text: &'a str,
) -> impl Iterator<Item = Result<(usize, &'a str, &'a str), FontsError>> {
    let file = file.to_owned();
    text.lines().enumerate().filter_map(move |(index, line)| {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            return None;
        }
        let number = index + 1;
        Some(match line.split_once('=') {
            Some((key, value)) => Ok((number, key.trim(), value.trim())),
            None => Err(error(
                &file,
                Some(number),
                Problem::NotKeyValue(line.to_owned()),
            )),
        })
    })
}

/// Small letters, digits and hyphens: what a folder or a label is made of. It also keeps a
/// folder name from leading out of `assets/fonts/`.
fn is_plain_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

const PLAIN_NAME: &str = "small letters, digits and hyphens";

/// One role's font, by the folder it is in, and the line that says so.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pick {
    pub folder: String,
    pub line: usize,
}

/// The font of each role, by folder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Roles {
    pub text: Pick,
    pub code: Pick,
    pub logo: Pick,
}

impl Roles {
    fn of(&self, role: Role) -> &Pick {
        match role {
            Role::Text => &self.text,
            Role::Code => &self.code,
            Role::Logo => &self.logo,
        }
    }
}

/// Where `config/fonts.txt` is, for the messages.
pub const ROLES_FILE: &str = "config/fonts.txt";

/// Reads `config/fonts.txt`: exactly one line for each role.
pub fn parse_roles(text: &str) -> Result<Roles, FontsError> {
    let mut picks: [Option<Pick>; 3] = [None, None, None];
    for setting in settings(ROLES_FILE, text) {
        let (line, key, value) = setting?;
        let role = Role::from_name(key)
            .ok_or_else(|| error(ROLES_FILE, Some(line), Problem::UnknownRole(key.to_owned())))?;
        let slot = &mut picks[role as usize];
        if slot.is_some() {
            return Err(error(
                ROLES_FILE,
                Some(line),
                Problem::RepeatedRole(role.name()),
            ));
        }
        if !is_plain_name(value) {
            return Err(error(
                ROLES_FILE,
                Some(line),
                Problem::NotAllowed {
                    key: key.to_owned(),
                    value: value.to_owned(),
                    expected: PLAIN_NAME,
                },
            ));
        }
        *slot = Some(Pick {
            folder: value.to_owned(),
            line,
        });
    }
    let [text, code, logo] = picks;
    let need = |pick: Option<Pick>, role: Role| {
        pick.ok_or_else(|| error(ROLES_FILE, None, Problem::MissingRole(role.name())))
    };
    Ok(Roles {
        text: need(text, Role::Text)?,
        code: need(code, Role::Code)?,
        logo: need(logo, Role::Logo)?,
    })
}

/// A generic family: what the browser falls back on while the font is not there.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fallback {
    Serif,
    SansSerif,
    Monospace,
    Cursive,
    Fantasy,
    SystemUi,
}

impl Fallback {
    const ALL: [(&'static str, Fallback); 6] = [
        ("serif", Fallback::Serif),
        ("sans-serif", Fallback::SansSerif),
        ("monospace", Fallback::Monospace),
        ("cursive", Fallback::Cursive),
        ("fantasy", Fallback::Fantasy),
        ("system-ui", Fallback::SystemUi),
    ];
    const NAMES: &'static str = "one of serif, sans-serif, monospace, cursive, fantasy, system-ui";

    fn css(self) -> &'static str {
        Self::ALL.iter().find(|(_, f)| *f == self).unwrap().0
    }
}

/// `font-display`: what the browser shows while the font file arrives.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Display {
    Auto,
    Block,
    Swap,
    Fallback,
    Optional,
}

impl Display {
    const ALL: [(&'static str, Display); 5] = [
        ("auto", Display::Auto),
        ("block", Display::Block),
        ("swap", Display::Swap),
        ("fallback", Display::Fallback),
        ("optional", Display::Optional),
    ];
    const NAMES: &'static str = "one of auto, block, swap, fallback, optional";

    fn css(self) -> &'static str {
        Self::ALL.iter().find(|(_, d)| *d == self).unwrap().0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style {
    Normal,
    Italic,
}

impl Style {
    fn css(self) -> &'static str {
        match self {
            Style::Normal => "normal",
            Style::Italic => "italic",
        }
    }
}

/// The format of a font file, from its extension, as the `@font-face` rule names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Woff2,
    Woff,
}

impl Format {
    fn css(self) -> &'static str {
        match self {
            Format::Woff2 => "woff2",
            Format::Woff => "woff",
        }
    }
}

/// One file of a font, with the style and weight it serves.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontFile {
    pub label: String,
    pub file: String,
    pub format: Format,
    /// What the fonts page shows under the sample. The stylesheet does not use it.
    pub name: String,
    pub style: Style,
    pub weight: u16,
}

/// A font, as its description says.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Font {
    pub folder: String,
    pub family: String,
    pub fallback: Fallback,
    pub display: Display,
    /// The font's native size in pixels, if the description gives one.
    pub size: Option<u16>,
    pub files: Vec<FontFile>,
}

impl Font {
    /// The value of a role that this font plays: its family, then the generic family.
    fn role_value(&self) -> String {
        format!("\"{}\", {}", self.family, self.fallback.css())
    }
}

/// The keys of one file, as they come in, with the line each was given on.
#[derive(Default)]
struct FileLines<'a> {
    first_line: usize,
    file: Option<(usize, &'a str)>,
    name: Option<(usize, &'a str)>,
    style: Option<(usize, &'a str)>,
    weight: Option<(usize, &'a str)>,
}

/// Where the description of a font is, for the messages and for the build.
pub fn description_path(folder: &str) -> String {
    format!("config/fonts/{folder}.txt")
}

/// Reads the description of the font in `folder`. `file_exists` says whether a file is in
/// the font's folder in `assets/fonts/`.
pub fn parse_font(
    folder: &str,
    text: &str,
    file_exists: &impl Fn(&str, &str) -> bool,
) -> Result<Font, FontsError> {
    let path = description_path(folder);
    let fail = |line: Option<usize>, problem: Problem| error(&path, line, problem);
    let not_allowed = |line: usize, key: &str, value: &str, expected: &'static str| {
        fail(
            Some(line),
            Problem::NotAllowed {
                key: key.to_owned(),
                value: value.to_owned(),
                expected,
            },
        )
    };

    let mut family: Option<(usize, &str)> = None;
    let mut fallback: Option<(usize, &str)> = None;
    let mut display: Option<(usize, &str)> = None;
    let mut size: Option<(usize, &str)> = None;
    // In order of first appearance, so the files keep the order they are written in.
    let mut labels: Vec<(&str, FileLines)> = Vec::new();

    for setting in settings(&path, text) {
        let (line, key, value) = setting?;
        let repeated = || fail(Some(line), Problem::RepeatedKey(key.to_owned()));
        let whole = match key {
            "family" => Some(&mut family),
            "fallback" => Some(&mut fallback),
            "display" => Some(&mut display),
            "size" => Some(&mut size),
            _ => None,
        };
        if let Some(slot) = whole {
            if slot.is_some() {
                return Err(repeated());
            }
            *slot = Some((line, value));
            continue;
        }
        let unknown = || fail(Some(line), Problem::UnknownKey(key.to_owned()));
        let (label, field) = key.split_once('.').ok_or_else(unknown)?;
        if !is_plain_name(label) {
            return Err(not_allowed(line, label, label, PLAIN_NAME));
        }
        let position = labels.iter().position(|(known, _)| *known == label);
        let position = position.unwrap_or_else(|| {
            labels.push((
                label,
                FileLines {
                    first_line: line,
                    ..FileLines::default()
                },
            ));
            labels.len() - 1
        });
        let lines = &mut labels[position].1;
        let taken = match field {
            "file" => lines.file.replace((line, value)).is_some(),
            "name" => lines.name.replace((line, value)).is_some(),
            "style" => lines.style.replace((line, value)).is_some(),
            "weight" => lines.weight.replace((line, value)).is_some(),
            _ => return Err(unknown()),
        };
        if taken {
            return Err(repeated());
        }
    }

    let (family_line, family) = family.ok_or_else(|| fail(None, Problem::MissingKey("family")))?;
    let (fallback_line, fallback) =
        fallback.ok_or_else(|| fail(None, Problem::MissingKey("fallback")))?;
    let (display_line, display) =
        display.ok_or_else(|| fail(None, Problem::MissingKey("display")))?;

    // The family goes between quotes in the CSS: a quote or a backslash would end the string.
    if family.is_empty() || family.contains(['"', '\\']) {
        return Err(not_allowed(
            family_line,
            "family",
            family,
            "a name with no quotation mark and no backslash",
        ));
    }
    let fallback = Fallback::ALL
        .iter()
        .find(|(name, _)| *name == fallback)
        .map(|(_, value)| *value)
        .ok_or_else(|| not_allowed(fallback_line, "fallback", fallback, Fallback::NAMES))?;
    let display = Display::ALL
        .iter()
        .find(|(name, _)| *name == display)
        .map(|(_, value)| *value)
        .ok_or_else(|| not_allowed(display_line, "display", display, Display::NAMES))?;

    let size = size
        .map(|(line, value)| {
            value
                .parse::<u16>()
                .ok()
                .filter(|size| *size > 0)
                .ok_or_else(|| not_allowed(line, "size", value, "a positive whole number"))
        })
        .transpose()?;

    if labels.is_empty() {
        return Err(fail(None, Problem::NoFiles));
    }
    let mut files: Vec<FontFile> = Vec::new();
    for (label, lines) in labels {
        let missing = |key: &'static str| {
            fail(
                Some(lines.first_line),
                Problem::MissingFileKey {
                    label: label.to_owned(),
                    key,
                },
            )
        };
        let (file_line, file) = lines.file.ok_or_else(|| missing("file"))?;
        let (name_line, name) = lines.name.ok_or_else(|| missing("name"))?;
        let (style_line, style) = lines.style.ok_or_else(|| missing("style"))?;
        let (weight_line, weight) = lines.weight.ok_or_else(|| missing("weight"))?;

        let key = |field: &str| format!("{label}.{field}");
        let format = match file.rsplit_once('.') {
            Some((stem, "woff2")) if !stem.is_empty() => Format::Woff2,
            Some((stem, "woff")) if !stem.is_empty() => Format::Woff,
            _ => {
                return Err(not_allowed(
                    file_line,
                    &key("file"),
                    file,
                    "a file name ending in .woff2 or .woff",
                ));
            }
        };
        if file.contains(['/', '\\', '"']) {
            return Err(not_allowed(
                file_line,
                &key("file"),
                file,
                "a file name, not a path",
            ));
        }
        if name.is_empty() {
            return Err(not_allowed(name_line, &key("name"), name, "a name to show"));
        }
        let style = match style {
            "normal" => Style::Normal,
            "italic" => Style::Italic,
            other => {
                return Err(not_allowed(
                    style_line,
                    &key("style"),
                    other,
                    "normal or italic",
                ));
            }
        };
        let weight = weight
            .parse::<u16>()
            .ok()
            .filter(|weight| (1..=1000).contains(weight))
            .ok_or_else(|| {
                not_allowed(
                    weight_line,
                    &key("weight"),
                    weight,
                    "a whole number from 1 to 1000",
                )
            })?;
        if !file_exists(folder, file) {
            return Err(fail(
                Some(file_line),
                Problem::FileMissing {
                    file: file.to_owned(),
                    folder: format!("assets/fonts/{folder}/"),
                },
            ));
        }
        if let Some(other) = files
            .iter()
            .find(|other| other.style == style && other.weight == weight)
        {
            return Err(fail(
                Some(lines.first_line),
                Problem::SameStyleAndWeight {
                    label: label.to_owned(),
                    other: other.label.clone(),
                },
            ));
        }
        files.push(FontFile {
            label: label.to_owned(),
            file: file.to_owned(),
            format,
            name: name.to_owned(),
            style,
            weight,
        });
    }

    Ok(Font {
        folder: folder.to_owned(),
        family: family.to_owned(),
        fallback,
        display,
        size,
        files,
    })
}

/// The fonts of the three roles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fonts {
    pub text: Font,
    pub code: Font,
    pub logo: Font,
}

impl Fonts {
    fn of(&self, role: Role) -> &Font {
        match role {
            Role::Text => &self.text,
            Role::Code => &self.code,
            Role::Logo => &self.logo,
        }
    }
}

/// Reads the roles, then the description of each font they name. `description` gives the text
/// of `config/fonts/<folder>.txt`, or `None` when there is none; `file_exists` says whether a
/// file is in a font's folder in `assets/fonts/`. Both are passed in so that the build reads
/// the disk and the tests do not.
pub fn load(
    roles_text: &str,
    description: &impl Fn(&str) -> Option<String>,
    file_exists: &impl Fn(&str, &str) -> bool,
) -> Result<Fonts, FontsError> {
    let roles = parse_roles(roles_text)?;
    let font = |role: Role| {
        let pick = roles.of(role);
        let text = description(&pick.folder).ok_or_else(|| {
            error(
                ROLES_FILE,
                Some(pick.line),
                Problem::NoDescription {
                    folder: pick.folder.clone(),
                    path: description_path(&pick.folder),
                },
            )
        })?;
        parse_font(&pick.folder, &text, file_exists)
    };
    Ok(Fonts {
        text: font(Role::Text)?,
        code: font(Role::Code)?,
        logo: font(Role::Logo)?,
    })
}

/// One sample of the fonts page: the label shown under it and the variant that draws it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sample {
    pub name: String,
    pub class: String,
}

/// A font as the fonts page shows it: once, with the roles it plays and a sample for each file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShownFont {
    pub family: String,
    pub folder: String,
    /// In the roles' order (text, code, logo); never empty.
    pub roles: Vec<Role>,
    pub samples: Vec<Sample>,
}

/// The variant that draws a file in a role: `sample-<role>-<style>-<weight>`.
fn variant(role: Role, file: &FontFile) -> String {
    format!(
        "sample-{}-{}-{}",
        role.name(),
        file.style.css(),
        file.weight
    )
}

impl Fonts {
    /// The distinct fonts, in the order of their first role. A font that plays two roles is
    /// listed once, its samples drawn with the variants of its first role.
    pub fn shown(&self) -> Vec<ShownFont> {
        let mut shown: Vec<ShownFont> = Vec::new();
        for role in Role::ALL {
            let font = self.of(role);
            if let Some(known) = shown.iter_mut().find(|known| known.folder == font.folder) {
                known.roles.push(role);
                continue;
            }
            shown.push(ShownFont {
                family: font.family.clone(),
                folder: font.folder.clone(),
                roles: vec![role],
                samples: font
                    .files
                    .iter()
                    .map(|file| Sample {
                        name: file.name.clone(),
                        class: variant(role, file),
                    })
                    .collect(),
            });
        }
        shown
    }
}

/// The stylesheet's fonts: the three roles in `@theme`, then one `@font-face` rule for each
/// file of each font that plays a role (a font that plays two roles is declared once).
///
/// `--font-*: initial` comes first in the same block: it clears Tailwind's own fonts (sans,
/// serif, mono) so that only the three roles exist, and it only clears what was defined before
/// it, so it must come before the roles, not in another block that is read after them.
pub fn css(fonts: &Fonts) -> String {
    let mut out = String::from(
        "/* Generated by the build from config/fonts.txt and config/fonts/. Do not edit. */\n\n\
         @theme {\n  --font-*: initial;\n",
    );
    for role in Role::ALL {
        let _ = writeln!(
            out,
            "  --font-{}: {};",
            role.name(),
            fonts.of(role).role_value()
        );
    }
    out.push_str("}\n");
    let mut declared: Vec<&str> = Vec::new();
    for role in Role::ALL {
        let font = fonts.of(role);
        if declared.contains(&font.folder.as_str()) {
            continue;
        }
        declared.push(&font.folder);
        for file in &font.files {
            let _ = write!(
                out,
                "\n@font-face {{\n  font-family: \"{family}\";\n  \
                 src: url(\"fonts/{folder}/{name}\") format(\"{format}\");\n  \
                 font-style: {style};\n  font-weight: {weight};\n  font-display: {display};\n}}\n",
                family = font.family,
                folder = font.folder,
                name = file.file,
                format = file.format.css(),
                style = file.style.css(),
                weight = file.weight,
                display = font.display.css(),
            );
        }
    }
    // One variant for each file of each role's font, for the samples of the fonts page: the
    // font comes from the role's variable, the browser picks the file from the @font-face rules.
    let mut names: Vec<String> = Vec::new();
    for role in Role::ALL {
        let font = fonts.of(role);
        for file in &font.files {
            let name = variant(role, file);
            let _ = write!(
                out,
                "\n@utility {name} {{\n  @apply font-{role};\n  font-style: {style};\n  font-weight: {weight};\n",
                role = role.name(),
                style = file.style.css(),
                weight = file.weight,
            );
            if let Some(size) = font.size {
                let _ = writeln!(out, "  font-size: {size}px;\n  max-width: 80ch;");
            }
            out.push_str("}\n");
            names.push(name);
        }
    }
    // The names are in no scanned file (the page's HTML is generated), so Tailwind is told.
    let _ = writeln!(out, "\n@source inline(\"{}\");", names.join(" "));
    out
}

/// The configuration in the repository, read from the disk as the build reads it.
#[cfg(test)]
pub fn real() -> Fonts {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    load(
        &std::fs::read_to_string(root.join(ROLES_FILE)).unwrap(),
        &|folder| std::fs::read_to_string(root.join(description_path(folder))).ok(),
        &|folder, file| root.join("assets/fonts").join(folder).join(file).is_file(),
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROLES: &str = "text=literata\ncode=hack\nlogo=ibm-vga-8x16\n";
    const LITERATA: &str = include_str!("../../config/fonts/literata.txt");
    const HACK: &str = include_str!("../../config/fonts/hack.txt");
    const PIXEL: &str = include_str!("../../config/fonts/ibm-vga-8x16.txt");

    fn describe(folder: &str) -> Option<String> {
        match folder {
            "literata" => Some(LITERATA.to_owned()),
            "hack" => Some(HACK.to_owned()),
            "ibm-vga-8x16" => Some(PIXEL.to_owned()),
            _ => None,
        }
    }

    fn everything_exists(_: &str, _: &str) -> bool {
        true
    }

    fn roles_error(text: &str) -> String {
        parse_roles(text).unwrap_err().to_string()
    }

    fn font_error(text: &str) -> String {
        parse_font("x", text, &everything_exists)
            .unwrap_err()
            .to_string()
    }

    /// A valid description with one file, to break one line at a time.
    const SMALL: &str = "family=Small\nfallback=serif\ndisplay=swap\n\
                         regular.file=small.woff2\nregular.name=Regular\n\
                         regular.style=normal\nregular.weight=400\n";

    fn small_without(key: &str) -> String {
        SMALL
            .lines()
            .filter(|line| !line.starts_with(key))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn small_with(key: &str, value: &str) -> String {
        SMALL
            .lines()
            .map(|line| {
                if line.starts_with(&format!("{key}=")) {
                    format!("{key}={value}")
                } else {
                    line.to_owned()
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    // The roles' file.

    #[test]
    fn the_roles_are_read_with_their_lines() {
        let roles =
            parse_roles("# the fonts\n\ntext=literata\ncode = hack\nlogo=ibm-vga-8x16\n").unwrap();
        assert_eq!(roles.text.folder, "literata");
        assert_eq!(roles.text.line, 3);
        assert_eq!(roles.code.folder, "hack");
        assert_eq!(roles.logo.folder, "ibm-vga-8x16");
    }

    #[test]
    fn a_missing_role_stops_the_build() {
        assert_eq!(
            roles_error("text=literata\ncode=hack\n"),
            "config/fonts.txt: the role `logo` is missing"
        );
    }

    #[test]
    fn a_repeated_role_stops_the_build() {
        assert_eq!(
            roles_error("text=literata\ncode=hack\ntext=hack\nlogo=x\n"),
            "config/fonts.txt:3: the role `text` is given more than once"
        );
    }

    #[test]
    fn an_unknown_role_stops_the_build() {
        assert_eq!(
            roles_error("text=literata\nheading=hack\n"),
            "config/fonts.txt:2: `heading` is not a role (the roles are text, code and logo)"
        );
    }

    #[test]
    fn a_roles_line_that_is_not_key_value_stops_the_build() {
        assert_eq!(
            roles_error("text=literata\ncode hack\n"),
            "config/fonts.txt:2: `code hack` is not in the form key=value"
        );
    }

    #[test]
    fn a_folder_that_could_leave_assets_fonts_stops_the_build() {
        assert_eq!(
            roles_error("text=../secrets\n"),
            "config/fonts.txt:1: `text` is `../secrets`, which is not allowed: small letters, digits and hyphens"
        );
    }

    #[test]
    fn a_role_with_no_description_stops_the_build() {
        let error = load(
            "text=nowhere\ncode=hack\nlogo=hack\n",
            &describe,
            &everything_exists,
        )
        .unwrap_err();
        assert_eq!(
            error.to_string(),
            "config/fonts.txt:1: the font `nowhere` has no description: `config/fonts/nowhere.txt` is missing or unreadable"
        );
    }

    // A font's description.

    #[test]
    fn a_description_is_read() {
        let font = parse_font("small", SMALL, &everything_exists).unwrap();
        assert_eq!(font.family, "Small");
        assert_eq!(font.fallback, Fallback::Serif);
        assert_eq!(font.display, Display::Swap);
        assert_eq!(font.files.len(), 1);
        assert_eq!(font.files[0].format, Format::Woff2);
        assert_eq!(font.files[0].name, "Regular");
    }

    #[test]
    fn a_missing_key_of_the_font_stops_the_build() {
        for key in ["family", "fallback", "display"] {
            assert_eq!(
                font_error(&small_without(key)),
                format!("config/fonts/x.txt: the key `{key}` is missing")
            );
        }
    }

    #[test]
    fn a_missing_key_of_a_file_stops_the_build_at_the_files_first_line() {
        for key in ["file", "name", "style", "weight"] {
            // The label's first line is line 4, whichever of its keys comes first.
            let line = 4;
            assert_eq!(
                font_error(&small_without(&format!("regular.{key}"))),
                format!("config/fonts/x.txt:{line}: the file `regular` lacks the key `{key}`")
            );
        }
    }

    #[test]
    fn a_font_with_no_file_stops_the_build() {
        assert_eq!(
            font_error("family=A\nfallback=serif\ndisplay=swap\n"),
            "config/fonts/x.txt: the font describes no file"
        );
    }

    #[test]
    fn an_unknown_key_stops_the_build() {
        assert_eq!(
            font_error(&format!("{SMALL}colour=red\n")),
            "config/fonts/x.txt:8: `colour` is not a key of a font's description"
        );
        assert_eq!(
            font_error(&format!("{SMALL}regular.size=3\n")),
            "config/fonts/x.txt:8: `regular.size` is not a key of a font's description"
        );
    }

    #[test]
    fn a_repeated_key_stops_the_build() {
        assert_eq!(
            font_error(&format!("{SMALL}family=Other\n")),
            "config/fonts/x.txt:8: the key `family` is given more than once"
        );
        assert_eq!(
            font_error(&format!("{SMALL}regular.weight=700\n")),
            "config/fonts/x.txt:8: the key `regular.weight` is given more than once"
        );
    }

    #[test]
    fn a_line_that_is_not_key_value_stops_the_build() {
        assert_eq!(
            font_error(&format!("{SMALL}serif\n")),
            "config/fonts/x.txt:8: `serif` is not in the form key=value"
        );
    }

    #[test]
    fn a_style_that_is_not_allowed_stops_the_build() {
        assert_eq!(
            font_error(&small_with("regular.style", "oblique")),
            "config/fonts/x.txt:6: `regular.style` is `oblique`, which is not allowed: normal or italic"
        );
    }

    #[test]
    fn a_weight_that_is_not_allowed_stops_the_build() {
        for weight in ["0", "1001", "bold", "-3", "400.5", ""] {
            assert_eq!(
                font_error(&small_with("regular.weight", weight)),
                format!(
                    "config/fonts/x.txt:7: `regular.weight` is `{weight}`, which is not allowed: a whole number from 1 to 1000"
                )
            );
        }
        for weight in ["1", "1000"] {
            assert!(
                parse_font(
                    "x",
                    &small_with("regular.weight", weight),
                    &everything_exists
                )
                .is_ok()
            );
        }
    }

    #[test]
    fn a_display_that_is_not_allowed_stops_the_build() {
        assert_eq!(
            font_error(&small_with("display", "slow")),
            "config/fonts/x.txt:3: `display` is `slow`, which is not allowed: one of auto, block, swap, fallback, optional"
        );
    }

    #[test]
    fn a_fallback_that_is_not_allowed_stops_the_build() {
        assert_eq!(
            font_error(&small_with("fallback", "Times")),
            "config/fonts/x.txt:2: `fallback` is `Times`, which is not allowed: one of serif, sans-serif, monospace, cursive, fantasy, system-ui"
        );
    }

    #[test]
    fn a_family_that_would_break_the_css_stops_the_build() {
        for family in ["", "A\"B", "A\\B"] {
            assert!(
                font_error(&small_with("family", family))
                    .starts_with("config/fonts/x.txt:1: `family` is"),
                "{family}"
            );
        }
    }

    #[test]
    fn a_file_extension_that_is_not_allowed_stops_the_build() {
        for file in ["small.ttf", "small", ".woff2", "small.woff2.zip"] {
            assert_eq!(
                font_error(&small_with("regular.file", file)),
                format!(
                    "config/fonts/x.txt:4: `regular.file` is `{file}`, which is not allowed: a file name ending in .woff2 or .woff"
                )
            );
        }
    }

    #[test]
    fn a_file_name_that_is_a_path_stops_the_build() {
        assert_eq!(
            font_error(&small_with("regular.file", "../small.woff2")),
            "config/fonts/x.txt:4: `regular.file` is `../small.woff2`, which is not allowed: a file name, not a path"
        );
    }

    #[test]
    fn a_label_that_is_not_plain_stops_the_build() {
        assert_eq!(
            font_error(&format!("{SMALL}Bold.file=b.woff2\n")),
            "config/fonts/x.txt:8: `Bold` is `Bold`, which is not allowed: small letters, digits and hyphens"
        );
    }

    #[test]
    fn a_file_that_is_not_in_the_fonts_folder_stops_the_build() {
        let nothing = |_: &str, _: &str| false;
        let error = parse_font("small", SMALL, &nothing).unwrap_err();
        assert_eq!(
            error.to_string(),
            "config/fonts/small.txt:4: `small.woff2` is not in `assets/fonts/small/`"
        );
    }

    #[test]
    fn two_files_with_the_same_style_and_weight_stop_the_build() {
        let twice = format!(
            "{SMALL}other.file=other.woff2\nother.name=Other\nother.style=normal\nother.weight=400\n"
        );
        assert_eq!(
            font_error(&twice),
            "config/fonts/x.txt:8: `other` and `regular` have the same style and weight, so the browser could not choose"
        );
        // The same weight in another style, or another weight in the same style, is fine.
        let italic = twice.replace("other.style=normal", "other.style=italic");
        assert!(parse_font("x", &italic, &everything_exists).is_ok());
    }

    // The size.

    #[test]
    fn the_size_is_optional_and_read_as_pixels() {
        assert_eq!(
            parse_font("x", SMALL, &everything_exists).unwrap().size,
            None
        );
        assert_eq!(
            parse_font("x", &with_size("16"), &everything_exists)
                .unwrap()
                .size,
            Some(16)
        );
        assert_eq!(real().logo.size, Some(16));
        assert_eq!(real().text.size, None);
        assert_eq!(real().code.size, None);
    }

    #[test]
    fn a_size_that_is_not_a_positive_whole_number_stops_the_build() {
        for size in ["0", "-3", "16.5", "big", "", "70000"] {
            assert_eq!(
                font_error(&with_size(size)),
                format!(
                    "config/fonts/x.txt:8: `size` is `{size}`, which is not allowed: a positive whole number"
                )
            );
        }
    }

    #[test]
    fn a_repeated_size_stops_the_build() {
        assert_eq!(
            font_error(&format!("{}size=8\n", with_size("16"))),
            "config/fonts/x.txt:9: the key `size` is given more than once"
        );
    }

    // The samples.

    #[test]
    fn each_file_of_the_real_fonts_gets_a_variant_named_from_its_role_style_and_weight() {
        let shown = real().shown();
        let classes = |at: usize| -> Vec<&str> {
            shown[at].samples.iter().map(|s| s.class.as_str()).collect()
        };
        assert_eq!(
            classes(0),
            [
                "sample-text-normal-400",
                "sample-text-italic-400",
                "sample-text-normal-700",
                "sample-text-italic-700"
            ]
        );
        assert_eq!(classes(1), ["sample-code-normal-400"]);
        assert_eq!(classes(2), ["sample-logo-normal-400"]);
        let names: Vec<&str> = shown[0].samples.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names, ["Regular", "Italic", "Bold", "Bold italic"]);
        assert_eq!(shown[0].family, "Literata");
        assert_eq!(shown[2].family, "IBM VGA 8x16");
    }

    #[test]
    fn a_font_that_plays_two_roles_is_shown_once_with_both_roles() {
        let fonts = load(
            "text=literata\ncode=hack\nlogo=hack\n",
            &describe,
            &everything_exists,
        )
        .unwrap();
        let shown = fonts.shown();
        assert_eq!(shown.len(), 2);
        assert_eq!(shown[1].family, "Hack");
        assert_eq!(shown[1].roles, [Role::Code, Role::Logo]);
        assert_eq!(shown[1].samples[0].class, "sample-code-normal-400");
    }

    #[test]
    fn a_file_added_to_the_configuration_gets_its_sample_and_its_variant() {
        let more = format!(
            "{LITERATA}semibold.file=literata-latin-600-normal.woff2\nsemibold.name=Semibold\n\
             semibold.style=normal\nsemibold.weight=600\n"
        );
        let describe = |folder: &str| match folder {
            "literata" => Some(more.clone()),
            other => describe(other),
        };
        let fonts = load(ROLES, &describe, &everything_exists).unwrap();
        let samples = &fonts.shown()[0].samples;
        assert_eq!(samples.len(), 5);
        assert_eq!(samples[4].name, "Semibold");
        assert_eq!(samples[4].class, "sample-text-normal-600");
        let css = css(&fonts);
        assert!(css.contains("@utility sample-text-normal-600 {"));
        assert!(css.contains("sample-text-italic-700 sample-text-normal-600"));
    }

    fn rule<'a>(css: &'a str, name: &str) -> &'a str {
        let start = css.find(&format!("@utility {name} {{")).unwrap();
        let rule = &css[start..];
        &rule[..rule.find("}\n").unwrap()]
    }

    #[test]
    fn the_variants_take_the_font_from_the_role_and_never_name_one() {
        let css = css(&real());
        assert_eq!(
            rule(&css, "sample-text-italic-700"),
            "@utility sample-text-italic-700 {\n  @apply font-text;\n  font-style: italic;\n  font-weight: 700;\n"
        );
        assert!(rule(&css, "sample-code-normal-400").contains("@apply font-code;"));
        assert!(rule(&css, "sample-logo-normal-400").contains("@apply font-logo;"));
        for name in [
            "sample-text-normal-400",
            "sample-code-normal-400",
            "sample-logo-normal-400",
        ] {
            assert!(!rule(&css, name).contains("font-family"), "{name}");
        }
    }

    #[test]
    fn a_font_with_a_size_is_drawn_at_it_and_no_wider_than_80_characters() {
        let css = css(&real());
        let pixel = rule(&css, "sample-logo-normal-400");
        assert!(pixel.contains("  font-size: 16px;\n"));
        assert!(pixel.contains("  max-width: 80ch;\n"));
        // Without a size the page's size and width are kept.
        for name in ["sample-text-normal-400", "sample-code-normal-400"] {
            assert!(!rule(&css, name).contains("font-size"), "{name}");
            assert!(!rule(&css, name).contains("max-width"), "{name}");
        }
    }

    #[test]
    fn tailwind_is_told_every_variant() {
        let css = css(&real());
        let line = css
            .lines()
            .find(|l| l.starts_with("@source inline("))
            .unwrap();
        for name in [
            "sample-text-normal-400",
            "sample-text-italic-400",
            "sample-text-normal-700",
            "sample-text-italic-700",
            "sample-code-normal-400",
            "sample-logo-normal-400",
        ] {
            assert!(line.contains(name), "{name}");
        }
    }

    // The CSS.

    fn real() -> Fonts {
        load(ROLES, &describe, &everything_exists).unwrap()
    }

    fn with_size(value: &str) -> String {
        format!("{SMALL}size={value}\n")
    }

    #[test]
    fn the_real_configuration_gives_the_roles_the_stylesheet_had() {
        let css = css(&real());
        assert!(css.contains("  --font-*: initial;\n"));
        assert!(css.contains("  --font-text: \"Literata\", serif;\n"));
        assert!(css.contains("  --font-code: \"Hack\", monospace;\n"));
        assert!(css.contains("  --font-logo: \"IBM VGA 8x16\", monospace;\n"));
    }

    #[test]
    fn the_real_configuration_declares_the_six_font_faces_the_stylesheet_had() {
        let css = css(&real());
        assert_eq!(css.matches("@font-face").count(), 6);
        assert!(css.contains(
            "@font-face {\n  font-family: \"Literata\";\n  \
             src: url(\"fonts/literata/literata-latin-700-italic.woff2\") format(\"woff2\");\n  \
             font-style: italic;\n  font-weight: 700;\n  font-display: swap;\n}\n"
        ));
        assert!(css.contains(
            "@font-face {\n  font-family: \"Hack\";\n  \
             src: url(\"fonts/hack/hack-regular-subset.woff2\") format(\"woff2\");\n  \
             font-style: normal;\n  font-weight: 400;\n  font-display: swap;\n}\n"
        ));
        assert!(css.contains(
            "@font-face {\n  font-family: \"IBM VGA 8x16\";\n  \
             src: url(\"fonts/ibm-vga-8x16/Web437_IBM_VGA_8x16.woff\") format(\"woff\");\n  \
             font-style: normal;\n  font-weight: 400;\n  font-display: block;\n}\n"
        ));
    }

    /// Acceptance test 6 of the card on linking the fonts to the build.
    #[test]
    fn changing_a_roles_font_changes_the_roles_value() {
        let fonts = load(
            "text=hack\ncode=hack\nlogo=ibm-vga-8x16\n",
            &describe,
            &everything_exists,
        )
        .unwrap();
        let css = css(&fonts);
        assert!(css.contains("  --font-text: \"Hack\", monospace;\n"));
        assert!(!css.contains("Literata"));
    }

    #[test]
    fn a_font_that_plays_two_roles_is_declared_once() {
        let fonts = load(
            "text=hack\ncode=hack\nlogo=ibm-vga-8x16\n",
            &describe,
            &everything_exists,
        )
        .unwrap();
        let css = css(&fonts);
        assert_eq!(css.matches("font-family: \"Hack\"").count(), 1);
        assert_eq!(css.matches("@font-face").count(), 2);
    }

    /// The configuration in the repository is read from the disk as the build reads it, so a
    /// file named there but missing from `assets/fonts/` fails here, before the build.
    #[test]
    fn the_configuration_in_the_repository_is_valid_on_the_disk() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let fonts = load(
            &std::fs::read_to_string(root.join(ROLES_FILE)).unwrap(),
            &|folder| std::fs::read_to_string(root.join(description_path(folder))).ok(),
            &|folder, file| root.join("assets/fonts").join(folder).join(file).is_file(),
        )
        .unwrap();
        assert_eq!(fonts, real());
    }

    // The stylesheet.

    /// The stylesheet without its comments, which may name a font to explain a choice.
    fn without_comments(css: &str) -> String {
        let mut out = String::new();
        let mut rest = css;
        while let Some(start) = rest.find("/*") {
            out.push_str(&rest[..start]);
            rest = &rest[start + 2..];
            rest = &rest[rest.find("*/").map_or(rest.len(), |end| end + 2)..];
        }
        out.push_str(rest);
        out
    }

    /// What names a font in the stylesheet: a font face, a `font-family` that is not one of the
    /// roles' variables, a role given a value, or the family of one of the fonts.
    fn font_names_in(css: &str, families: &[&str]) -> Vec<String> {
        let css = without_comments(css);
        let mut found = Vec::new();
        if css.contains("@font-face") {
            found.push("@font-face".to_owned());
        }
        for declaration in css.split("font-family:").skip(1) {
            let value = declaration.split(';').next().unwrap().trim();
            if !value.starts_with("var(--font-") {
                found.push(format!("font-family: {value}"));
            }
        }
        for role in Role::ALL {
            if css.contains(&format!("--font-{}:", role.name())) {
                found.push(format!("--font-{}", role.name()));
            }
        }
        for family in families {
            if css.contains(family) {
                found.push((*family).to_owned());
            }
        }
        found
    }

    fn families() -> Vec<String> {
        let fonts = real();
        [fonts.text, fonts.code, fonts.logo]
            .into_iter()
            .map(|font| font.family)
            .collect()
    }

    const STYLESHEET: &str = include_str!("../../style/input.css");

    /// Acceptance test 4 of the card on linking the fonts to the build: no font is named
    /// anywhere in the hand-written stylesheet. The fonts are in the configuration and reach the
    /// stylesheet through the file the build writes.
    #[test]
    fn the_stylesheet_names_no_font_anywhere() {
        let families = families();
        let families: Vec<&str> = families.iter().map(String::as_str).collect();
        let css = without_comments(STYLESHEET);
        assert!(
            css.contains("@utility sample-label"),
            "the comments were cut wrong"
        );
        assert!(
            css.contains("font-family: var(--font-text)"),
            "html and body"
        );
        assert_eq!(font_names_in(STYLESHEET, &families), Vec::<String>::new());
    }

    #[test]
    fn a_font_written_by_hand_in_the_stylesheet_is_found() {
        let families = families();
        let families: Vec<&str> = families.iter().map(String::as_str).collect();
        let by_hand = STYLESHEET.replace(
            "font-family: var(--font-text);",
            "font-family: \"Literata\", serif;",
        );
        assert_ne!(
            by_hand, STYLESHEET,
            "html and body no longer set font-family: var(--font-text)"
        );
        assert_eq!(
            font_names_in(&by_hand, &families),
            [
                "font-family: \"Literata\", serif",
                "font-family: \"Literata\", serif",
                "Literata"
            ]
        );

        let face = format!("{STYLESHEET}\n@font-face {{ src: url(x.woff2); }}\n");
        assert_eq!(font_names_in(&face, &families), ["@font-face"]);

        let role = format!("{STYLESHEET}\n@theme {{ --font-text: sans-serif; }}\n");
        assert_eq!(font_names_in(&role, &families), ["--font-text"]);

        // A family named in a comment is no violation.
        let comment = format!("{STYLESHEET}\n/* Literata is the text's font */\n");
        assert_eq!(font_names_in(&comment, &families), Vec::<String>::new());
    }
}
