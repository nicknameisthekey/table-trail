<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import { queryStore, chosenProfile } from './store';
	import { Listgroup, ListgroupItem } from 'flowbite-svelte';
	import { Input, Label, Button } from 'flowbite-svelte';
	import { TableRowOutline } from 'flowbite-svelte-icons';

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

	let contextMenuVisible = false;
	let contextMenuX = 0;
	let contextMenuY = 0;

	function handleRightClick(event: any) {
		console.log('right click');
		event.preventDefault(); // prevent the browser's default context menu from appearing

		contextMenuVisible = true;
		contextMenuX = event.clientX;
		contextMenuY = event.clientY;
	}

	function hideContextMenu() {
		contextMenuVisible = false;
	}
</script>

<div class="h-screen w-64 overflow-scroll">
	<div class="my-2 flex">
		<input
			class="border-primary-500 mx-auto h-8 w-56 rounded-xl border"
			type="text"
			on:input={onSearch}
			bind:value={search}
			placeholder="Search"
		/>
	</div>

	<ul class="border-primary-500 bg-primary-100 h-[90%] list-inside overflow-auto">
		{#each dbObjectsFiltered.tables as table}
			<li class="hover:bg-primary-300 flex justify-start pl-3">
				<button
					class="text-primary-500 flex h-6 w-full items-center"
					on:contextmenu={handleRightClick}
					on:click={() => select100(table.name)}
				>
					<TableRowOutline class="mr-2 size-4 text-center" />
					{table.name}
				</button>
			</li>
		{/each}
	</ul>
</div>

{#if contextMenuVisible}
	<div
		class="absolute border bg-white"
		style="top: {contextMenuY}px; left: {contextMenuX}px;"
		on:click={hideContextMenu}
	>
		<p>Context menu item 1</p>
		<p>Context menu item 2</p>
		<!-- add more items as needed -->
	</div>
{/if}
