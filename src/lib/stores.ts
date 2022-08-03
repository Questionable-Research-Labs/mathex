import { derived, writable, type Writable } from 'svelte/store';

const skipLogin = true;

export const password: Writable<string | null> = writable(
	import.meta.env.PROD ? null : skipLogin ? import.meta.env.VITE_PASSWORD : null
);

export const signedIn = derived(password, (val) => val !== null);
