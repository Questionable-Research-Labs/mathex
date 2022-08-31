<script lang="ts">
	import Sheet from '$lib/Sheet.svelte';
	import Timer from '$lib/Timer.svelte';

	let sheetHidden = false;
</script>

<svelte:head>
	<title>MATHX 2022</title>
</svelte:head>

<div>
	<Sheet hidden={sheetHidden} />
	<Timer
		on:expand={() => {
			sheetHidden = true;
		}}
		on:collapse={() => {
			sheetHidden = false;
		}}
	/>
</div>

<style>
	:global(*) {
		margin: 0;
		box-sizing: border-box;
		font-weight: 700;
	}

	:global(body) {
		font-family: 'Open Sans', sans-serif;
	}

	div {
		display: flex;
		width: 100vw;
		max-width: 100%;
		height: 100vh;
		max-height: 100%;
	}
</style>
