<script lang="ts">
	import Login from '$lib/views/Login.svelte';
	import { signedIn } from '$lib/stores';
</script>

{#if $signedIn}
	<slot />
{:else}
	<Login />
{/if}
