<script lang="ts">
	import { Avatar, type ModalComponent, type ModalSettings } from '@skeletonlabs/skeleton';
	import { onMount } from 'svelte';
	import AddDbConnection from './AddDBConnection.svelte';
	import { getModalStore } from '@skeletonlabs/skeleton';

  const modalStore = getModalStore();
	const addConnModal: ModalComponent = { ref: AddDbConnection };
	const modal: ModalSettings = {
		type: 'component',
		component: addConnModal
	};

	type ConnectionStatus = 'connected' | 'disconnected';

	interface Connection {
		id: number;
		name: string;
		status: ConnectionStatus;
		details: string;
	}

	const connections: Connection[] = [
		{
			id: 1,
			name: 'Connection 1',
			status: 'connected',
			details: 'Connection details 1'
		},
		{
			id: 2,
			name: 'Connection 2',
			status: 'disconnected',
			details: 'Connection details 2'
		},
		{
			id: 3,
			name: 'Connection 3',
			status: 'connected',
			details: 'Connection details 3'
		}
	];

	async function fetchConnections() {}
	function getFirstFourLetters(str: string | null | undefined): string {
		return str && str.length >= 4 ? str.substring(0, 4) : '';
	}

	function openAddConnectionModal(e: MouseEvent) {
		console.log('openAddConnectionModal');
		modalStore.trigger(modal);
    console.log($modalStore[0].title);
	}

	onMount(fetchConnections);
</script>

{#each connections as connection (connection.id)}
	<Avatar
		class="border-surface-300-600-token m-2 border-4"
		rounded="rounded-2xl"
		cursor="cursor-pointer"
		initials={getFirstFourLetters(connection.name)}
	/>
{/each}
<Avatar
	class="border-surface-300-600-token m-2 border-4"
	rounded="rounded-2xl"
	cursor="cursor-pointer"
	initials="+"
	on:click={openAddConnectionModal}
/>
