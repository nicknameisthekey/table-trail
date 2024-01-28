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

<div style="width:15vw;margin:5px">
	<input
		class="input"
		type="search"
		placeholder="Search..."
		style="width:12vw;height:30px;margin-left:5px;"
		bind:value={search}
		on:input={onSearch}
	/>
	<div class="card p-4" style="height:95vh; overflow:scroll; margin:5px">
		<ul class="list">
			{#each dbObjectsFiltered as dbObj}
				<li>
					<span
						style="border-bottom: 1px dashed rgba(169, 169, 169, 0.2);"
						on:click={() => select100(dbObj)}
						class="flex-auto">{dbObj}</span
					>
				</li>
			{/each}
		</ul>
	</div>
</div>
