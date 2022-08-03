<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	export let dispatch = createEventDispatcher();
	export let selected = false;

	function clickHandler(event: MouseEvent) {
		dispatch('click', event);
	}
</script>

<button
	class="rounded-t-xl bg-gray-200 px-3 text-gray-900 text-2xl font-bold"
	class:bg-gray-400={selected}
	on:click={clickHandler}
>
	<slot />
</button>
