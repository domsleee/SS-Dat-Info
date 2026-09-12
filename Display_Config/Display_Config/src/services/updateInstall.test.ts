import { afterAll, afterEach, beforeAll, beforeEach, expect, spyOn, test } from 'bun:test';
import { clearMocks, mockIPC } from '@tauri-apps/api/mocks';
import { createPinia, setActivePinia } from 'pinia';
import { commands } from '@/bindings';
import { useUpdateDialogStore } from '@/stores/updateDialogStore';
import { useErrorStore } from '@/stores/errorStore';
import { update } from './updaterService';

const originalWindow = Object.getOwnPropertyDescriptor(globalThis, 'window');
const download = spyOn(commands, 'downloadAndExtract');
const relaunch = spyOn(commands, 'relaunch');
const cancel = spyOn(commands, 'cancelDownload');

beforeAll(() => {
  Object.defineProperty(globalThis, 'window', { value: { crypto: globalThis.crypto }, configurable: true });
});
beforeEach(() => {
  setActivePinia(createPinia());
  mockIPC(() => undefined);
  relaunch.mockResolvedValue(undefined);
  cancel.mockResolvedValue(true);
});
afterEach(() => {
  clearMocks();
  download.mockReset();
  relaunch.mockReset();
  cancel.mockReset();
});
afterAll(() => {
  download.mockRestore();
  relaunch.mockRestore();
  cancel.mockRestore();
  if (originalWindow) Object.defineProperty(globalThis, 'window', originalWindow);
  else Reflect.deleteProperty(globalThis, 'window');
});

test('installation phase stays visible and successful update relaunches', async () => {
  download.mockImplementation(async (_url, channel) => {
    channel.onmessage({ event: 'installing' });
    expect(useUpdateDialogStore().state.key).toBe('installing');
    return { installed: true };
  });
  await update('0.4.6');
  expect(useUpdateDialogStore().state.key).toBe('finished');
  expect(relaunch).toHaveBeenCalledTimes(1);
});

test('failed installation closes the progress dialog and reports the error', async () => {
  download.mockImplementation(async (_url, channel) => {
    channel.onmessage({ event: 'installing' });
    throw 'Update failed; previous files were restored';
  });
  await update('0.4.6');
  expect(useUpdateDialogStore().state.key).toBe('closed');
  expect(useErrorStore().error).toBe('Update failed; previous files were restored');
  expect(relaunch).not.toHaveBeenCalled();
});

test('cancellation requested before the token arrives is forwarded and does not relaunch', async () => {
  download.mockImplementation(async (_url, channel) => {
    const state = useUpdateDialogStore().state;
    if (state.key !== 'downloading') throw new Error('Expected download');
    state.cancelling = true;
    channel.onmessage({ event: 'token', data: { token: 'request-id' } });
    expect(cancel).toHaveBeenCalledWith('request-id');
    return { installed: false };
  });
  await update('0.4.6');
  expect(useUpdateDialogStore().state.key).toBe('closed');
  expect(relaunch).not.toHaveBeenCalled();
});
