import { afterAll, afterEach, expect, spyOn, test } from 'bun:test';
import { commands } from '@/bindings';
import { checkForUpdates } from './updaterService';

const check = spyOn(commands, 'checkForUpdates');
afterEach(() => check.mockReset());
afterAll(() => check.mockRestore());

test('automatic checks use the cache and manual checks force a refresh', async () => {
  check.mockResolvedValue({ currentVersion: '0.4.5', latestVersion: '0.4.6' });
  await checkForUpdates();
  expect(check).toHaveBeenLastCalledWith(false);
  await checkForUpdates(true);
  expect(check).toHaveBeenLastCalledWith(true);
});

test('manual check failures reach the caller', async () => {
  check.mockRejectedValue('Could not check for updates: offline');
  await expect(checkForUpdates(true)).rejects.toBe('Could not check for updates: offline');
});
