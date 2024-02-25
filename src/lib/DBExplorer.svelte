<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import { queryStore, chosenProfile } from './store';
	import { Listgroup, ListgroupItem } from 'flowbite-svelte';
	import { Input, Label, Button } from 'flowbite-svelte';
	import { SearchOutline } from 'flowbite-svelte-icons';

	type DbObjects = {
		tables: Table[];
	};

	type Table = {
		name: String;
		schema: String;
	};

	let dbObjects: DbObjects = { tables: [] };
	let dbObjectsFiltered: DbObjects = { tables: [] };
	let search = '';

	onMount(async () => {
		chosenProfile.subscribe((profileId) => {
			if (profileId != 0) getDbObjects(profileId);
		});
	});

	async function getDbObjects(profileId: number) {
		dbObjects = await invoke('db_objects', { profile_id: profileId });
		dbObjectsFiltered = dbObjects;
		console.log('fetched dbObjects: ', dbObjects);
	}

	function onSearch(event: any) {
		let tables = dbObjects.tables.filter((t) => t.name.includes(search));
		dbObjectsFiltered = { tables };
	}

	function select100(obj: any) {
		let query = 'select * from ' + obj;
		console.log('send ' + query);
		queryStore.set(query);
	}
</script>

<div class="db-explorer h-screen">
	<div class="db-explorer-search">
		<Input on:input={onSearch} bind:value={search} id="search" placeholder="Search" size="lg">
			<SearchOutline slot="right" class="mx-2 h-6 w-6 text-gray-500 dark:text-gray-400" />
		</Input>
	</div>

	<div class="h-[90%] overflow-auto">
		<Listgroup active>
			{#each dbObjectsFiltered.tables as table}
				<ListgroupItem on:click={() => select100(table.name)} class="gap-2 text-base font-semibold">
					{table.name}
				</ListgroupItem>
			{/each}
		</Listgroup>
	</div>
</div>

<style>
	.db-explorer {
		width: 300px;
		min-width: 200px;
		margin-right: 10px;
		overflow: scroll;
	}

	.db-explorer-search {
		margin-bottom: 10px;
	}
</style>
