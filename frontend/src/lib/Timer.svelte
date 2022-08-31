<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	let mins = '40';
	let secs = '00';
	let initMins = mins;
	let initSecs = secs;
	let startTime: number | null = null;
	let timeLimit = 0;
	let interval: number;
	let expanded = false;

	function start() {
		initMins = mins;
		initSecs = secs;
		expanded = false;
		timeLimit = parseInt(mins) * 60000 + 1000 * parseInt(secs);
		startTime = Date.now();
		interval = setInterval(() => {
			if (startTime) {
				const delta = Date.now() - startTime;

				if (delta >= timeLimit - 100) stop();

				const temp = Math.floor((timeLimit - delta) / 1000);

				if (!expanded && temp < 300) {
					expanded = true;
				}

				mins = '' + Math.floor(temp / 60);
				secs = '' + (temp % 60);

				if (mins.length < 2) {
					mins = '0' + mins;
				}

				if (secs.length < 2) {
					secs = '0' + secs;
				}
			}
		}, 500) as unknown as number;
	}

	function stop() {
		expanded = false;
		clearInterval(interval);
		startTime = null;
		startTime = null;
	}

	function expand() {
		expanded = !expanded;
	}

	function reset() {
		mins = initMins;
		secs = initSecs;
	}

	$: if (expanded) dispatch('expand');
	else dispatch('collapse');
</script>

<div class="outer" class:expanded>
	<button class="max" on:click={expand}>
		<span class="material-icons">fullscreen</span>
	</button>

	<div class="main-display">
		{#if startTime}
			<h1>{mins}:{secs}</h1>
			<button class="stop" on:click={stop}>Stop</button>
		{:else}
			<div class="inputs">
				<input bind:value={mins} type="number" />:<input bind:value={secs} type="number" />
			</div>
			<button class="stop" on:click={reset}>Reset</button>
			<button class="start" on:click={start}>Start</button>
		{/if}
	</div>
</div>

<style>
	.outer {
		position: relative;

		transition: all 500ms ease-in-out;
		width: 20%;
	}

	.expanded {
		width: 100%;
	}

	button {
		border: none;
		background-color: transparent;
		position: relative;
	}

	.max {
		position: absolute;
		left: 10px;
		top: 10px;
	}

	.main-display {
		display: flex;
		align-items: center;
		justify-content: center;
		flex-direction: column;
		width: 100%;
		height: 100%;
		gap: 1rem;
	}

	.start {
		background-color: rgb(60, 60, 236);
		font-size: 2rem;
		color: #ddd;
		padding: 1rem;
		border-radius: 10px;
	}

	.stop {
		background-color: rgb(101, 101, 228);
		font-size: 1rem;
		color: #eee;
		padding: 0.75rem;
		border-radius: 10px;
	}

	.inputs {
		display: flex;
		font-size: 5rem;
	}

	.inputs input {
		border: none;
		width: 7rem;
		font-size: 5rem;
	}

	h1 {
		font-size: 6rem;
	}
</style>
