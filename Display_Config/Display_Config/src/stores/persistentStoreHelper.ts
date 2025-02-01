import { type Serializer } from "pinia-plugin-persistedstate";
import destr from "destr";
import type { Ref } from "vue";

export function getPersistentSettings<U>(getDefaultStoreValue: () => {[k in string]: Ref<U>}): { persist: { serializer: Serializer } } {
  return {
      persist: {
      serializer: {
        serialize: (value:  {[k in string]: Ref<U>}) => JSON.stringify(value),
        deserialize: (value: string) => {
          const result: Record<string, unknown> = {};
          const incomingValue = destr(value) as {[k in string]: Partial<U>};
          const defaultValue = getDefaultStoreValue();
          for (const [k, defaultValueEntry] of Object.entries(defaultValue)) {
            result[k] = { ...defaultValueEntry.value, ...incomingValue[k] };
          }
          return result;
        },
      },
    }
  };
}