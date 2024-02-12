<script lang="ts">
	import '../app.pcss';
	import { AppShell } from '@skeletonlabs/skeleton';
	import DBExplorer from '$lib/DBExplorer.svelte';
	import QueryEditor from '$lib/QueryEditor.svelte';
	import QueryResult from '$lib/QueryResult.svelte';
	import { initializeStores } from '@skeletonlabs/skeleton';
	import AddDbConnection from '$lib/AddDBConnection.svelte';

	initializeStores();
</script>

<AddDbConnection/>
<AppShell>
	<svelte:fragment slot="sidebarLeft"><DBExplorer /></svelte:fragment>
	<svelte:fragment slot="sidebarRight"><QueryResult /></svelte:fragment>
	<slot />
	<QueryEditor />
	<svelte:fragment slot="pageFooter">Page Footer</svelte:fragment>
</AppShell>
