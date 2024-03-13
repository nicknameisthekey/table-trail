<script lang="ts">
	import '../app.pcss';
	import QueryEditor from '$lib/QueryEditor.svelte';
	import QueryResults from '$lib/QueryResults.svelte';
	import DBProfiles from '$lib/DBProfiles.svelte';
	import DbExplorer from '$lib/DBExplorer.svelte';
	import QueryTabs from '$lib/QueryTabs.svelte';
</script>

<div class="flex h-full w-full flex-col items-start bg-primary-100 font-sans">
	<div class="grid w-full grid-cols-[auto,1fr]">
		<div>
			<DBProfiles />
		</div>
		<div>
			<div class="inline-block w-full">
				<QueryTabs />
			</div>
			<div class="flex flex-row">
				<DbExplorer />
				<QueryEditor />
				<QueryResults />
			</div>
		</div>
	</div>
</div>
