<script lang="ts">
	import Box from './Box.svelte';
	import { resultStore } from './store';
	import { onMount } from 'svelte';

	let data: Record<string, string>[] = [];

	const getUniqueKeys = (data: Record<string, string>[]): string[] => {
		const keysSet = new Set<string>();
		data.forEach((item) => {
			Object.keys(item).forEach((key) => keysSet.add(key));
		});
		return Array.from(keysSet);
	};

	let keys: string[] = getUniqueKeys(data);

	onMount(async () => {
		resultStore.subscribe((v) => OnResult(v));
	});

	function OnResult(newData: Record<string, string>[]) {
		console.log('got new data');
		data = newData;
		keys = getUniqueKeys(data);
	}
</script>

{#if keys.length>0}
<Box>
	<table style="width: 200px; height:200px;">
		<thead>
			<tr>
				{#each keys as key}
					<th>{key}</th>
				{/each}
			</tr>
		</thead>
		<tbody>
			{#each data as row}
				<tr>
					{#each keys as key}
						<td>{row[key]}</td>
					{/each}
				</tr>
			{/each}
		</tbody>
	</table>
</Box>
{/if}

<style>
	th,
	td {
		border: 1px solid #dddddd;
		text-align: left;
		padding: 8px;
	}

	th {
		background-color: #f2f2f2;
	}
</style>
