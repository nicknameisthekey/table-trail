<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import { queryStore } from './store';
	import { Listgroup, ListgroupItem } from 'flowbite-svelte';
	import { Input, Label, Button } from 'flowbite-svelte';
	import { SearchOutline } from 'flowbite-svelte-icons';

	let dbObjects: string[] = [];
	let dbObjectsFiltered: string[] = [];
	let search = '';

	onMount(async () => {
		await get_tables();
	});

	async function get_tables() {
		dbObjects = await invoke('tables');
		dbObjectsFiltered = dbObjects;
	}

	function onSearch(event: any) {
		dbObjectsFiltered = dbObjects.filter((t) => t.includes(search));
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
			<SearchOutline slot="right" class="h-6 w-6 text-gray-500 dark:text-gray-400 mx-2" />
		</Input>
	</div>

	<div class="h-[90%] overflow-auto">
		<Listgroup active>
			{#each dbObjectsFiltered as dbObj}
			<ListgroupItem on:click={() => select100(dbObj)} class="gap-2 text-base font-semibold">
				{dbObj}
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
