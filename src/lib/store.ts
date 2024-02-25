import { writable } from 'svelte/store';

export const queryStore = writable('');
export const chosenProfile = writable(0);

export const resultStore = writable<Record<string, string>[]>([]);