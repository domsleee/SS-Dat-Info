import { file, TOML } from 'bun';
import { expect, test } from 'bun:test';
import { name, version } from '../package.json';
import tauri from '../src-tauri/tauri.conf.json';

test('package.json, Tauri and Cargo versions match', async () => {
  const cargo = TOML.parse(await file(new URL('../src-tauri/Cargo.toml', import.meta.url)).text()) as {
    package: { version: string };
  };
  const lock = TOML.parse(await file(new URL('../src-tauri/Cargo.lock', import.meta.url)).text()) as {
    package: { name: string; version: string }[];
  };

  expect({
    'tauri.conf.json': tauri.version,
    'Cargo.toml': cargo.package.version,
    'Cargo.lock': lock.package.find(entry => entry.name === name)?.version,
  }).toEqual({
    'tauri.conf.json': version,
    'Cargo.toml': version,
    'Cargo.lock': version,
  });
});
