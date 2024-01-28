<script lang="ts">
	import { Table, tableMapperValues } from '@skeletonlabs/skeleton';
	import type { TableSource } from '@skeletonlabs/skeleton';
	import { resultStore } from './store';
	import { onMount } from 'svelte';

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

	const emptyTable = {
		head: [],
		body: []
	};
	let tableSimple: TableSource = emptyTable;

	function setTableSource(): TableSource {
		return {
			head: headers,
			body: tableMapperValues(data, headers)
		};
	}

	$: tableSimple = data ? setTableSource() : emptyTable;

	function OnResult(newData: Record<string, string>[]) {
		data = newData;
		headers = getUniqueKeys(data);
		console.log(headers);
		console.log(data);
	}
</script>

{#if data.length > 0}
	<Table source={tableSimple} />
{/if}
