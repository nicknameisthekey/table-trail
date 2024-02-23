<script lang="ts">
	import { onMount } from 'svelte';
	import { Button, Input, Label } from 'flowbite-svelte';
	import 'flowbite/dist/flowbite.min.css';
	import { Modal } from 'flowbite-svelte';
	import { CirclePlusOutline } from 'flowbite-svelte-icons';
	import { invoke } from '@tauri-apps/api/tauri';

	interface Connection {
		name: string;
		host: string;
		port: number;
		username: string;
		password: string;
		database_type: string;
		status: 'connected' | 'disconnected';
	}

	//modal props
	let modalOpened = false;

	let connectionName: string = '';
	let host: string = '';
	let port: number | null = null;
	let password: string = '';
	let username: string = '';

	let connections: Connection[] = [];

	onMount(fetchConnections);

	async function fetchConnections() {
		connections = await invoke('connection_configs');
		console.log(connections);
	}

	async function AddConnection() {
		let config = {
			name: connectionName,
			host: host,
			port: parseInt(port as any), 
			username: username,
			password: password,
			database_type: 'Sqlite' //todo
		};

		console.log(config);
		await invoke('add_db_config', {
			config: config
		});

		await fetchConnections();

		connectionName = '';
		host = '';
		port = null;
		username = '';
		password = '';

		modalOpened = false;
		//todo save connection on backend
	}

	function onConnectionClick(connection: Connection) {
		console.log(connection);
		//todo change connection
	}
</script>

<div class="connections-sidebar h-screen bg-gray-500">
	{#each connections as connection}
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
		<Label for="default-input" class="mb-2 block">Useraname</Label>
		<Input bind:value={username} id="default-input" placeholder="Password" />
	</div>

	<div class="mb-6">
		<Label for="default-input" class="mb-2 block">Password</Label>
		<Input bind:value={password} id="default-input" placeholder="Password" />
	</div>

	<svelte:fragment slot="footer">
		<Button on:click={AddConnection} class="mx-auto w-1/2">Add</Button>
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
