<script lang="ts">
	import '../app.pcss';
	import DBExplorer from '$lib/DBExplorer.svelte';
	import QueryEditor from '$lib/QueryEditor.svelte';
	import QueryResult from '$lib/QueryResult.svelte';
	import DbConnections from '$lib/DBConnections.svelte';
	import DbExplorer from '$lib/DBExplorer.svelte';
</script>

<div class="main-window bg-gray-500">
	<DbConnections />
	<DbExplorer />
	<QueryEditor />
	<QueryResult />
</div>

<style>
	.main-window {
		display: flex;
		align-items: flex-start;
		flex-direction: row;
		padding-top: 6px;
	}
</style>
