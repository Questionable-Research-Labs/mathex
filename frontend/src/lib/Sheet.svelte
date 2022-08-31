<script lang="ts">
	import { PUBLIC_SHEET_ID } from '$env/static/public';

	export let hidden = false;
</script>

<iframe
	src={`https://docs.google.com/spreadsheets/d/${PUBLIC_SHEET_ID}/edit`}
	title="Sheets"
	frameborder="0"
	class:hidden
/>

<style>
	iframe {
		width: 80%;
		height: 100%;
		opacity: 100%;
	}

	.hidden {
		width: 0%;
		opacity: 0%;
		transition: all 500ms ease-in-out;
	}
</style>
