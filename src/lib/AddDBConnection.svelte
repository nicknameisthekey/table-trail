<script lang="ts">
	import { getModalStore } from '@skeletonlabs/skeleton';

	const modalStore = getModalStore();
	let name: string = '';
	let host: string = '';
	let port: number | null = null;
	let password: string = '';

	function add(){
		modalStore.close();
	}
</script>

{#if $modalStore[0]}
	<div class="modal-form text-2xl font-bold">
		<header class="text-2xl font-bold">{$modalStore[0].title ?? '(title missing)'}</header>
		<article>{$modalStore[0].body ?? '(body missing)'}</article>
		<!-- Enable for debugging: -->
		<form class="modal-form border border-surface-500 p-4 space-y-4 rounded-container-token">
			<input type="text" bind:value={name} placeholder="Name" />
			<input type="text" bind:value={host} placeholder="Host" />
			<input type="number" bind:value={port} placeholder="Port" />
			<input type="password" bind:value={password} placeholder="Password" />
		</form>
		<button on:click={add}>Add</button>
	</div>
{/if}
