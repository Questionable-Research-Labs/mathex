import { derived, writable, type Writable } from 'svelte/store';

export const password: Writable<string | null> = writable(
	import.meta.env.PROD ? null : import.meta.env.VITE_PUBLIC_PASSWORD
);
export const signedIn = derived(password, (val) => val !== null);
