import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { test, expect, type Page } from '@playwright/test';
import { ADDRESSES, FOOTER_RANGE, LOGO_RANGE, READER_SPACING, retryWhenLive, sizeOnRange, TITLE_RANGE, VIEWPORTS, XL } from './support';

// Enforces the agreed responsive header layout. It checks what the reader
// sees (positions, sizes, visibility), never how the page is built.
//
// Forms by viewport width, at the breakpoint xl of config/breakpoints.txt (1280 today):
//   < xl       title below the header line, centred
//   >= xl      title in the header row, at most 2 lines
// The logo is the same at every width: a round icon and the name "Lvk@73r" as text. The icon
// grows from 48px at 320px of page to 64px from 1366px (LOGO_RANGE in support.ts); the header's
// line crosses it at 80% of its height; the name, Literata bold italic at the title's size, has its
// baseline on the line, 6px from the icon, and leans 5 degrees more than the italic. Why:
// docs/board/cards/review-the-logo.md
//
// The title's size does not change with the band: it grows with the page's width in a straight
// line, from 24px at 320px to 32px at 1366px, and stays at 24px below and at 32px above
// (TITLE_RANGE in support.ts, `title-size` in style/input.css), within 0.05px. The footer's text
// does the same, from 16px to 18px (FOOTER_RANGE), checked by the last test of this file. The
// tests' browser hides the scrollbars, so the page's width is the window's.
// Why: docs/board/cards/let-the-frame-text-grow-within-the-injected-ranges.md
//
// A second test per viewport applies the reader's text spacing (WCAG 1.4.12,
// READER_SPACING in support.ts) and checks that the header's logo, title and
// menu slot do not intersect, that nothing in the header cuts off its content
// and that the page does not scroll sideways. It uses the home page title and
// the two invented titles below instead of the real pages' titles: the build
// limits a heading to a number of graphemes (card
// docs/board/cards/enforce-the-title-length-limit-in-the-build.md), so the
// longest title any page can have is covered by the invented ones, at every width.
// The rest of the 1.4.12 checks live in readability.spec.ts, at the widths of
// the text column.

const PAD = 8;
const MENU_WIDTH = 56;
const SPACE_UNDER_LINE = 24;
// Where the line crosses the icon, as a share of its height from the top, and the name's distance
// from the icon, in pixels.
const LINE_AT = 0.8;
const NAME_GAP = 6;
const SKEW_DEGREES = 5;
const MIN_GAP = 24;
// How far a measured text size may be from its range's straight line, in pixels.
const SIZE_TOLERANCE = 0.05;

const HOME_TITLE = 'Home Page';
// The longest heading the build accepts, in graphemes. It is read from the same file the
// build includes (config/heading-max-graphemes.txt), so there is one figure, not two copies that
// could drift apart; and it is counted in graphemes, as the build does. The tests also run on
// the live site, always from a checkout of the repository, so the file is on the disk there too.
const HEADING_MAX_GRAPHEMES = Number.parseInt(
  readFileSync(resolve(__dirname, '../../config/heading-max-graphemes.txt'), 'utf8').trim(),
  10,
);
const graphemes = (text: string) =>
  [...new Intl.Segmenter(undefined, { granularity: 'grapheme' }).segment(text)].length;

// Invented titles, each exactly as long as the limit lets a real one be.
const LONG_TITLES = [
  'Disaccoppiamento infrastrutturale nei microservizi',
  'Contestualizzazione dell’interoperabilità nei dati',
];

type Box = { l: number; r: number; t: number; b: number };
type Part = { visible: boolean; box: Box };
type Name = Part & {
  fontSize: number;
  family: string;
  weight: string;
  style: string;
  transform: string;
  baseline: number;
  gapFromIcon: number;
};
type Snapshot = {
  innerWidth: number;
  scrollWidth: number;
  h1Count: number;
  h1Visible: boolean;
  h1Box: Box;
  h1Text: Box;
  h1FontSize: number;
  h1LineHeight: number;
  icon: Part | null;
  name: Name | null;
  drawnLogos: number;
  menu: { visible: boolean; box: Box } | null;
  line: { y: number; bottom: number } | null;
};


// ---- Measuring (runs in the browser) -------------------------------------

async function snapshot(page: Page): Promise<Snapshot> {
  return page.evaluate(() => {
    const boxOf = (r: DOMRect) => ({ l: r.left, r: r.right, t: r.top, b: r.bottom });

    // Box of the text itself, not of the element that contains it.
    function textBox(el: Element) {
      const range = document.createRange();
      range.selectNodeContents(el);
      return boxOf(range.getBoundingClientRect());
    }

    function isVisible(el: Element) {
      const style = getComputedStyle(el);
      const rect = el.getBoundingClientRect();
      return (
        style.display !== 'none' &&
        style.visibility === 'visible' &&
        Number(style.opacity) > 0 &&
        rect.width > 0 &&
        rect.height > 0
      );
    }

    // The header line is the topmost full-width visible horizontal border
    // (the footer has one too, near the bottom of the page).
    function findHeaderLine() {
      let best: { y: number; bottom: number } | null = null;
      for (const el of Array.from(document.querySelectorAll('*'))) {
        const style = getComputedStyle(el);
        const rect = el.getBoundingClientRect();
        if (rect.width < 0.9 * window.innerWidth) continue;
        const top = parseFloat(style.borderTopWidth);
        if (top > 0 && style.borderTopStyle !== 'none') {
          const y = rect.top;
          if (!best || y < best.y) best = { y, bottom: y + top };
        }
        const bottom = parseFloat(style.borderBottomWidth);
        if (bottom > 0 && style.borderBottomStyle !== 'none') {
          const y = rect.bottom - bottom;
          if (!best || y < best.y) best = { y, bottom: rect.bottom };
        }
      }
      return best;
    }

    const h1s = Array.from(document.querySelectorAll('h1'));
    const h1 = h1s[0];
    const iconEl = document.querySelector('a.logo-link img');
    const nameEl = document.querySelector('a.logo-link .logo-name') as HTMLElement | null;
    // The box of the name has no height (line-height 0); its text, in a span, has.
    const nameText = nameEl?.firstElementChild ?? null;
    // Anything of the two old forms: the logo drawn in characters, or in the pixel font.
    const drawnLogos =
      document.querySelectorAll('[data-testid="ascii-logo"], header pre, .text-logo-large, .art-logo-left').length +
      Array.from(document.querySelectorAll('header *')).filter((el) =>
        /VGA|pixel/i.test(getComputedStyle(el).fontFamily),
      ).length;
    // The baseline of the name: where a zero-size inline-block, set on the baseline, ends.
    function baselineOf(el: HTMLElement) {
      const probe = document.createElement('span');
      probe.style.cssText = 'display:inline-block;width:0;height:0;vertical-align:baseline';
      el.appendChild(probe);
      const y = probe.getBoundingClientRect().bottom;
      probe.remove();
      return y;
    }
    const menu = document.querySelector('[data-testid="menu-slot"]');
    const h1Style = h1 ? getComputedStyle(h1) : null;
    const fontSize = h1Style ? parseFloat(h1Style.fontSize) : NaN;
    const empty = { l: NaN, r: NaN, t: NaN, b: NaN };

    return {
      innerWidth: window.innerWidth,
      scrollWidth: document.documentElement.scrollWidth,
      h1Count: h1s.length,
      h1Visible: h1 ? isVisible(h1) : false,
      h1Box: h1 ? boxOf(h1.getBoundingClientRect()) : empty,
      h1Text: h1 ? textBox(h1) : empty,
      h1FontSize: fontSize,
      // "normal" is not a number: treat it as the font size
      h1LineHeight: h1Style ? parseFloat(h1Style.lineHeight) || fontSize : NaN,
      icon: iconEl ? { visible: isVisible(iconEl), box: boxOf(iconEl.getBoundingClientRect()) } : null,
      name:
        nameEl && iconEl
          ? {
              visible: nameText ? isVisible(nameText) : false,
              box: textBox(nameEl),
              fontSize: parseFloat(getComputedStyle(nameEl).fontSize),
              family: getComputedStyle(nameEl).fontFamily,
              weight: getComputedStyle(nameEl).fontWeight,
              style: getComputedStyle(nameEl).fontStyle,
              transform: getComputedStyle(nameEl).transform,
              baseline: baselineOf(nameEl),
              // Layout distance, which the skew does not change.
              gapFromIcon: nameEl.offsetLeft - ((iconEl as HTMLElement).offsetLeft + (iconEl as HTMLElement).offsetWidth),
            }
          : null,
      drawnLogos,
      menu: menu ? { visible: isVisible(menu), box: boxOf(menu.getBoundingClientRect()) } : null,
      line: findHeaderLine(),
    };
  });
}

// ---- Rules (run in Node) -------------------------------------------------

const near = (a: number, b: number, tol: number) => Math.abs(a - b) <= tol;
const overlap = (a: Box, b: Box) => a.l < b.r - 0.5 && b.l < a.r - 0.5 && a.t < b.b - 0.5 && b.t < a.b - 0.5;
const fmt = (n: number) => (Number.isFinite(n) ? n.toFixed(1) : String(n));

// Returns one message per broken rule; empty means the layout is as agreed.
function violations(s: Snapshot, title: string, centred: boolean): string[] {
  const w = s.innerWidth;
  const wide = w >= XL;
  const where = `[${w}px, "${title}"]`;
  const out: string[] = [];
  const rule = (ok: boolean, message: string) => {
    if (!ok) out.push(`${where} ${message}`);
  };

  // Everywhere: one visible title, no horizontal scroll
  rule(s.h1Count === 1, `exactly one h1 expected, found ${s.h1Count}`);
  rule(s.h1Visible, 'the h1 must be visible');
  rule(s.scrollWidth <= w, `no horizontal scroll: scrollWidth ${s.scrollWidth} > innerWidth ${w}`);

  // Logo: the icon and the name, at every width, and nothing drawn
  const icon = s.icon;
  const name = s.name;
  rule(!!icon?.visible, 'the round icon must be visible');
  rule(!!name?.visible, 'the name "Lvk@73r" must be visible');
  rule(s.drawnLogos === 0, `no logo drawn in characters or in the pixel font may exist, found ${s.drawnLogos}`);
  if (icon?.visible) {
    const expectedWidth = sizeOnRange(LOGO_RANGE, w);
    const width = icon.box.r - icon.box.l;
    rule(near(width, expectedWidth, 0.5), `icon width must be ${expectedWidth.toFixed(1)}px, is ${fmt(width)}px`);
    rule(near(icon.box.b - icon.box.t, width, 0.5), 'the icon must be square');
  }
  if (name?.visible) {
    const expectedSize = sizeOnRange(TITLE_RANGE, w);
    rule(near(name.fontSize, expectedSize, SIZE_TOLERANCE), `name font-size must be the title's, ${expectedSize.toFixed(2)}px, is ${name.fontSize}px`);
    rule(/^"?Literata"?/.test(name.family), `name must be in Literata, is in ${name.family}`);
    rule(name.weight === '700', `name must be bold (700), is ${name.weight}`);
    rule(name.style === 'italic', `name must be italic, is ${name.style}`);
    rule(near(name.gapFromIcon, NAME_GAP, 1), `name must be ${NAME_GAP}px from the icon, is ${fmt(name.gapFromIcon)}px`);
    const matrix = /^matrix\(([^)]+)\)$/.exec(name.transform)?.[1].split(',').map(Number);
    const skew = matrix ? (Math.atan(matrix[2]) * 180) / Math.PI : NaN;
    rule(near(skew, -SKEW_DEGREES, 0.1), `name must be skewed by ${SKEW_DEGREES} degrees more, transform is ${name.transform}`);
  }
  const logo = icon?.visible && name?.visible ? { box: { l: icon.box.l, r: name.box.r, t: icon.box.t, b: icon.box.b } } : null;

  // Title size and number of lines
  const expectedSize = sizeOnRange(TITLE_RANGE, w);
  rule(
    Math.abs(s.h1FontSize - expectedSize) <= SIZE_TOLERANCE,
    `h1 font-size must be ${expectedSize.toFixed(2)}px, on its range's line, is ${s.h1FontSize}px`,
  );
  const lines = Math.round((s.h1Box.b - s.h1Box.t) / s.h1LineHeight);
  if (wide) rule(lines <= 2, `the title must take at most 2 lines, takes ${lines}`);

  // Menu slot
  const menu = s.menu;
  rule(!!menu?.visible, 'the menu slot (data-testid="menu-slot") must exist and be visible');
  if (menu?.visible) {
    const width = menu.box.r - menu.box.l;
    rule(near(width, MENU_WIDTH, 1), `menu slot must be ${MENU_WIDTH}px wide, is ${fmt(width)}px`);
    rule(
      near(menu.box.r, w - PAD, 1),
      `menu slot right edge must be at ${w - PAD}, is at ${fmt(menu.box.r)} (side padding ${PAD}px)`,
    );
  }
  if (logo) {
    rule(near(logo.box.l, PAD, 1), `logo left edge must be at ${PAD}, is at ${fmt(logo.box.l)} (side padding ${PAD}px)`);
  }

  // The header line
  const line = s.line;
  rule(!!line, 'a full-width header line (visible top or bottom border) must exist');
  if (line) {
    // The line crosses the icon at 80% of its height, and the name stands on it. The line's
    // thickness is part of the line: it is measured at its middle.
    const middle = (line.y + line.bottom) / 2;
    if (icon?.visible) {
      const at = icon.box.t + LINE_AT * (icon.box.b - icon.box.t);
      rule(near(middle, at, 1), `the line (${fmt(middle)}) must cross the icon at 80% of its height (${fmt(at)})`);
    }
    if (name?.visible) {
      rule(near(name.baseline, middle, 1), `the name's baseline (${fmt(name.baseline)}) must be on the header line (${fmt(middle)})`);
    }
    if (wide) {
      rule(s.h1Box.b <= line.y + 0.5, `title bottom (${fmt(s.h1Box.b)}) must be above the header line (${fmt(line.y)})`);
    } else {
      const space = s.h1Text.t - line.bottom;
      rule(near(space, SPACE_UNDER_LINE, 1.5), `title text must start ${SPACE_UNDER_LINE}px under the header line, is ${fmt(space)}px`);
    }
  }

  // No overlaps among logo, title text and menu slot
  if (logo && overlap(logo.box, s.h1Text)) out.push(`${where} the title overlaps the logo`);
  // Below xl the circle ends above the title's text, which keeps 24px under the line: the
  // gap the user approved (12 to 15px) stays.
  if (!wide && icon?.visible) rule(icon.box.b < s.h1Text.t, `the icon's bottom (${fmt(icon.box.b)}) must be above the title's text (${fmt(s.h1Text.t)})`);
  if (menu?.visible && overlap(s.h1Text, menu.box)) out.push(`${where} the title overlaps the menu slot`);
  if (logo && menu?.visible && overlap(logo.box, menu.box)) out.push(`${where} the logo overlaps the menu slot`);

  // Title in the header row: keep away from logo and menu slot
  const centre = (s.h1Text.l + s.h1Text.r) / 2;
  const isCentred = near(centre, w / 2, 2);
  if (wide) {
    // The title is in a proportional font and, on two lines, centred inside
    // its column: the text starts further right than the column, which keeps
    // the 24px from the logo. So a wrapped title is measured on the h1 box;
    // a one-line title on its text.
    const twoLines = s.h1Text.b - s.h1Text.t > 1.5 * s.h1LineHeight;
    const logoGap = logo ? (twoLines ? s.h1Box.l : s.h1Text.l) - logo.box.r : NaN;
    const menuGap = menu ? menu.box.l - s.h1Text.r : NaN;
    rule(logoGap >= MIN_GAP - 0.5, `title must be at least ${MIN_GAP}px from the logo, is ${fmt(logoGap)}px`);
    rule(menuGap >= MIN_GAP - 0.5, `title must be at least ${MIN_GAP}px from the menu slot, is ${fmt(menuGap)}px`);
    if (!centred && !isCentred) {
      // No room to centre: shifted right, just enough to keep the gap to the logo
      rule(centre > w / 2, `title must be shifted toward the right when it cannot be centred (centre ${fmt(centre)}, page centre ${w / 2})`);
      rule(near(logoGap, MIN_GAP, 1.5), `a shifted title must keep exactly ${MIN_GAP}px from the logo, keeps ${fmt(logoGap)}px`);
    }
  }
  if (centred) {
    rule(isCentred, `title must be centred on the page: its centre is ${fmt(centre)}, the page centre ${w / 2}`);
  }
  return out;
}

// ---- Tests ---------------------------------------------------------------

async function setTitle(page: Page, title: string) {
  await page.evaluate((text) => {
    document.querySelector('h1')!.textContent = text;
  }, title);
  await page.evaluate(() => new Promise((done) => requestAnimationFrame(() => done(null))));
}

for (const { name, width, height } of VIEWPORTS) {
  test(`header layout at ${width}x${height} (${name})`, async ({ page }, testInfo) => {
    expect(HEADING_MAX_GRAPHEMES).toBeGreaterThan(0);
    for (const title of LONG_TITLES) expect(graphemes(title)).toBe(HEADING_MAX_GRAPHEMES);
    await page.setViewportSize({ width, height });

    let screenshot: Buffer | undefined;
    const check = async () => {
      await page.goto('./');
      await page.evaluate(() => document.fonts.ready);

      screenshot = await page.screenshot({ fullPage: true });
      const found = violations(await snapshot(page), HOME_TITLE, true);

      for (const title of LONG_TITLES) {
        await setTitle(page, title);
        found.push(...violations(await snapshot(page), title, false));
      }
      expect(found.length, `layout rules broken:\n${found.join("\n")}`).toBe(0);
    };

    try {
      await retryWhenLive(check);
    } finally {
      // For a human to review in the report: no baseline comparison.
      if (screenshot) {
        await testInfo.attach(`page-${width}x${height}.png`, { body: screenshot, contentType: 'image/png' });
      }
    }
  });
}

// ---- The logo on every page ---------------------------------------------------

// The frame is the same on every page, but a page could still hide or replace the logo, or the
// longest title could reach under the circle. Every page of the built site, at every width.
for (const { name, width, height } of VIEWPORTS) {
  test(`the logo on every page at ${width}x${height} (${name})`, async ({ page }) => {
    await page.setViewportSize({ width, height });
    test.skip(ADDRESSES.length === 0, 'a live run has no folder of pages to walk');
    const found: string[] = [];
    for (const address of ADDRESSES) {
      await page.goto(address);
      await page.evaluate(() => document.fonts.ready);
      const s = await snapshot(page);
      const where = `[${address}, ${width}px]`;
      if (!s.icon?.visible) found.push(`${where} the icon is not visible`);
      if (!s.name?.visible) found.push(`${where} the name is not visible`);
      if (s.drawnLogos !== 0) found.push(`${where} a drawn or pixel logo exists`);
      if (width < XL && s.icon && s.icon.box.b >= s.h1Text.t) {
        found.push(`${where} the icon's bottom ${fmt(s.icon.box.b)} reaches the title's text at ${fmt(s.h1Text.t)}`);
      }
    }
    expect(found.length, `logo rules broken:\n${found.join('\n')}`).toBe(0);
  });
}

// ---- The line under the name ------------------------------------------------

// The header's line disappears under the name, like a thread that runs inside the letters: it
// reaches the foot of the "L" and comes out at the foot of the "r" (`logo-name` in
// style/input.css, which hides it with a background of the page's colour trimmed by 0.32em, a
// number that belongs to the "r" of Literata bold italic). Measured on the real pixels, not on
// the CSS values, in two shots of the rows the line covers, taken at the same layout:
//  - the line alone: the name's letters made transparent (the background stays), so what shows
//    in those rows is the line, and a hole where the background hides it;
//  - the letters alone: the line's colour made transparent, so what shows in the two rows just
//    above is the ink of the letters at their feet (INK_ROWS).
// The line must end at the first inked column and come back at the one after the last, within
// the margins below; between them nothing of it may show; before and after it must be whole, from the
// icon's right edge to the page's right edge. The background's box must not reach the icon or
// the title's text. Why: docs/board/cards/hide-the-header-line-under-the-logo-name.md
// The margins, in pixels, between the line's ends and the letters' feet. One pixel is the rule,
// agreed with the user: each end is read on whole columns of an antialiased image. The one
// exception is the "r", where the line may come back up to 2 pixels before the end of its foot:
// it then runs under the last bit of the foot, which is drawn over it, so it does not show
// (measured at 1024px: back at 189, the foot ends at 191). It may not come back late: a gap
// between the foot and the line would show.
const THREAD_L_MARGIN = 1;
const THREAD_R_EARLY_MARGIN = 2;
const THREAD_R_LATE_MARGIN = 1;
// The trim of the background, in em (style/input.css, `logo-name`): the box's right end is
// where the "r"'s advance ends, the line comes back this much before it.
const THREAD_TRIM_EM = 0.32;
// The letters' feet are drawn on the rows just above the line's: the text's baseline is snapped
// to a whole pixel, so the row of the line itself holds only what dips below the baseline.
const INK_ROWS = 2;
const LETTERS_HIDDEN = 'a.logo-link .logo-name { color: transparent !important; }';
const LINE_HIDDEN = '.title-below-line { border-top-color: transparent !important; } .header-one-row { border-bottom-color: transparent !important; }';

// The columns of a shot of `rows` pixel rows, from `top`, in which something differs from the
// page's colour, once `css` is added to the page.
async function paintedColumns(page: Page, css: string, top: number, rows: number): Promise<boolean[]> {
  const width = page.viewportSize()!.width;
  await page.evaluate((text) => {
    const style = document.createElement('style');
    style.id = 'measure-only';
    style.textContent = text;
    document.head.appendChild(style);
  }, css);
  await page.evaluate(() => new Promise((done) => requestAnimationFrame(() => done(null))));
  const png = await page.screenshot({ clip: { x: 0, y: top, width, height: rows } });
  await page.evaluate(() => document.getElementById('measure-only')!.remove());
  return page.evaluate(
    async ({ data, rows }) => {
      // The page's colour as the browser computes it, from a probe painted with the colour.
      const probe = document.createElement('div');
      probe.style.background = 'var(--color-page)';
      document.body.appendChild(probe);
      const colour = getComputedStyle(probe).backgroundColor;
      probe.remove();
      const page = (colour.match(/\d+(\.\d+)?/g) ?? []).slice(0, 3).map(Number);
      if (page.length !== 3) throw new Error(`cannot read the page's colour from "${colour}"`);
      const image = new Image();
      image.src = `data:image/png;base64,${data}`;
      await image.decode();
      const canvas = document.createElement('canvas');
      canvas.width = image.width;
      canvas.height = image.height;
      const context = canvas.getContext('2d')!;
      context.drawImage(image, 0, 0);
      const pixels = context.getImageData(0, 0, image.width, image.height).data;
      // A column is painted when its strongest pixel is at least a quarter as far from the
      // page's colour as the strongest of the shot: the faint edge of an antialiased stroke
      // does not count. A shot with nothing in it, only noise, paints nothing.
      const strongest: number[] = [];
      for (let x = 0; x < image.width; x++) {
        let best = 0;
        for (let y = 0; y < rows; y++) {
          const at = (y * image.width + x) * 4;
          const difference =
            Math.abs(pixels[at] - page[0]) + Math.abs(pixels[at + 1] - page[1]) + Math.abs(pixels[at + 2] - page[2]);
          best = Math.max(best, difference);
        }
        strongest.push(best);
      }
      const top = Math.max(...strongest);
      const out = strongest.map((best) => top >= 48 && best * 4 >= top);
      return out;
    },
    { data: png.toString('base64'), rows },
  );
}

// What is wrong with the line under the name in the page as it is now, as messages.
async function threadProblems(page: Page, title: string): Promise<string[]> {
  const width = page.viewportSize()!.width;
  const where = `[${width}px, "${title}"]`;
  const found: string[] = [];
  const s = await snapshot(page);
  if (!s.line || !s.icon || !s.name) return [`${where} the line, the icon or the name is missing`];

  // The rows the line covers (its middle is the name's baseline).
  const top = Math.round(s.line.y);
  const rows = Math.max(1, Math.round(s.line.bottom) - top);
  const from = Math.ceil(s.icon.box.r);
  const line = (await paintedColumns(page, LETTERS_HIDDEN, top, rows)).slice(from);
  const ink = (await paintedColumns(page, LINE_HIDDEN, top - INK_ROWS, INK_ROWS)).slice(from);

  const inkFirst = ink.indexOf(true);
  const inkLast = ink.lastIndexOf(true);
  const hole = line.indexOf(false);
  const holeEnd = line.lastIndexOf(false) + 1;
  if (inkFirst < 0) return [`${where} the letters leave no ink in the line's rows`];
  if (hole < 0) return [`${where} the line is whole under the name: nothing hides it`];

  // Outside the name: whole from the icon's right edge up to the hole, and from it to the right end.
  const before = line.slice(0, hole).filter((painted) => !painted).length;
  const after = line.slice(holeEnd).filter((painted) => !painted).length;
  if (before !== 0) found.push(`${where} the line is not whole between the icon and the name`);
  if (after !== 0 || holeEnd >= line.length) found.push(`${where} the line is not whole from the "r" to its right end`);
  // Under the name: no pixel of the line between the letters.
  const between = line.slice(hole, holeEnd).filter((painted) => painted).length;
  if (between !== 0) found.push(`${where} ${between} columns of the line show between the letters`);
  // The two ends: the foot of the "L" and the foot of the "r".
  if (Math.abs(hole - inkFirst) > THREAD_L_MARGIN) {
    found.push(`${where} at the "L" the line ends at ${from + hole}, the foot starts at ${from + inkFirst}: more than ${THREAD_L_MARGIN}px apart`);
  }
  const back = holeEnd - (inkLast + 1);
  if (back < -THREAD_R_EARLY_MARGIN) {
    found.push(`${where} at the "r" the line comes back at ${from + holeEnd}, too early: the foot ends at ${from + inkLast + 1}, more than ${THREAD_R_EARLY_MARGIN}px after`);
  }
  if (back > THREAD_R_LATE_MARGIN) {
    found.push(`${where} at the "r" the line comes back at ${from + holeEnd}, too late: the foot ends at ${from + inkLast + 1}, more than ${THREAD_R_LATE_MARGIN}px before`);
  }

  // The background covers neither the icon nor the title's text: its box is the text's, trimmed.
  const box = await page.evaluate((trim) => {
    const inner = document.querySelector('a.logo-link .logo-name')!.firstElementChild!;
    const rect = inner.getBoundingClientRect();
    const size = parseFloat(getComputedStyle(inner).fontSize);
    return { l: rect.left, r: rect.right - trim * size, t: rect.top, b: rect.bottom };
  }, THREAD_TRIM_EM);
  if (overlap(box, s.icon.box)) found.push(`${where} the name's background covers the icon`);
  if (overlap(box, s.h1Text)) found.push(`${where} the name's background covers the title's text`);
  return found;
}

for (const { name, width, height } of VIEWPORTS) {
  test(`the header's line under the name at ${width}x${height} (${name})`, { tag: '@colours' }, async ({ page }) => {
    for (const title of LONG_TITLES) expect(graphemes(title)).toBe(HEADING_MAX_GRAPHEMES);
    await page.setViewportSize({ width, height });

    await retryWhenLive(async () => {
      await page.goto('./');
      await page.evaluate(() => document.fonts.ready);

      const found: string[] = [];
      for (const title of [HOME_TITLE, ...LONG_TITLES]) {
        await setTitle(page, title);
        found.push(...(await threadProblems(page, title)));
      }
      expect(found.length, `the line under the name is wrong:\n${found.join('\n')}`).toBe(0);
    });
  });
}

// ---- Header with the reader's text spacing (WCAG 1.4.12) ------------------

// After the reader's spacing is applied: what is wrong in the header, as messages.
async function headerSpacingProblems(page: Page, title: string): Promise<string[]> {
  const where = `[${page.viewportSize()!.width}px, "${title}"]`;
  const found = await page.evaluate(() => {
    const out: string[] = [];
    const label = (el: Element) => `<${el.tagName.toLowerCase()}> "${el.textContent?.slice(0, 30)}"`;
    const header = document.querySelector('header:not(main header)')!;

    if (document.documentElement.scrollWidth > window.innerWidth) {
      out.push(`horizontal scroll: scrollWidth ${document.documentElement.scrollWidth} > innerWidth ${window.innerWidth}`);
    }

    // The boxes of the text itself, not of the elements around it.
    const textBox = (el: Element) => {
      const range = document.createRange();
      range.selectNodeContents(el);
      return range.getBoundingClientRect();
    };
    const parts = Array.from(header.children)
      .filter((el) => getComputedStyle(el).display !== 'none')
      .map((el) => ({
        name: el.getAttribute('data-testid') ?? el.tagName.toLowerCase() + (el.getAttribute('role') ? '[logo]' : ''),
        box: el.textContent?.trim() ? textBox(el) : el.getBoundingClientRect(),
      }))
      .filter((p) => p.box.width > 0 && p.box.height > 0);
    for (let i = 0; i < parts.length; i++) {
      for (let j = i + 1; j < parts.length; j++) {
        const a = parts[i].box;
        const b = parts[j].box;
        if (a.left < b.right - 0.5 && b.left < a.right - 0.5 && a.top < b.bottom - 0.5 && b.top < a.bottom - 0.5) {
          out.push(`header: ${parts[i].name} intersects ${parts[j].name}`);
        }
      }
    }

    // Nothing in the header cut off by its own box.
    for (const el of [header, ...Array.from(header.querySelectorAll('*'))]) {
      const rect = el.getBoundingClientRect();
      if (rect.width === 0 || rect.height === 0) continue;
      const style = getComputedStyle(el);
      const clipsX = style.overflowX !== 'visible';
      const clipsY = style.overflowY !== 'visible';
      if ((clipsX && el.scrollWidth > el.clientWidth + 1) || (clipsY && el.scrollHeight > el.clientHeight + 1)) {
        out.push(
          `${label(el)} cuts off its content (scroll ${el.scrollWidth}x${el.scrollHeight}, box ${el.clientWidth}x${el.clientHeight})`,
        );
      }
    }
    return out;
  });
  return found.map((message) => `${where} ${message}`);
}

for (const { name, width, height } of VIEWPORTS) {
  test(`header with the reader's text spacing at ${width}x${height} (${name})`, async ({ page }) => {
    for (const title of LONG_TITLES) expect(graphemes(title)).toBe(HEADING_MAX_GRAPHEMES);
    await page.setViewportSize({ width, height });

    await retryWhenLive(async () => {
      await page.goto('./');
      await page.addStyleTag({ content: READER_SPACING });
      await page.evaluate(() => document.fonts.ready);

      const found: string[] = [];
      for (const title of [HOME_TITLE, ...LONG_TITLES]) {
        await setTitle(page, title);
        found.push(...(await headerSpacingProblems(page, title)));
      }
      expect(found.length, `header problems with the reader's text spacing:\n${found.join('\n')}`).toBe(0);
    });
  });
}

// ---- Footer text size ------------------------------------------------------

// The footer is part of the page frame, as the header is, and its text grows with the page's
// width on its own range: kept here, with the frame's other tests that run on every device.
for (const { name, width, height } of VIEWPORTS) {
  test(`footer text size at ${width}x${height} (${name})`, async ({ page }) => {
    await page.setViewportSize({ width, height });
    await retryWhenLive(async () => {
      await page.goto('./');
      await page.evaluate(() => document.fonts.ready);
      const size = await page.evaluate(() => parseFloat(getComputedStyle(document.querySelector('footer')!).fontSize));
      const expected = sizeOnRange(FOOTER_RANGE, width);
      expect(
        Math.abs(size - expected),
        `the footer's text follows its range's line at ${width}px: ${expected.toFixed(2)}px expected, ${size}px measured`,
      ).toBeLessThanOrEqual(SIZE_TOLERANCE);
    });
  });
}
