<script lang="ts">
	import TabButton from '$lib/elements/TabButton.svelte';

	export let tabs: string[] = [];
	let currentTab = 0;

	$: console.log(currentTab);

	function setTab(tabName: number): () => void {
		return () => (currentTab = tabName);
	}
</script>

<div class="flex flex-col bg-blue-900 rounded-xl">
	<div class="flex row-auto justify-between bg-blue-700 rounded-t-xl">
		{#each tabs as tab, i}
			<TabButton on:click={setTab(i)} selected={i == currentTab}>{tab}</TabButton>
		{/each}
	</div>

	<div class="p-10">
		{#if currentTab == 0}
			<slot name="1" />
		{:else if currentTab == 1}
			<slot name="2" />
		{:else if currentTab == 2}
			<slot name="3" />
		{/if}
	</div>
</div>
