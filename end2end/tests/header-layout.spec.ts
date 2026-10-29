import { test, expect, type Page } from '@playwright/test';
import { retryWhenLive } from './support';

// Enforces the agreed responsive header layout. It checks what the reader
// sees (positions, sizes, visibility), never how the page is built.
//
// Bands by viewport width:
//   < 768    text logo 32px; title below the header line, 24px, centred
//   768-1279 ASCII logo 16px; title below the header line, 32px, centred
//   >= 1280  ASCII logo 16px; title in the header row, 32px, at most 2 lines

const PAD = 8;
const MENU_WIDTH = 56;
const SPACE_UNDER_LINE = 24;
const MIN_GAP = 24;

const VIEWPORTS = [
  { name: 'iPhone SE', width: 320, height: 568 },
  { name: 'iPhone 15', width: 393, height: 659 },
  { name: 'Pixel 7', width: 412, height: 839 },
  { name: 'iPhone 15 landscape', width: 659, height: 393 },
  { name: 'boundary 767', width: 767, height: 800 },
  { name: 'iPad Mini', width: 768, height: 1024 },
  { name: 'iPad 7th gen', width: 810, height: 1080 },
  { name: 'iPad Mini landscape', width: 1024, height: 768 },
  { name: 'iPad Pro 11 landscape', width: 1194, height: 834 },
  { name: 'boundary 1279', width: 1279, height: 800 },
  { name: 'Laptop', width: 1280, height: 720 },
  { name: 'Full HD', width: 1920, height: 1080 },
];

const HOME_TITLE = 'Home Page';
const LONG_TITLES = [
  'Disaccoppiamento infrastrutturale nei microservizi',
  'Contestualizzazione dell’interoperabilità nei dati',
];

type Box = { l: number; r: number; t: number; b: number };
type Logo = { visible: boolean; box: Box; fontSize: number };
type Snapshot = {
  innerWidth: number;
  scrollWidth: number;
  h1Count: number;
  h1Visible: boolean;
  h1Box: Box;
  h1Text: Box;
  h1FontSize: number;
  h1LineHeight: number;
  asciiLogo: Logo | null;
  textLogo: Logo | null;
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

    function logoInfo(el: Element | null) {
      if (!el) return null;
      return {
        visible: isVisible(el),
        box: textBox(el),
        fontSize: parseFloat(getComputedStyle(el).fontSize),
      };
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
    const ascii = document.querySelector('[role="img"][aria-label="Lvk@73r"]');
    const textLogo = Array.from(document.querySelectorAll('body *')).find(
      (el) => el.children.length === 0 && el.textContent?.trim() === 'Lvk@73r',
    );
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
      asciiLogo: logoInfo(ascii),
      textLogo: logoInfo(textLogo ?? null),
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
  const narrow = w < 768;
  const wide = w >= 1280;
  const where = `[${w}px, "${title}"]`;
  const out: string[] = [];
  const rule = (ok: boolean, message: string) => {
    if (!ok) out.push(`${where} ${message}`);
  };

  // Everywhere: one visible title, no horizontal scroll
  rule(s.h1Count === 1, `exactly one h1 expected, found ${s.h1Count}`);
  rule(s.h1Visible, 'the h1 must be visible');
  rule(s.scrollWidth <= w, `no horizontal scroll: scrollWidth ${s.scrollWidth} > innerWidth ${w}`);

  // Logo: which one is shown, and at what size
  const ascii = s.asciiLogo;
  const text = s.textLogo;
  if (narrow) {
    rule(!!text?.visible, 'the text logo "Lvk@73r" must be visible below 768px');
    rule(!ascii?.visible, 'the ASCII-art logo must be hidden below 768px');
    if (text?.visible) rule(text.fontSize === 32, `text logo font-size must be 32px, is ${text.fontSize}px`);
  } else {
    rule(!!ascii?.visible, 'the ASCII-art logo must be visible from 768px');
    rule(!text?.visible, 'the text logo must be hidden from 768px');
    if (ascii?.visible) rule(ascii.fontSize === 16, `ASCII logo font-size must be 16px, is ${ascii.fontSize}px`);
  }
  const logo = (narrow ? text : ascii)?.visible ? (narrow ? text : ascii)! : null;

  // Title size and number of lines
  const expectedSize = narrow ? 24 : 32;
  rule(s.h1FontSize === expectedSize, `h1 font-size must be ${expectedSize}px, is ${s.h1FontSize}px`);
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
    if (logo) rule(logo.box.b <= line.y + 0.5, `logo bottom (${fmt(logo.box.b)}) must be above the header line (${fmt(line.y)})`);
    if (wide) {
      rule(s.h1Box.b <= line.y + 0.5, `title bottom (${fmt(s.h1Box.b)}) must be above the header line (${fmt(line.y)})`);
    } else {
      const space = s.h1Text.t - line.bottom;
      rule(near(space, SPACE_UNDER_LINE, 1.5), `title text must start ${SPACE_UNDER_LINE}px under the header line, is ${fmt(space)}px`);
    }
  }

  // No overlaps among logo, title text and menu slot
  if (logo && overlap(logo.box, s.h1Text)) out.push(`${where} the title overlaps the logo`);
  if (menu?.visible && overlap(s.h1Text, menu.box)) out.push(`${where} the title overlaps the menu slot`);
  if (logo && menu?.visible && overlap(logo.box, menu.box)) out.push(`${where} the logo overlaps the menu slot`);

  // Title in the header row: keep away from logo and menu slot
  const centre = (s.h1Text.l + s.h1Text.r) / 2;
  const isCentred = near(centre, w / 2, 2);
  if (wide) {
    const logoGap = logo ? s.h1Text.l - logo.box.r : NaN;
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
    for (const title of LONG_TITLES) expect(title).toHaveLength(50);
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
