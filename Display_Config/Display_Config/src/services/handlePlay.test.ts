
import { test, expect, describe } from 'bun:test';
import { getTrainerSettingsFromUI, requiresInject } from './handlePlay';
import { getDefaultTrainerUISettings } from '../stores/trainerSettings';

describe('handlePlay', () => {
  test('requiresInject', () => {
    const trainerSettings = getTrainerSettingsFromUI(getDefaultTrainerUISettings().trainerSettings.value);
    trainerSettings.changeFov = false;
    // The render-distance fix (extendRenderDistance) ships default-ON, so
    // out-of-the-box settings DO inject; with it off, nothing else should.
    expect(requiresInject(trainerSettings), 'extendRenderDistance defaults on and requires inject.').toBe(true);
    trainerSettings.extendRenderDistance = false;
    expect(requiresInject(trainerSettings), 'configCrashGuard also defaults on and requires inject.').toBe(true);
    trainerSettings.configCrashGuard = false;
    expect(requiresInject(trainerSettings), 'with every option off there is nothing to inject.').toBe(false);

    const boolKeys = (Object.keys(trainerSettings) as (keyof typeof trainerSettings)[])
      .filter(key => trainerSettings[key] === false || trainerSettings[key] === true);
    const original = Object.freeze(trainerSettings);
    for (const key of boolKeys) {
      const newSettings = { ...original, [key]: true };
      expect(requiresInject(newSettings), `'${key}' should make requiresInject true`).toBe(true);
    }
  });
})