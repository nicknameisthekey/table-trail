<script lang="ts">
	import { onMount } from 'svelte';
	import { Button, Input, Label } from 'flowbite-svelte';
	import 'flowbite/dist/flowbite.min.css';
	import { Modal } from 'flowbite-svelte';
	import { CirclePlusOutline } from 'flowbite-svelte-icons';

	interface Connection {
		id: number;
		name: string;
		status: 'connected' | 'disconnected';
		details: string;
	}

	//modal props
	let modalOpened = false;

	let connectionName: string = '';
	let host: string = '';
	let port: number | null = null;
	let password: string = '';

	let connections: Connection[] = [
		{
			id: 1,
			name: 'long long Connection 1',
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

	onMount(fetchConnections);

	async function fetchConnections() {
		//todo: fetch connections from backend
	} 

	function AddConnection() {
		connections = [
			...connections,
			{
				id: connections.length + 1,
				name: connectionName,
				status: 'disconnected',
				details: `Host: ${host}, Port: ${port}`
			}
		];

		password = '';
		connectionName = '';
		host = '';
		port = null;
		modalOpened = false;
		//todo save connection on backend
	}

	function onConnectionClick(connection: Connection) {
		console.log(connection);
		//todo change connection
	}
</script>

<div class="connections-sidebar h-screen bg-gray-500">
	{#each connections as connection (connection.id)}
		<div class="connection-container">
			<Button
				color="light"
				size="xs"
				class="h-20 w-80"
				on:click={() => onConnectionClick(connection)}
			>
				{connection.name}
			</Button>
		</div>
	{/each}

	<div class="connection-container">
		<Button color="light" size="xs" class="h-20 w-80" on:click={() => (modalOpened = true)}>
			<CirclePlusOutline
				class="h-12 w-12 text-gray-500 transition duration-75 group-hover:text-gray-900 dark:text-gray-400 dark:group-hover:text-white"
			/>
		</Button>
	</div>
</div>

<Modal title="Add connection" bind:open={modalOpened} autoclose>
	<div class="mb-6">
		<Label for="default-input" class="mb-2 block">Name</Label>
		<Input bind:value={connectionName} id="default-input" placeholder="Connection name" />
	</div>

	<div class="mb-6">
		<Label for="default-input" class="mb-2 block">Host</Label>
		<Input bind:value={host} id="default-input" placeholder="localhost" />
	</div>

	<div class="mb-6">
		<Label for="default-input" class="mb-2 block">Port</Label>
		<Input bind:value={port} id="default-input" placeholder="5555" />
	</div>

	<div class="mb-6">
		<Label for="default-input" class="mb-2 block">Password</Label>
		<Input bind:value={password} id="default-input" placeholder="Password" />
	</div>

	<svelte:fragment slot="footer">
		<Button on:click={AddConnection} class="w-1/2 mx-auto">Add</Button>
	</svelte:fragment>
</Modal>

<style>
	.connections-sidebar {
		width: 120px;
		margin-right: 10px;
		padding-right: 5px;
		border-right: solid 1px black;
	}

	.connection-container {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: center;
		padding-left: 2px;
		padding-right: 2px;
		margin-bottom: 3px;
	}
</style>
