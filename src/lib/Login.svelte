<script lang="ts">
	import Pane from './layouts/Pane.svelte';
	import { password } from './stores';

	let passwordInput = '';
	let error = false;

	async function tryLogin() {
		let res = await fetch('/api/pwcheck', {
			headers: {
				Authorization: passwordInput
			}
		});

		if (res.status === 200) {
			$password = passwordInput;
			error = false;
		} else {
			error = true;
		}
	}
</script>

<Pane>
	<form on:submit|preventDefault={tryLogin} class="flex flex-col justify-center items-center gap-2">
		<div class="bg-white p-5 rounded-xl drop-shadow-xl">
			<label for="password">Password</label>
			<input
				type="password"
				id="password"
				class="rounded-xl border-gray-400 border-2 bg-gray-200 px-3"
				bind:value={passwordInput}
			/>
		</div>
		<input type="submit" value="Login" class="bg-gray-200 p-3 rounded-xl drop-shadow-xl" />
		{#if error}
			<span class="text-red-500 text-center">Invalid Password</span>
		{/if}
	</form>
</Pane>
