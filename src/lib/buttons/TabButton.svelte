<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	export let dispatch = createEventDispatcher();

	function clickHandler(event: MouseEvent) {
		dispatch('click', event);
	}
</script>

<button class="rounded-t-lg" on:click={clickHandler}><slot /></button>
