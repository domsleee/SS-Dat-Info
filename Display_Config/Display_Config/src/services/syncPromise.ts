import { type Ref, ref } from "vue";

export interface SyncPromise<T> {
    value?: T;
    isLoading: boolean;
}

export function getSyncPromise<T>(getValue: Promise<T>): SyncPromise<T> {
    let value: Ref<T | undefined> = ref(undefined);
    let isLoading = true;
    getValue.then((v) => {
        value.value = v;
        isLoading = false;
    });

    return {
        get value() {
            return value.value;
        },
        get isLoading() {
            return isLoading;
        },
    };
}