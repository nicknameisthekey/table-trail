<script>
	import QueryEditor from '$lib/QueryEditor.svelte';
	import QueryResult from '$lib/QueryResult.svelte';
	import DBExplorer from '../lib/DBExplorer.svelte';
	import DbConnections from '$lib/DBConnections.svelte';

	let showDbConnections = false;
</script>

<div class="container" style="max-height: 95vh;">
	<button style="width: 20px; height:20px;" on:click={() => (showDbConnections = true)}>+</button>

	<DbConnections bind:showDbConnections></DbConnections>
	<DBExplorer />
	<QueryEditor />
	<QueryResult />
</div>

<style>
	.container {
		display: flex;
		gap: 10px;
	}
</style>
