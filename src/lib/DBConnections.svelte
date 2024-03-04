<script lang="ts">
	import { onMount } from 'svelte';
	import { Button, Input, Label, P, Select } from 'flowbite-svelte';
	import 'flowbite/dist/flowbite.min.css';
	import { Modal } from 'flowbite-svelte';
	import { CirclePlusOutline } from 'flowbite-svelte-icons';
	import { invoke } from '@tauri-apps/api/tauri';
	import { chosenProfile } from './store';

	type ConnectionProfile = {
		id: number;
		params: ConnectionParams;
		connected: boolean;
	};

	type ConnectionParams = {
		name: string;
		host: string;
		port: string | null;
		username: string | null;
		password: string | null;
		database_type: string;
	};

	let modalOpened = false;

	let databaseTypes = [
		{ value: 'sqlite', name: 'Sqlite' },
		{ value: 'postgres', name: 'Postgres' }
	];

	const emptyModalState: ConnectionProfile = {
		params: {
			name: '',
			host: '',
			port: null,
			username: null,
			password: null,
			database_type: 'sqlite'
		},
		id: 0,
		connected: false
	};

	let addProfileModalState: ConnectionProfile = emptyModalState;

	let profiles: ConnectionProfile[] = [];

	onMount(fetchProfiles);

	async function fetchProfiles() {
		profiles = await invoke('connection_profiles');
		console.log('[fetching profiles]', profiles);
	}

	async function addProfile() {
		if (addProfileModalState.params.database_type === 'sqlite') {
			addProfileModalState.params.port = '0';
			addProfileModalState.params.username = '';
			addProfileModalState.params.password = '';
		}

		console.log('adding profile', addProfileModalState);
		await invoke('add_profile', {
			profile: addProfileModalState
		});

		addProfileModalState = emptyModalState;
		modalOpened = false;

		await fetchProfiles();
	}

	function onProfileClick(profile: ConnectionProfile) {
		console.log('clicked on profile ', profile);
		chosenProfile.set(profile.id);
	}
</script>

<div class="profiles-sidebar h-screen bg-gray-500">
	{#each profiles as profile}
		<div class="profile-container">
			<Button
				color={profile.connected ? 'green' : 'red'}
				size="xs"
				class="h-20 w-80"
				on:click={() => onProfileClick(profile)}
			>
				{profile.params.name}
			</Button>
		</div>
	{/each}

	<div class="profile-container">
		<Button color="light" size="xs" class="h-20 w-80" on:click={() => (modalOpened = true)}>
			<CirclePlusOutline
				class="h-12 w-12 text-gray-500 transition duration-75 group-hover:text-gray-900 dark:text-gray-400 dark:group-hover:text-white"
			/>
		</Button>
	</div>
</div>

<Modal title="Add profile" bind:open={modalOpened} autoclose>
	<Select id="countries" class="mt-2" bind:value={addProfileModalState.params.database_type}>
		<option selected value={addProfileModalState.params.database_type}
			>{addProfileModalState.params.database_type}</option
		>

		{#each databaseTypes as { value, name }}
			<option {value}>{name}</option>
		{/each}
	</Select>

	<div class="mb-6">
		<Label for="default-input" class="mb-2 block">Name</Label>
		<Input
			bind:value={addProfileModalState.params.name}
			id="default-input"
			placeholder="Connection name"
		/>
	</div>

	<div class="mb-6">
		{#if addProfileModalState.params.database_type === 'sqlite'}
			<Label for="default-input" class="mb-2 block">Db file</Label>
			<Input
				bind:value={addProfileModalState.params.host}
				id="default-input"
				placeholder="/path/to/sqlite.db"
			/>
		{:else}
			<Label for="default-input" class="mb-2 block">Host</Label>
			<Input
				bind:value={addProfileModalState.params.host}
				id="default-input"
				placeholder="localhost"
			/>
		{/if}
	</div>

	{#if addProfileModalState.params.database_type != 'sqlite'}
		<div class="mb-6">
			<Label for="default-input" class="mb-2 block">Port</Label>
			<Input bind:value={addProfileModalState.params.port} id="default-input" placeholder="5555" />
		</div>

		<div class="mb-6">
			<Label for="default-input" class="mb-2 block">Useraname</Label>
			<Input
				bind:value={addProfileModalState.params.username}
				id="default-input"
				placeholder="Password"
			/>
		</div>

		<div class="mb-6">
			<Label for="default-input" class="mb-2 block">Password</Label>
			<Input
				bind:value={addProfileModalState.params.password}
				id="default-input"
				placeholder="Password"
			/>
		</div>
	{/if}
	<svelte:fragment slot="footer">
		<Button on:click={addProfile} class="mx-auto w-1/2">Add</Button>
	</svelte:fragment>
</Modal>

<style>
	.profiles-sidebar {
		width: 120px;
		margin-right: 10px;
		padding-right: 5px;
		border-right: solid 1px black;
	}

	.profile-container {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: center;
		padding-left: 2px;
		padding-right: 2px;
		margin-bottom: 3px;
	}
</style>
