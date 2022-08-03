<script lang="ts">
	import Pane from '../layouts/Pane.svelte';
	import { password } from '../stores';
	import Input from '../elements/Input.svelte';
	import SubmitButton from '$lib/elements/SubmitButton.svelte';

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
		<Input bind:value={passwordInput} type="password" id="password">Password</Input>
		<SubmitButton value="Login" />
		{#if error}
			<span class="text-red-500 text-center">Invalid Password</span>
		{/if}
	</form>
</Pane>
