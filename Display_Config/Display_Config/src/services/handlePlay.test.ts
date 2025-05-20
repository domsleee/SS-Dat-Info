
import { test, expect, describe } from 'bun:test';
import { getTrainerSettingsFromUI, requiresInject } from './handlePlay';
import { getDefaultTrainerUISettings } from '../stores/trainerSettings';

describe('handlePlay', () => {
  test('requiresInject', () => {
    const trainerSettings = getTrainerSettingsFromUI(getDefaultTrainerUISettings().trainerSettings.value);
    trainerSettings.changeFov = false;
    expect(requiresInject(trainerSettings)).toBe(false);

    const boolKeys = (Object.keys(trainerSettings) as (keyof typeof trainerSettings)[])
      .filter(key => trainerSettings[key] === false || trainerSettings[key] === true);
    const original = Object.freeze(trainerSettings);
    for (const key of boolKeys) {
      const newSettings = { ...original, [key]: true };
      expect(requiresInject(newSettings), `'${key}' should make requiresInject true`).toBe(true);
    }
  });
})