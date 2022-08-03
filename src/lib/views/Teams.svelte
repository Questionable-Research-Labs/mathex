<script lang="ts">
	import SectionTitle from '$lib/elements/SectionTitle.svelte';
	import Input from '$lib/elements/Input.svelte';
	import type { Team } from '@prisma/client';
	import SubmitButton from '$lib/elements/SubmitButton.svelte';

	let teamName = '';
	let teamLevel = '';

	$: {
		teamLevel = [...teamLevel].filter((x) => Number(x)).join('');
	}

	const teams: Team[] = [];

	function addTeam() {}
</script>

<SectionTitle>Teams</SectionTitle>
<table class="w-[50vw] max-h-20 overflow-y-scroll">
	<colgroup>
		<col span="1" class="w-1/6" />
		<col span="1" class="w-3/6 overflow-x-clip" />
		<col span="1" class="w-1/6" />
		<col span="1" class="w-1/6" />
	</colgroup>
	<thead>
		<tr>
			<th>ID</th>
			<th>Name</th>
			<th>Year</th>
			<th />
		</tr>
	</thead>

	<tbody>
		{#each teams as team}
			<tr>
				<td>{team.id}</td>
				<td>{team.name}</td>
				<td>{team.level}</td>
				<td><a href={`/admin/team/${team.id}`}>Edit team</a></td>
			</tr>
		{/each}
	</tbody>
</table>

<div class="h-5" />

<SectionTitle>Add team</SectionTitle>
<form on:submit|preventDefault={addTeam} class="flex justify-around flex-row items-center">
	<Input bind:value={teamName} id="team-name" bg={false}>Team name</Input>
	<Input bind:value={teamLevel} id="team-level" bg={false}>Team level</Input>
	<SubmitButton value="Add team" />
</form>
