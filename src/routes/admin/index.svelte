<script lang="ts">
	import Questions from '$lib/views/Questions.svelte';
	import Rounds from '$lib/views/Rounds.svelte';
	import Teams from '$lib/views/Teams.svelte';
	import TabLayout from '$lib/layouts/TabLayout.svelte';
</script>

<TabLayout tabs={['Teams', 'Questions', 'Rounds']}>
	<svelte:fragment slot="1"><Teams /></svelte:fragment>
	<svelte:fragment slot="2"><Questions /></svelte:fragment>
	<svelte:fragment slot="3"><Rounds /></svelte:fragment>
</TabLayout>
