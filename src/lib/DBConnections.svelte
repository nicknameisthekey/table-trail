<script lang="ts">
	type Connection = { id: number; name: any; host: any; port: any; username: any; password: any };

	export let showDbConnections: Boolean;

	let dialog: HTMLDialogElement;

	$: if (dialog && showDbConnections) dialog.showModal();

	let connections = [
		{
			id: 1,
			name: 'Connection 1',
			host: 'localhost',
			port: '5432',
			username: 'user1',
			password: 'password1'
		},
		{
			id: 2,
			name: 'Connection 2',
			host: 'example.com',
			port: '5432',
			username: 'user2',
			password: 'password2'
		}
		// Add more connections as needed
	];

	let selectedConnection: Connection | null = null;

	const selectConnection = (connection: Connection) => {
		selectedConnection = connection;
	};

	const clearSelection = () => {
		selectedConnection = null;
	};
</script>

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions -->
<dialog
	bind:this={dialog}
	on:close={() => (showDbConnections = false)}
	on:click|self={() => dialog.close()}
>
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<div on:click|stopPropagation>
		<slot name="header" />

		<div class="container">
			<div class="connections-list">
				{#each connections as connection}
					<div
						class="connection-item {selectedConnection &&
							selectedConnection.id === connection.id &&
							'selected-connection'}"
						on:click={() => selectConnection(connection)}
					>
						{connection.name}
					</div>
				{/each}
			</div>

			<div class="connection-form">
				{#if selectedConnection}
					<h2>Edit Connection</h2>
					<form on:submit|preventDefault={clearSelection}>
						<label>Name: <input bind:value={selectedConnection.name} /></label><br />
						<label>Host: <input bind:value={selectedConnection.host} /></label><br />
						<label>Port: <input bind:value={selectedConnection.port} /></label><br />
						<label>Username: <input bind:value={selectedConnection.username} /></label><br />
						<label
							>Password: <input type="password" bind:value={selectedConnection.password} /></label
						><br />
						<button type="submit">Save</button>
						<button on:click={clearSelection}>Cancel</button>
					</form>
				{:else}
					<p>Select a connection to edit.</p>
				{/if}
			</div>
		</div>

		<!-- svelte-ignore a11y-autofocus -->
		<button autofocus on:click={() => dialog.close()}>close modal</button>
	</div>
</dialog>

<style>
	.container {
		display: flex;
	}

	.connections-list {
		flex: 1;
		padding: 10px;
		border-right: 1px solid #ccc;
	}

	.connection-form {
		flex: 2;
		padding: 10px;
	}

	.connection-item {
		cursor: pointer;
		padding: 5px;
		margin-bottom: 5px;
		border: 1px solid #ccc;
		border-radius: 4px;
	}

	.selected-connection {
		background-color: #e0e0e0;
	}

	/* svelte modal */
	dialog {
		max-width: 32em;
		border-radius: 0.2em;
		border: none;
		padding: 0;
	}
	dialog::backdrop {
		background: rgba(0, 0, 0, 0.3);
	}
	dialog > div {
		padding: 1em;
	}
	dialog[open] {
		animation: zoom 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
	}
	@keyframes zoom {
		from {
			transform: scale(0.95);
		}
		to {
			transform: scale(1);
		}
	}
	dialog[open]::backdrop {
		animation: fade 0.2s ease-out;
	}
	@keyframes fade {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}
	button {
		display: block;
	}
</style>
