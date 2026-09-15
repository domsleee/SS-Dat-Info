import { expect, test } from 'bun:test';
import { formatNotes, mergeNotes } from './release.mjs';

const repository = 'domsleee/SS-Dat-Info';
const pr = number => `https://github.com/${repository}/pull/${number}`;
const changelog = `[Full changelog](https://github.com/${repository}/compare/0.4.5...0.4.6)`;

test('shortens GitHub notes while keeping other authors and title text', () => {
  const notes = [
    `* Owner fix by @domsleee in ${pr(53)}`,
    `* Community fix by @contributor in ${pr(54)}`,
    `* Similar username by @domsleee-extra in ${pr(55)}`,
    `* Mention by @domsleee in the title by @contributor in ${pr(56)}`,
    '', '', `**Full Changelog**: https://github.com/${repository}/compare/0.4.5...0.4.6`,
  ].join('\r\n');

  expect(formatNotes(notes).replaceAll('\r\n', '\n')).toBe([
    `* Owner fix ([#53](${pr(53)}))`,
    `* Community fix by @contributor ([#54](${pr(54)}))`,
    `* Similar username by @domsleee-extra ([#55](${pr(55)}))`,
    `* Mention by @domsleee in the title by @contributor ([#56](${pr(56)}))`,
    '', changelog,
  ].join('\n'));
});

const generated = [
  `* Old change rewritten or omitted by the maintainer ([#42](${pr(42)}))`,
  `* Already covered fix ([#49](${pr(49)}))`,
  `* New fix ([#53](${pr(53)}))`,
  `* Community fix by @contributor ([#54](${pr(54)}))`,
  '', '## New Contributors',
  `* @contributor made their first contribution ([#54](${pr(54)}))`,
  '', changelog,
].join('\n');

test('preserves edits and appends only new PRs before the installation and changelog footer', () => {
  const summary = 'My rewritten summary, with no original PR links.\n\nSecurity fix covered here (#49).';
  const installation = `[Installation instructions](https://github.com/${repository}/wiki/Display_Config)`;
  const body = `${summary}\n\n${installation} · [Full changelog](https://github.com/${repository}/compare/old...older)`;
  const merged = mergeNotes(body, generated, repository, new Set(['49', '53', '54']));

  expect(merged).toBe([
    summary, '', `* New fix ([#53](${pr(53)}))`,
    `* Community fix by @contributor ([#54](${pr(54)}))`,
    '', `${installation} · ${changelog}`,
  ].join('\n'));
  expect(mergeNotes(merged, generated, repository, new Set())).toBe(merged);
  expect(mergeNotes(merged, generated, repository, new Set(['53', '54']))).toBe(merged);
});

test('recognizes bare PR numbers and full URLs in legacy CRLF notes', () => {
  const body = `Edited (#53)\r\n\r\nContributor fix: ${pr(54)}\r\n\r\n**Full Changelog**: https://github.com/${repository}/compare/old...older`;
  const notes = `* Original title ([#53](${pr(53)}))\n* Community fix ([#54](${pr(54)}))\n\n${changelog}`;
  expect(mergeNotes(body, notes, repository, null)).toBe(
    `Edited (#53)\r\n\r\nContributor fix: ${pr(54)}\r\n\r\n${changelog}`,
  );
});

test('adds new bullets and a missing footer to handwritten notes', () => {
  expect(mergeNotes('My summary.', generated, repository, new Set(['53']))).toBe(
    `My summary.\n\n* New fix ([#53](${pr(53)}))\n\n${changelog}`,
  );
});

test.each([undefined, '', ' \r\n'])('uses all generated notes for a new or empty draft (%j)', body => {
  expect(mergeNotes(body, generated, repository, null)).toBe(generated);
});
