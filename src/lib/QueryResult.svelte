<script lang="ts">
	import { resultStore } from './store';
	import { onMount } from 'svelte';
	import {
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow,
		TableHead,
		TableHeadCell,
		Checkbox,
		TableSearch
	} from 'flowbite-svelte';

	let data: Record<string, string>[] = [];
	let headers: string[] = [];

	onMount(async () => {
		resultStore.subscribe((v) => OnResult(v));
	});

	const getUniqueKeys = (data: Record<string, string>[]): string[] => {
		const keysSet = new Set<string>();
		data.forEach((item) => {
			Object.keys(item).forEach((key) => keysSet.add(key));
		});
		return Array.from(keysSet);
	};

	let flattenedData: string[][] = [];

	function OnResult(newData: Record<string, string>[]) {
		data = newData;
		headers = getUniqueKeys(data);
		flattenedData = data.map((item) => Object.values(item));

		console.log(headers);
		console.log(data);
	}
</script>

<div>
	<Table>
		<TableHead>
			{#each headers as header}
			<TableHeadCell>{header}</TableHeadCell>
			{/each}
		</TableHead>
		<TableBody tableBodyClass="divide-y">
			{#each flattenedData as row}
			<TableBodyRow>
				{#each row as cell}
				<TableBodyCell>{cell}</TableBodyCell>
				{/each}
			</TableBodyRow>
			{/each}
		</TableBody>
	</Table>
</div>
