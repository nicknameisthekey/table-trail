<script lang="ts">
	import type { QueryResultData } from '$lib/models';
	import { TrashBinOutline } from 'flowbite-svelte-icons';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();
	export let queryResult: QueryResultData;
</script>

<div class="mb-5">
	<div class="mb-2 flex justify-end">
		<button
			on:click={() => dispatch('onDelete', queryResult)}
			class="text-primary-700 hover:border-primary-700 mr-1 rounded border-2 border-transparent"
		>
			<TrashBinOutline class="size-7" />
		</button>
	</div>
	<div class="h-[350px] w-[600px] overflow-scroll">
		<table class="divide-primary-300 divide-y">
			<thead class="bg-primary-300">
				<tr>
					{#each queryResult.columns as header}
						<th class="text-primary-500 px-2 py-1 text-left text-xs">
							{header}
						</th>
					{/each}
				</tr>
			</thead>
			<tbody class="divide-primary-300 bg-primary-50 divide-y">
				{#each queryResult.rows as row}
					<tr>
						{#each row as cell}
							<td class="text-s whitespace-nowrap px-1 py-1 text-left"> {cell} </td>
						{/each}
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
</div>
