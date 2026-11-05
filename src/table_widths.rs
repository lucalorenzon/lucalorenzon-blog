//! Estimates, at build time, how wide a Markdown table needs to be to stay readable, and how
//! wide the label column of its record viewer should be (see `markdown.rs`).
//!
//! The text of a table is in a proportional font (Literata), so the build cannot know its width
//! exactly; it estimates it from the advance width of each character (`literata_widths.rs`,
//! measured in Chromium). Kerning is ignored: it mostly makes pairs narrower, so the estimate
//! errs on the wide side, and the table switches to the viewer a little early, never late.
//!
//! Every width here is an integer count of `UNIT` = 1/1 024 000 em. The unit is chosen so that
//! all the inputs are whole numbers (Literata's thousandths of em times 1 024, and Hack's
//! 0.85 x 1233 / 2048 em times 1 024 000 = 524 025), which keeps the arithmetic exact and the
//! tests hand-computable. Results are rounded up when they leave this module.
//!
//! The rule (card "Markdown annotations for wide content", slice 2): each column needs the
//! larger of its longest word and the smaller of `M` and its content on one line (body cells
//! only, see `estimate`); `M` is 17 times the average character width of running English text
//! in Literata regular (see `readable_line`). A word
//! is a run without whitespace; a cell with no whitespace is one word. The table's threshold is
//! the sum of the columns' needs, plus the cells' side padding (0.5em each side) and the 1px
//! lines between columns.

use crate::literata_widths::{LITERATA_BOLD, LITERATA_REGULAR};
use pulldown_cmark::{Event, Tag, TagEnd};

/// One em, in units.
const EM: u64 = 1_024_000;
/// Literata's table is in thousandths of em.
const FROM_THOUSANDTHS: u64 = EM / 1000;
/// Advance of a character of Hack at 0.85em of the text: 0.85 x 1233 / 2048 em.
const CODE_CHAR: u64 = 524_025;
/// A column line, 1px, in em at the smallest text size (18px), rounded up.
const COLUMN_LINE_DIVISOR: u64 = 18;
/// Side padding of a cell: 0.5em each side.
const CELL_PADDING: u64 = EM;
/// Characters of a line of running text that a column asks for.
const READABLE_CHARACTERS: u64 = 17;
/// Letter frequencies of English text, in thousandths of a percent, a to z (Lewand, "Cryptological
/// Mathematics", 2000, the table most often quoted).
const ENGLISH_LETTER_FREQUENCY: [u64; 26] = [
    8167, 1492, 2782, 4253, 12702, 2228, 2015, 6094, 6966, 153, 772, 4025, 2406, 6749, 7507, 1929,
    95, 5987, 6327, 9056, 2758, 978, 2360, 150, 1974, 74,
];
/// An average English word, in tenths of a letter, followed by one space.
const LETTERS_PER_WORD_TENTHS: u64 = 47;
/// Extra room for the label column, 0.1em, before rounding up: Chromium at 1x pixel density rounds
/// each glyph advance to whole pixels, which made "Description" 1px wider than estimated.
const LABEL_MARGIN: u64 = EM / 10;

/// The cells of a table, each one the events between its start and end tags.
pub struct Cells<'a> {
    pub header: Vec<Vec<Event<'a>>>,
    pub rows: Vec<Vec<Vec<Event<'a>>>>,
}

impl<'a> Cells<'a> {
    /// Splits the events of one table (from its start tag to its end tag).
    pub fn from_events(table: &[Event<'a>]) -> Self {
        let mut header = Vec::new();
        let mut rows = Vec::new();
        let mut row: Vec<Vec<Event<'a>>> = Vec::new();
        let mut cell: Option<Vec<Event<'a>>> = None;
        for event in table {
            match event {
                Event::Start(Tag::TableCell) => cell = Some(Vec::new()),
                Event::End(TagEnd::TableCell) => row.extend(cell.take()),
                Event::End(TagEnd::TableHead) => header = std::mem::take(&mut row),
                Event::End(TagEnd::TableRow) => rows.push(std::mem::take(&mut row)),
                other => {
                    if let Some(cell) = cell.as_mut() {
                        cell.push(other.clone());
                    }
                }
            }
        }
        Cells { header, rows }
    }

    pub fn columns(&self) -> usize {
        self.header.len()
    }
}

/// What the build needs to know about a table.
#[derive(Debug, PartialEq, Eq)]
pub struct Estimate {
    /// Width below which the table is replaced by the viewer, in thousandths of em, rounded up.
    pub threshold_thousandths: u64,
    /// Width of the viewer's label column, in tenths of em, rounded up.
    pub label_tenths: u64,
}

pub fn estimate(cells: &Cells) -> Estimate {
    let columns = cells.columns() as u64;
    let readable = readable_line();
    let mut need_sum = 0;
    for column in 0..cells.columns() {
        let mut longest_word = 0;
        let mut widest_line = 0;
        let header = cells.header.get(column).map(|cell| (cell, true));
        let body = cells
            .rows
            .iter()
            .filter_map(|row| row.get(column))
            .map(|cell| (cell, false));
        for (cell, bold) in header.into_iter().chain(body) {
            let width = measure(cell, bold);
            longest_word = longest_word.max(width.longest_word);
            // A header is a short bold label that reads fine on two lines: it asks only for its
            // longest word. Counted as a line, a two-word header would make a column of short
            // values ask for M.
            if !bold {
                widest_line = widest_line.max(width.line);
            }
        }
        need_sum += longest_word.max(widest_line.min(readable));
    }
    let lines = (columns.saturating_sub(1) * EM).div_ceil(COLUMN_LINE_DIVISOR);
    let threshold = need_sum + columns * CELL_PADDING + lines;

    // The label is the header cell in bold, on one line, with the padding of a cell and a margin.
    let label = cells
        .header
        .iter()
        .map(|cell| measure(cell, true).line)
        .max()
        .unwrap_or(0)
        + CELL_PADDING
        + LABEL_MARGIN;
    Estimate {
        threshold_thousandths: threshold.div_ceil(FROM_THOUSANDTHS),
        label_tenths: label.div_ceil(EM / 10),
    }
}

/// `M`: 17 times the average character width of running English text in Literata regular,
/// rounded up. The average is that of an average word, `LETTERS_PER_WORD_TENTHS` letters weighted
/// by English letter frequency plus one space: (4.7 x weighted letter width + space) / 5.7. Not the
/// plain mean of a to z, because M was chosen by eye on real text, whose average character is
/// narrower than the alphabet's: spaces are narrow and so are the frequent letters (e, t, a, i).
fn readable_line() -> u64 {
    let frequency_sum: u64 = ENGLISH_LETTER_FREQUENCY.iter().sum();
    let weighted: u64 = ('a'..='z')
        .zip(ENGLISH_LETTER_FREQUENCY)
        .map(|(c, frequency)| frequency * u64::from(advance(&LITERATA_REGULAR, c)))
        .sum();
    let space = u64::from(advance(&LITERATA_REGULAR, ' '));
    // average = (47 x weighted / frequency_sum + 10 x space) / 57, in thousandths of em.
    let numerator = LETTERS_PER_WORD_TENTHS * weighted + 10 * space * frequency_sum;
    let denominator = (LETTERS_PER_WORD_TENTHS + 10) * frequency_sum;
    (READABLE_CHARACTERS * numerator * FROM_THOUSANDTHS).div_ceil(denominator)
}

/// Advance of a character in thousandths of em. A character outside the measured set takes the
/// widest advance of that weight: conservative, it makes the viewer come early.
fn advance(table: &[(char, u16)], c: char) -> u16 {
    match table.binary_search_by_key(&c, |&(character, _)| character) {
        Ok(index) => table[index].1,
        Err(_) => table.iter().map(|&(_, width)| width).max().unwrap_or(0),
    }
}

struct CellWidth {
    longest_word: u64,
    line: u64,
}

/// Browsers break lines at whitespace except at no-break spaces.
fn breaks_line(c: char) -> bool {
    c.is_whitespace() && !matches!(c, '\u{a0}' | '\u{2007}' | '\u{202f}')
}

/// Width of a cell's longest word and of its whole content on one line. Header cells are bold,
/// and so is text in strong emphasis; italic uses the regular widths; inline code is Hack.
/// Runs of whitespace collapse to one space, and none counts at either end.
fn measure(cell: &[Event], bold_cell: bool) -> CellWidth {
    let mut strong = 0;
    let mut image = 0;
    let mut longest_word = 0;
    let mut word = 0;
    let mut line = 0;
    let mut pending_space = None;
    for event in cell {
        let (text, code) = match event {
            Event::Start(Tag::Strong) => {
                strong += 1;
                continue;
            }
            Event::End(TagEnd::Strong) => {
                strong -= 1;
                continue;
            }
            Event::Start(Tag::Image { .. }) => {
                image += 1;
                continue;
            }
            Event::End(TagEnd::Image) => {
                image -= 1;
                continue;
            }
            Event::Text(text) if image == 0 => (text.as_ref(), false),
            Event::Code(text) => (text.as_ref(), true),
            Event::SoftBreak | Event::HardBreak => (" ", false),
            _ => continue,
        };
        let table = if bold_cell || strong > 0 {
            &LITERATA_BOLD[..]
        } else {
            &LITERATA_REGULAR[..]
        };
        for c in text.chars() {
            let width = if code {
                CODE_CHAR
            } else {
                u64::from(advance(table, c)) * FROM_THOUSANDTHS
            };
            if breaks_line(c) {
                if line > 0 && pending_space.is_none() {
                    pending_space = Some(width);
                }
                word = 0;
            } else {
                if let Some(space) = pending_space.take() {
                    line += space;
                }
                line += width;
                word += width;
                longest_word = longest_word.max(word);
            }
        }
    }
    CellWidth { longest_word, line }
}

#[cfg(test)]
mod tests {
    use super::{Cells, Estimate, estimate, readable_line};
    use pulldown_cmark::{Event, Options, Parser};

    fn estimate_of(markdown: &str) -> Estimate {
        let events: Vec<Event> = Parser::new_ext(markdown, Options::ENABLE_TABLES).collect();
        estimate(&Cells::from_events(&events))
    }

    #[test]
    fn splits_header_and_rows() {
        let events: Vec<Event> = Parser::new_ext(
            "| a | b |\n|---|---|\n| c | d |\n| e | f |\n",
            Options::ENABLE_TABLES,
        )
        .collect();
        let cells = Cells::from_events(&events);
        assert_eq!(cells.columns(), 2);
        assert_eq!(cells.rows.len(), 2);
        assert_eq!(cells.rows[1][1], [Event::Text("f".into())]);
    }

    /// Weighted by English letter frequency the letters average 534.75 thousandths of em
    /// (Literata regular); with one space (200) per 4.7 letters the average character is
    /// (4.7 x 534.75 + 200) / 5.7 = 476.02, so M = 17 x that = 8.0924em, in units 8 286 641.2
    /// rounded up.
    #[test]
    fn readable_line_is_seventeen_average_characters_of_english_text() {
        assert_eq!(readable_line(), 8_286_642);
    }

    #[test]
    fn one_short_column_needs_its_widest_word_and_the_padding() {
        // Header "a" in bold (0.578em) is the widest; plus 1em of padding, no column line.
        let found = estimate_of("| a |\n|---|\n| a |\n");
        assert_eq!(found.threshold_thousandths, 1578);
        // Label: 0.578 + 1 + 0.1 margin = 1.678em, rounded up to 0.1em.
        assert_eq!(found.label_tenths, 17);
    }

    #[test]
    fn column_lines_add_one_pixel_each_in_em_of_the_smallest_text() {
        // 2 x 0.578 + 2 x 1 + 1/18 = 3.2117..., rounded up to 3.212.
        let found = estimate_of("| a | a |\n|---|---|\n| a | a |\n");
        assert_eq!(found.threshold_thousandths, 3212);
        assert_eq!(found.label_tenths, 17);
    }

    #[test]
    fn running_text_asks_for_at_most_m_not_its_whole_line() {
        let words = vec!["a"; 30].join(" ");
        let found = estimate_of(&format!("| x |\n|---|\n| {words} |\n"));
        // M (8.0924em) + 1em of padding: 9.0924, rounded up.
        assert_eq!(found.threshold_thousandths, 9_093);
    }

    #[test]
    fn a_word_longer_than_m_wins_over_m() {
        let word = "a".repeat(30);
        let found = estimate_of(&format!("| x |\n|---|\n| {word} |\n"));
        // 30 x 0.539 = 16.17em + 1em.
        assert_eq!(found.threshold_thousandths, 17_170);
    }

    #[test]
    fn a_short_line_asks_for_its_whole_width_with_spaces() {
        // "a a": 0.539 + 0.2 + 0.539 = 1.278em, wider than the header "i" in bold (0.364).
        let found = estimate_of("| i |\n|---|\n| a a |\n");
        assert_eq!(found.threshold_thousandths, 2_278);
        // The label is the header only: 0.364 + 1 + 0.1 = 1.464em, rounded up to 1.5.
        assert_eq!(found.label_tenths, 15);
    }

    #[test]
    fn inline_code_is_hack_at_85_percent() {
        // Two characters: 2 x 0.85 x 1233 / 2048 = 1.02349em (1.0234.. rounded up with the
        // padding to 2.024).
        let found = estimate_of("| a |\n|---|\n| `ab` |\n");
        assert_eq!(found.threshold_thousandths, 2024);
    }

    #[test]
    fn strong_text_is_measured_in_bold() {
        // Header "i" is 0.364 in bold; bold a is 0.578, regular a is 0.539.
        assert_eq!(
            estimate_of("| i |\n|---|\n| **a** |\n").threshold_thousandths,
            1578
        );
        assert_eq!(
            estimate_of("| i |\n|---|\n| a |\n").threshold_thousandths,
            1539
        );
    }

    #[test]
    fn a_character_outside_the_measured_set_takes_the_widest_advance() {
        let widest = super::LITERATA_REGULAR
            .iter()
            .map(|&(_, w)| u64::from(w))
            .max()
            .unwrap();
        // A CJK character is not in the table.
        let found = estimate_of("| a |\n|---|\n| 漢 |\n");
        assert_eq!(found.threshold_thousandths, 1000 + widest);
    }

    #[test]
    fn whitespace_collapses_and_the_ends_do_not_count() {
        let found = estimate_of("| i |\n|---|\n| a   a |\n");
        assert_eq!(found.threshold_thousandths, 2_278);
    }

    #[test]
    fn the_label_is_the_widest_header_not_the_widest_cell() {
        let found = estimate_of("| a | aaaaaaaaaa |\n|---|---|\n| aaaaaaaaaaaaaaa | a |\n");
        // 10 x 0.578 = 5.78em + 1em + 0.1em = 6.88em, rounded up to 6.9.
        assert_eq!(found.label_tenths, 69);
    }

    #[test]
    fn a_header_asks_only_for_its_longest_word() {
        // A long header on one line would ask for M; as a header it asks for its widest word,
        // bold a (0.578), the body cell is regular a (0.539).
        let header = vec!["a"; 30].join(" ");
        let found = estimate_of(&format!("| {header} |\n|---|\n| a |\n"));
        assert_eq!(found.threshold_thousandths, 1578);
    }
}
