import { test, expect } from '@playwright/test';
import { execFileSync } from 'node:child_process';
import { readdirSync, readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { live } from './support';

// Why this check exists: end2end/README.md must list every browser test
// exactly once, a rule of the project's CLAUDE.md. The agent keeps the README
// in step with the tests; this check makes the report certain instead of
// leaving it to someone remembering (card:
// docs/wip/describe-the-browser-tests-in-a-readme.md).
//
// How it works. Playwright itself lists the declarations (`playwright test
// --list`): a declaration is one place in the code, a file and a line, even
// when it produces many runs. The README's test rows are the lines whose
// first two cells are each one code span: the text to search for, and the
// file. Every problem found is collected, then reported together:
//   - the text to search for is not in the test files exactly once;
//   - its only place is not in the file the row names;
//   - its only place is not a line where Playwright reports a declaration;
//   - a declaration has no row, or more than one.

type Place = { file: string; line: number };
type Suite = { specs?: { file: string; line: number }[]; suites?: Suite[] };

const END2END = resolve(__dirname, '..');
const TESTS = resolve(END2END, 'tests');
const ROW = /^\| `([^`]+)` \| `([^`]+)` \|/;

const declarations = (): Place[] => {
  const cli = require.resolve('@playwright/test/cli');
  const output = execFileSync(process.execPath, [cli, 'test', '--list', '--reporter=json'], {
    cwd: END2END,
    env: process.env,
    encoding: 'utf8',
    maxBuffer: 64 * 1024 * 1024,
  });
  const found = new Map<string, Place>();
  const walk = (suite: Suite) => {
    for (const spec of suite.specs ?? []) {
      found.set(`${spec.file}:${spec.line}`, { file: spec.file, line: spec.line });
    }
    (suite.suites ?? []).forEach(walk);
  };
  (JSON.parse(output).suites as Suite[]).forEach(walk);
  return [...found.values()];
};

test('the README lists every browser test once', () => {
  test.skip(live, 'checked in the build only: the README is documentation, not the site');
  test.setTimeout(60_000);

  const declared = declarations();
  const rows = readFileSync(resolve(END2END, 'README.md'), 'utf8')
    .split('\n')
    .flatMap((text) => {
      const match = ROW.exec(text);
      return match ? [{ key: match[1], file: match[2] }] : [];
    });
  const sources = readdirSync(TESTS)
    .filter((name) => name.endsWith('.ts'))
    .map((name) => ({ name, lines: readFileSync(resolve(TESTS, name), 'utf8').split('\n') }));

  const problems: string[] = [];
  const reported = new Set(declared.map((d) => `${d.file}:${d.line}`));

  for (const row of rows) {
    const places = sources.flatMap(({ name, lines }) =>
      lines.flatMap((text, i) => (text.includes(row.key) ? [`${name}:${i + 1}`] : [])),
    );
    if (places.length !== 1) {
      problems.push(
        `README row \`${row.key}\` is found ${places.length} times in end2end/tests/ (${places.join(', ') || 'nowhere'}): change its first column to a piece of the declaring line that appears exactly once`,
      );
      continue;
    }
    const [name, line] = places[0].split(':');
    if (name !== row.file) {
      problems.push(`README row \`${row.key}\` names ${row.file}, but the text is in ${places[0]}: fix the File column`);
    }
    if (!reported.has(`${name}:${line}`)) {
      problems.push(`README row \`${row.key}\` points at ${places[0]}, which is not a test declaration: use a piece of the line that declares the test`);
    }
  }

  for (const d of declared) {
    const name = d.file.split('/').pop() ?? d.file;
    const text = sources.find((s) => s.name === name)?.lines[d.line - 1]?.trim() ?? '';
    const matching = rows.filter((row) => row.file === name && text.includes(row.key)).length;
    if (matching !== 1) {
      problems.push(
        `${matching === 0 ? 'No' : `${matching}`} README row${matching === 1 ? '' : 's'} for the test declared at ${name}:${d.line} (\`${text}\`): the README must list it exactly once`,
      );
    }
  }

  expect(problems, `end2end/README.md and the declared tests disagree:\n- ${problems.join('\n- ')}\n`).toEqual([]);
});
