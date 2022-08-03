<script lang="ts">
	import Login from '$lib/Login.svelte';
	import { signedIn } from '$lib/stores';
</script>

{#if $signedIn}
	<slot />
{:else}
	<Login />
{/if}
