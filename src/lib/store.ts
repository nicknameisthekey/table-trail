import { writable } from 'svelte/store';

export const queryStore = writable('');

export const resultStore = writable<Record<string, string>[]>([]);