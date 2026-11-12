//! The site's colours: six roles, each with a dark and a light value, and the pairs of roles
//! that may be used together.
//!
//! The stylesheet does not hold the colours: the build checks them here and writes them to
//! `target/generated/colours.css`, which `style/input.css` imports.
//!
//! Dark is the default scheme. The light values replace it when the browser prefers light
//! pages, which is also what a browser with no preference reports. The light block is
//! unlayered and comes after the theme, so it wins over the theme's dark values. `color-scheme`
//! makes the checkbox and the scrollbars follow the scheme.
//!
//! Each accent has its own role because the light page needs a different shade for each use.
//! The light amber shades have a redder hue (32 degrees) than the dark amber (40 degrees),
//! chosen by the user, with the same saturation.
//!
//! The ink is off-white, not pure white: pure white smears on a dark background for readers
//! with astigmatism.
//!
//! The contrast ratios are not written here: the tests hold them, and the build refuses a
//! palette in which an allowed pair falls below its threshold.

use std::fmt::{self, Write};

/// A colour, built only from a CSS hex code `#rrggbb`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Colour {
    red: u8,
    green: u8,
    blue: u8,
}

/// Why a text is not a colour.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum InvalidColour {
    #[error("a colour starts with `#`")]
    MissingHash,
    #[error("a colour has six hexadecimal digits after `#`, not {0}")]
    WrongLength(usize),
    #[error("`{0}` is not a hexadecimal digit")]
    NotHexadecimal(char),
}

const fn hex_digit(byte: u8) -> Result<u8, InvalidColour> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(InvalidColour::NotHexadecimal(byte as char)),
    }
}

const fn hex_pair(high: u8, low: u8) -> Result<u8, InvalidColour> {
    let high = match hex_digit(high) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    match hex_digit(low) {
        Ok(low) => Ok(high * 16 + low),
        Err(error) => Err(error),
    }
}

impl Colour {
    /// Reads `#rrggbb`. `const`, so that a wrong code in a constant fails the compilation.
    pub const fn parse(code: &str) -> Result<Self, InvalidColour> {
        let bytes = code.as_bytes();
        if bytes.is_empty() || bytes[0] != b'#' {
            return Err(InvalidColour::MissingHash);
        }
        if bytes.len() != 7 {
            return Err(InvalidColour::WrongLength(bytes.len() - 1));
        }
        let red = match hex_pair(bytes[1], bytes[2]) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        let green = match hex_pair(bytes[3], bytes[4]) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        let blue = match hex_pair(bytes[5], bytes[6]) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        Ok(Self { red, green, blue })
    }

    /// For constants: a wrong code stops the compilation instead of reaching the build.
    const fn code(code: &str) -> Self {
        match Self::parse(code) {
            Ok(colour) => colour,
            Err(_) => panic!("invalid colour code in the palette"),
        }
    }

    /// Relative luminance, as WCAG 2 defines it.
    fn luminance(self) -> f64 {
        // The linearisation threshold is 0.04045 (the sRGB standard); WCAG 2.x prints 0.03928.
        // For 8-bit values both give the same result.
        let linear = |channel: u8| {
            let value = f64::from(channel) / 255.0;
            if value <= 0.04045 {
                value / 12.92
            } else {
                ((value + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * linear(self.red) + 0.7152 * linear(self.green) + 0.0722 * linear(self.blue)
    }
}

impl fmt::Display for Colour {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "#{:02x}{:02x}{:02x}",
            self.red, self.green, self.blue
        )
    }
}

/// The WCAG 2 contrast ratio between two colours, from 1 to 21, unrounded.
pub fn contrast_ratio(first: Colour, second: Colour) -> f64 {
    let (first, second) = (first.luminance(), second.luminance());
    let (lighter, darker) = if first >= second {
        (first, second)
    } else {
        (second, first)
    };
    (lighter + 0.05) / (darker + 0.05)
}

/// What a colour is for. Each role has its own name in the stylesheet (`--color-<name>`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// Page and footer background.
    Page,
    /// Running text, footer text, code block text.
    Ink,
    /// Code blocks and odd table rows.
    CodeSurface,
    /// Every line and frame, focus outlines.
    AccentLine,
    /// Large text only: logo, titles, h2, h3.
    AccentHeading,
    /// Normal-size text: inline code, the "wrap" label.
    AccentCode,
}

impl Role {
    pub const ALL: [Role; 6] = [
        Role::Page,
        Role::Ink,
        Role::CodeSurface,
        Role::AccentLine,
        Role::AccentHeading,
        Role::AccentCode,
    ];

    pub const fn name(self) -> &'static str {
        match self {
            Role::Page => "page",
            Role::Ink => "ink",
            Role::CodeSurface => "code-surface",
            Role::AccentLine => "accent-line",
            Role::AccentHeading => "accent-heading",
            Role::AccentCode => "accent-code",
        }
    }
}

/// The six colours of one scheme.
#[derive(Debug, Clone, Copy)]
pub struct Colours {
    pub page: Colour,
    pub ink: Colour,
    pub code_surface: Colour,
    pub accent_line: Colour,
    pub accent_heading: Colour,
    pub accent_code: Colour,
}

impl Colours {
    pub const fn of(self, role: Role) -> Colour {
        match role {
            Role::Page => self.page,
            Role::Ink => self.ink,
            Role::CodeSurface => self.code_surface,
            Role::AccentLine => self.accent_line,
            Role::AccentHeading => self.accent_heading,
            Role::AccentCode => self.accent_code,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub dark: Colours,
    pub light: Colours,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scheme {
    Dark,
    Light,
}

impl Scheme {
    const fn name(self) -> &'static str {
        match self {
            Scheme::Dark => "dark",
            Scheme::Light => "light",
        }
    }
}

impl Palette {
    pub const fn scheme(&self, scheme: Scheme) -> Colours {
        match scheme {
            Scheme::Dark => self.dark,
            Scheme::Light => self.light,
        }
    }
}

pub const PALETTE: Palette = Palette {
    dark: Colours {
        page: Colour::code("#000000"),
        ink: Colour::code("#e6e6e6"),
        code_surface: Colour::code("#1a1a1a"),
        accent_line: Colour::code("#dd9911"),
        accent_heading: Colour::code("#dd9911"),
        accent_code: Colour::code("#dd9911"),
    },
    light: Colours {
        page: Colour::code("#fbfaf7"),
        ink: Colour::code("#1c1a17"),
        code_surface: Colour::code("#ebe7df"),
        accent_line: Colour::code("#c06d0f"),
        accent_heading: Colour::code("#a9600d"),
        accent_code: Colour::code("#6f3f09"),
    },
};

/// What a pair of colours is used for. The use decides the contrast it needs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Use {
    NormalText,
    /// Large text needs less contrast (WCAG: 18.66px bold or 24px).
    LargeText,
    /// Lines, frames and focus outlines.
    Graphics,
}

impl Use {
    /// The least contrast ratio the use accepts. Normal text asks for the AAA level, 7:1.
    pub const fn threshold(self) -> f64 {
        match self {
            Use::NormalText => 7.0,
            Use::LargeText => 4.5,
            Use::Graphics => 3.0,
        }
    }
}

/// A foreground that may be used on each of the listed backgrounds, for one use.
#[derive(Debug, Clone, Copy)]
pub struct AllowedPair {
    pub foreground: Role,
    pub backgrounds: &'static [Role],
    pub usage: Use,
}

/// Every pair that may appear on the site, the same in both schemes. A pair missing from the
/// list is forbidden: for example the heading accent on the code surface, which the light
/// scheme cannot make legible.
pub const ALLOWED_PAIRS: [AllowedPair; 4] = [
    AllowedPair {
        foreground: Role::Ink,
        backgrounds: &[Role::Page, Role::CodeSurface],
        usage: Use::NormalText,
    },
    AllowedPair {
        foreground: Role::AccentCode,
        backgrounds: &[Role::Page, Role::CodeSurface],
        usage: Use::NormalText,
    },
    AllowedPair {
        foreground: Role::AccentHeading,
        backgrounds: &[Role::Page],
        usage: Use::LargeText,
    },
    AllowedPair {
        foreground: Role::AccentLine,
        backgrounds: &[Role::Page, Role::CodeSurface],
        usage: Use::Graphics,
    },
];

/// An allowed pair that does not reach its threshold.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
#[error(
    "{} scheme: {} on {} has a contrast of {ratio:.2}:1, below the {threshold:.2}:1 its use needs",
    scheme.name(), foreground.name(), background.name()
)]
pub struct ContrastTooLow {
    pub scheme: Scheme,
    pub foreground: Role,
    pub background: Role,
    pub ratio: f64,
    pub threshold: f64,
}

/// Checks every allowed pair in both schemes. The unrounded ratio is compared, so a ratio
/// shown as 4.50 that is really 4.496 does not pass.
pub fn check(palette: &Palette) -> Result<(), ContrastTooLow> {
    for scheme in [Scheme::Dark, Scheme::Light] {
        let colours = palette.scheme(scheme);
        for pair in ALLOWED_PAIRS {
            for &background in pair.backgrounds {
                let ratio = contrast_ratio(colours.of(pair.foreground), colours.of(background));
                let threshold = pair.usage.threshold();
                if ratio < threshold {
                    return Err(ContrastTooLow {
                        scheme,
                        foreground: pair.foreground,
                        background,
                        ratio,
                        threshold,
                    });
                }
            }
        }
    }
    Ok(())
}

fn variables(colours: Colours, indent: &str, out: &mut String) {
    for role in Role::ALL {
        let _ = writeln!(
            out,
            "{indent}--color-{}: {};",
            role.name(),
            colours.of(role)
        );
    }
}

/// The CSS for `style/input.css`: the dark values in Tailwind's theme, the light values in a
/// media query after it.
pub fn css(palette: &Palette) -> String {
    let mut out = String::from(
        "/* Generated by the build from src/style_guide/colours.rs. Do not edit. */\n\n\
         @theme {\n",
    );
    variables(palette.dark, "  ", &mut out);
    out.push_str("}\n\n:root {\n  color-scheme: dark;\n}\n\n");
    out.push_str("@media (prefers-color-scheme: light) {\n  :root {\n    color-scheme: light;\n");
    variables(palette.light, "    ", &mut out);
    out.push_str("  }\n}\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn colour(code: &str) -> Colour {
        Colour::parse(code).unwrap()
    }

    fn ratio(scheme: Scheme, foreground: Role, background: Role) -> String {
        let colours = PALETTE.scheme(scheme);
        format!(
            "{:.2}",
            contrast_ratio(colours.of(foreground), colours.of(background))
        )
    }

    #[test]
    fn a_hex_code_is_read_and_written_back() {
        assert_eq!(colour("#dd9911").to_string(), "#dd9911");
        assert_eq!(colour("#DD9911").to_string(), "#dd9911");
    }

    #[test]
    fn a_text_that_is_not_a_hex_code_is_refused() {
        assert_eq!(Colour::parse("dd9911"), Err(InvalidColour::MissingHash));
        assert_eq!(Colour::parse(""), Err(InvalidColour::MissingHash));
        assert_eq!(Colour::parse("#dd991"), Err(InvalidColour::WrongLength(5)));
        assert_eq!(
            Colour::parse("#dd99111"),
            Err(InvalidColour::WrongLength(7))
        );
        assert_eq!(
            Colour::parse("#dd99g1"),
            Err(InvalidColour::NotHexadecimal('g'))
        );
    }

    #[test]
    fn black_on_white_is_the_widest_contrast() {
        assert_eq!(ratio_of("#000000", "#ffffff"), "21.00");
    }

    fn ratio_of(first: &str, second: &str) -> String {
        format!("{:.2}", contrast_ratio(colour(first), colour(second)))
    }

    #[test]
    fn the_dark_ratios_are_the_ones_the_stylesheet_used_to_state() {
        use Role::*;
        let dark = |foreground, background| ratio(Scheme::Dark, foreground, background);
        assert_eq!(dark(Ink, Page), "16.83");
        assert_eq!(dark(Ink, CodeSurface), "13.94");
        assert_eq!(dark(AccentCode, Page), "8.64");
        assert_eq!(dark(AccentCode, CodeSurface), "7.16");
    }

    #[test]
    fn the_light_ratios_are_the_ones_the_stylesheet_used_to_state() {
        use Role::*;
        let light = |foreground, background| ratio(Scheme::Light, foreground, background);
        assert_eq!(light(Ink, Page), "16.63");
        assert_eq!(light(Ink, CodeSurface), "14.08");
        assert_eq!(light(AccentLine, Page), "3.70");
        assert_eq!(light(AccentLine, CodeSurface), "3.13");
        assert_eq!(light(AccentHeading, Page), "4.61");
        assert_eq!(light(AccentCode, Page), "8.42");
        assert_eq!(light(AccentCode, CodeSurface), "7.12");
    }

    #[test]
    fn the_heading_accent_on_the_code_surface_is_forbidden_because_it_is_too_dim() {
        let colours = PALETTE.light;
        let value = contrast_ratio(colours.accent_heading, colours.code_surface);
        assert!((3.85..3.95).contains(&value), "{value}");
        assert!(value < Use::LargeText.threshold());
        assert!(!ALLOWED_PAIRS.iter().any(|pair| {
            pair.foreground == Role::AccentHeading && pair.backgrounds.contains(&Role::CodeSurface)
        }));
    }

    #[test]
    fn the_palette_passes_the_check() {
        assert_eq!(check(&PALETTE), Ok(()));
    }

    #[test]
    fn a_pair_below_its_threshold_fails_the_check_and_is_named() {
        let mut palette = PALETTE;
        palette.light.accent_line = colour("#d8d2c4");
        let error = check(&palette).unwrap_err();
        assert_eq!(error.scheme, Scheme::Light);
        assert_eq!(error.foreground, Role::AccentLine);
        assert_eq!(error.background, Role::Page);
        assert_eq!(error.threshold, 3.0);
        assert!(error.ratio < 3.0);
        let text = error.to_string();
        assert!(text.starts_with("light scheme: accent-line on page has a contrast of "));
        assert!(text.ends_with("below the 3.00:1 its use needs"));
    }

    #[test]
    fn the_unrounded_ratio_is_compared() {
        // #fd8322 on the dark code surface is 6.99998:1: it prints as 7.00, yet stays below 7.
        let mut palette = PALETTE;
        palette.dark.accent_code = colour("#fd8322");
        let error = check(&palette).unwrap_err();
        assert_eq!(error.foreground, Role::AccentCode);
        assert_eq!(error.background, Role::CodeSurface);
        assert_eq!(format!("{:.2}", error.ratio), "7.00");
        assert!(error.ratio < error.threshold);
    }

    #[test]
    fn the_css_holds_the_dark_values_in_the_theme_and_the_light_ones_in_the_media_query() {
        let text = css(&PALETTE);
        let (before_media, light_block) = text.split_once("@media").unwrap();
        assert!(text.starts_with("/* Generated by the build"));
        assert!(before_media.contains("@theme {\n  --color-page: #000000;\n"));
        assert!(before_media.contains(":root {\n  color-scheme: dark;\n}"));
        assert!(light_block.contains("(prefers-color-scheme: light)"));
        assert!(light_block.contains("color-scheme: light;"));
        for role in Role::ALL {
            let dark = format!("--color-{}: {};", role.name(), PALETTE.dark.of(role));
            let light = format!("--color-{}: {};", role.name(), PALETTE.light.of(role));
            assert_eq!(before_media.matches(&dark).count(), 1, "{dark}");
            assert!(!before_media.contains(&light) || dark == light);
            assert_eq!(light_block.matches(&light).count(), 1, "{light}");
        }
        assert!(!light_block.contains("#000000"));
    }
}
