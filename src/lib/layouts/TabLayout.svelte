<script lang="ts">
	import TabButton from '$lib/buttons/TabButton.svelte';

	export let tabs: string[] = [];
	let currentTab: number = 0;

	$: console.log(currentTab);

	function setTab(tabName: number): () => void {
		return () => (currentTab = tabName);
	}
</script>

<div class="flex flex-col">
	<div class="flex row-auto ">
		{#each tabs as tab, i}
			<TabButton on:click={setTab(i)}>{tab}</TabButton>
		{/each}
	</div>

	<div>
		{#if currentTab == 0}
			<slot name="1" />
		{:else if currentTab == 1}
			<slot name="2" />
		{:else if currentTab == 2}
			<slot name="3" />
		{/if}
	</div>
</div>
