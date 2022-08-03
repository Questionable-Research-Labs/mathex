<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import Pane from '$lib/layouts/Pane.svelte';

	const countdown = new Date();
	countdown.setMinutes(countdown.getMinutes() + 40);
	let difference: Date | undefined;
	let interval: ReturnType<typeof setInterval> | undefined;

	onMount(() => {
		interval = setInterval(() => {
			difference = new Date(countdown.getTime() - Date.now());
		}, 1000);
	});

	onDestroy(() => {
		if (interval) {
			clearInterval(interval);
		}
	});
</script>

<Pane>
	{#if difference}
		<h1 class="text-gray-300 text-9xl font-bold">
			{difference.getMinutes()}:{('' + difference.getSeconds()).padStart(2, '0')}
		</h1>
	{/if}
</Pane>
