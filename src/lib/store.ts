import { writable } from 'svelte/store';

export const queryStore = writable('');
export const chosenProfile = writable(0);

type QueryResult = { columns: string[]; rows: string[][]; intial: Boolean };

export const resultStore = writable<QueryResult>({ columns: [], rows: [], intial: true });
