<script lang="ts">
	import TabButton from '$lib/buttons/TabButton.svelte';
	import Pane from '$lib/layouts/Pane.svelte';
	import TabLayout from '$lib/layouts/TabLayout.svelte';
</script>

<TabLayout tabs={['Teams', 'Questions', 'Rounds']}>
	<svelte:fragment slot="1">Teams display</svelte:fragment>
	<svelte:fragment slot="2">Questions display</svelte:fragment>
	<svelte:fragment slot="3">Rounds display</svelte:fragment>
</TabLayout>
