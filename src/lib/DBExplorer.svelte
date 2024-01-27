<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import Box from './Box.svelte';
	import { queryStore } from './store';

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

<div>
	<input class="input" type="search" bind:value={search} on:input={onSearch} placeholder="Search..." />
	<Box>
		{#each dbObjectsFiltered as dbObj}
			<div on:click={() => select100(dbObj)}>
				{dbObj}
			</div>
		{/each}
	</Box>
</div>
