import { spawnSync } from 'node:child_process';
import { appendFileSync, writeFileSync } from 'node:fs';

function gh(args, allowMissing = false) {
  const result = spawnSync('gh', args, { encoding: 'utf8' });
  if (result.error) throw result.error;
  if (result.status !== 0) {
    if (allowMissing && /HTTP 404|release not found/i.test(result.stderr)) return null;
    throw new Error(result.stderr.trim() || `gh ${args[0]} failed (${result.status})`);
  }
  return result.stdout.trimEnd();
}

// Used before the build and again before updating the draft.
function getDraft(version, repository, target) {
  const release = gh(['release', 'view', version, '--repo', repository, '--json', 'isDraft,body,targetCommitish'], true);
  const draft = release === null ? null : JSON.parse(release);
  if (draft && !draft.isDraft) {
    throw new Error(`Release ${version} is already published. Bump the version first.`);
  }

  const ref = gh(['api', `repos/${repository}/git/ref/tags/${version}`], true);
  if (ref !== null) {
    let object = JSON.parse(ref).object;
    while (object.type === 'tag') {
      object = JSON.parse(gh(['api', `repos/${repository}/git/tags/${object.sha}`])).object;
    }
    if (object.sha !== target) {
      throw new Error(`Tag ${version} already points at ${object.sha}, not ${target}. Delete the tag or run the release from that commit.`);
    }
  }
  return draft;
}

export function formatNotes(notes) {
  return notes
    .replace(/ by @domsleee(?= in https:\/\/github\.com\/[^/\s]+\/[^/\s]+\/pull\/\d+\r?$)/gm, '')
    .replace(/ in (https:\/\/github\.com\/[^/\s]+\/[^/\s]+\/pull\/(\d+))\r?$/gm, ' ([#$2]($1))')
    .replace(/^\*\*Full Changelog\*\*: (https:\/\/github\.com\/\S+)\r?$/gm, '[Full changelog]($1)')
    .trim()
    .replace(/(\r?\n){3,}/g, '\n\n');
}

function newPullRequests(previousTarget, target, repository) {
  // Older drafts may name a moving branch, so fall back to deduplicating their text.
  if (!/^[0-9a-f]{40}$/i.test(previousTarget)) return null;

  const numbers = new Set();
  const commits = gh([
    'api', '--method', 'GET', '--paginate', `repos/${repository}/compare/${previousTarget}...${target}`,
    '-f', 'per_page=100', '--jq', '.commits[].sha',
  ]).split(/\r?\n/).filter(Boolean);
  for (const commit of commits) {
    const prs = gh([
      'api', '--paginate', `repos/${repository}/commits/${commit}/pulls`,
      '--jq', '.[] | select(.merged_at != null) | .number',
    ]).split(/\r?\n/).filter(Boolean);
    for (const number of prs) numbers.add(number);
  }
  return numbers;
}

export function mergeNotes(body, generated, repository, newPrs) {
  if (!body?.trim()) return generated;

  const prUrl = `https://github.com/${repository}/pull/`;
  const prPattern = prUrl.replace(/[.*+?^${}()|[\]\\]/g, '\\$&') + '(\\d+)\\b';
  const references = new RegExp(`${prPattern}|(?<!\\w)#(\\d+)\\b`, 'g');
  const seen = new Set([...body.matchAll(references)].map(match => match[1] || match[2]));
  const bullet = new RegExp(`^\\* .*${prPattern}`);
  const additions = [];
  for (const line of generated.split(/\r?\n/)) {
    const number = line.match(bullet)?.[1];
    // Keep previously covered changes omitted or rewritten by the maintainer.
    if (!number || seen.has(number) || (newPrs && !newPrs.has(number))) continue;
    seen.add(number);
    additions.push(line);
  }

  const footerPattern = /\[Full changelog\]\([^)]+\)|\*\*Full Changelog\*\*: \S+/i;
  if (additions.length) {
    const footerLine = body.match(new RegExp(`^[^\\r\\n]*(?:${footerPattern.source})[^\\r\\n]*`, 'im'));
    const index = footerLine?.index ?? body.length;
    body = [body.slice(0, index).trimEnd(), additions.join('\n'), body.slice(index)]
      .filter(Boolean).join('\n\n');
  }
  const footer = generated.match(footerPattern)?.[0];
  if (footer) {
    body = footerPattern.test(body)
      ? body.replace(new RegExp(footerPattern, 'gi'), () => footer)
      : `${body.trimEnd()}\n\n${footer}`;
  }
  return body.trimEnd();
}

export function runRelease(action, version, zip) {
  if (!['check', 'update'].includes(action) || !version || (action === 'update' && !zip)) {
    throw new Error('Usage: bun Display_Config/scripts/release.mjs check <version> | update <version> <zip>');
  }
  const { GITHUB_REPOSITORY: repository, GITHUB_SHA: target } = process.env;
  if (!repository || !target) throw new Error('GITHUB_REPOSITORY and GITHUB_SHA must be set.');

  const draft = getDraft(version, repository, target);
  if (action === 'check') return;

  const generated = formatNotes(gh([
    'api', '--method', 'POST', `repos/${repository}/releases/generate-notes`,
    '-f', `tag_name=${version}`, '-f', `target_commitish=${target}`, '--jq', '.body',
  ]));
  const newPrs = draft?.body?.trim() ? newPullRequests(draft.targetCommitish, target, repository) : null;
  const notes = mergeNotes(draft?.body, generated, repository, newPrs);
  writeFileSync('release-notes.md', `${notes}\n`);

  if (draft) {
    gh(['release', 'edit', version, '--repo', repository, '--target', target, '--notes-file', 'release-notes.md']);
    gh(['release', 'upload', version, zip, '--repo', repository, '--clobber']);
  } else {
    gh([
      'release', 'create', version, zip, '--repo', repository, '--draft',
      '--title', `Display_Config ${version}`, '--target', target, '--notes-file', 'release-notes.md',
    ]);
  }
  if (process.env.GITHUB_STEP_SUMMARY) appendFileSync(process.env.GITHUB_STEP_SUMMARY, `${notes}\n`);
}

if (import.meta.main) {
  try {
    runRelease(...process.argv.slice(2));
  } catch (error) {
    console.error(error.message);
    process.exitCode = 1;
  }
}
