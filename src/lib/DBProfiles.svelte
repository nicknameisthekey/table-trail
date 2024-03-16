<script lang="ts">
	import { onMount } from 'svelte';
	import { Button, Input, Label, Select } from 'flowbite-svelte';
	import { Modal } from 'flowbite-svelte';
	import { PlusSolid, DatabaseSolid, DotsVerticalSolid } from 'flowbite-svelte-icons';
	import { invoke } from '@tauri-apps/api/tauri';
	import { chosenProfile } from './store';
	import DbProfileContextMenu from './DbProfileContextMenu.svelte';
	import type { ContextMenuState, ContextMenuItem } from '$lib/models';

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

	let contextMenuState: ContextMenuState = {
		Opened: false,
		X: 0,
		Y: 0,
		Options: []
	};

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

	function onEdit(profile: ConnectionProfile) {
		console.log('on edit');
		console.log(profile);
	}

	function onProfileOptionsClick(event: MouseEvent, profile: ConnectionProfile) {
		event.preventDefault();

		contextMenuState = {
			X: event.clientX,
			Y: event.clientY,
			Opened: true,
			Options: [
				{ text: 'Edit', callback: () => onEdit(profile) },
				{ text: 'Delete', callback: () => {} }
			]
		};
	}
</script>

<ul class="h-screen w-48 list-inside border-r border-primary-500 bg-primary-300">
	{#each profiles as profile}
		<li class="group flex justify-start pl-3 hover:bg-primary-400">
			<button
				class="flex h-6 w-full items-center text-primary-500"
				on:contextmenu={(e) => onProfileOptionsClick(e, profile)}
				on:click={() => onProfileClick(profile)}
			>
				<DatabaseSolid class="mr-2 size-4 text-center" />
				{profile.params.name}
			</button>
			<button
				class="invisible items-end rounded border border-transparent hover:border hover:border-primary-500 group-hover:visible"
			>
				<DotsVerticalSolid />
			</button>
		</li>
	{/each}
	<li class="mt-3 flex h-8 items-center justify-center border-t border-primary-500">
		<button
			class="p mr-2 rounded border border-transparent p-1 text-primary-500 hover:border hover:border-primary-500"
			on:click={() => (modalOpened = true)}
		>
			<PlusSolid class="size-4" />
		</button>
	</li>
</ul>

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

<DbProfileContextMenu bind:state={contextMenuState}></DbProfileContextMenu>
