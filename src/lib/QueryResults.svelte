<script lang="ts">
	import { resultStore } from './store';
	import { onMount } from 'svelte';
	import QueryResult from '$lib/QueryResult.svelte';
	import type { QueryResultData } from '$lib/models';

	let queriyResults: QueryResultData[] = [];

	onMount(async () => {
		resultStore.subscribe((v) => OnResult(v));
	});

	function OnResult(newData: QueryResultData) {
		if (newData.intial) return;

		console.log('Получен результат', newData);
		queriyResults = [...queriyResults, newData];
	}

	function onDelete(resultToDelete: CustomEvent<QueryResultData>) {
		console.log('Удаление результата', resultToDelete);
		queriyResults = queriyResults.filter((result) => result !== resultToDelete.detail);
	}
</script>

<div class="mx-2 mt-10 h-[92vh] w-[620px] overflow-scroll">
	{#each queriyResults as queryResult}
		<div class="">
			<QueryResult on:onDelete={onDelete} {queryResult} />
		</div>
	{/each}
</div>
