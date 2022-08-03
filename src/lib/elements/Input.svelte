<script lang="ts">
	export let value = '';
	export let id = '';
	export let type = 'text';
	export let bg = true;
	function typeInput(node: HTMLInputElement) {
		node.type = type;
	}
</script>

{#if $$slots.default && id}
	<div class="rounded-xl drop-shadow-xl" class:bg-white={bg} class:p-5={bg}>
		<label for={id} class:text-gray-100={!bg} class:font-bold={!bg}><slot /></label>
		<input
			use:typeInput
			{id}
			class="rounded-xl border-gray-400 border-2 bg-gray-200 px-3"
			bind:value
		/>
	</div>
{:else}
	<input
		use:typeInput
		{id}
		class="rounded-xl border-gray-400 border-2 bg-gray-200 px-3 drop-shadow-xl"
		bind:value
	/>
{/if}
