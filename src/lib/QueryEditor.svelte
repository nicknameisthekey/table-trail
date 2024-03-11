<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import type * as Monaco from 'monaco-editor/esm/vs/editor/editor.api';
	import { queryStore, resultStore, chosenProfile } from './store';
	import { invoke } from '@tauri-apps/api/tauri';
	import { CaretRightSolid } from 'flowbite-svelte-icons';

	let editor: Monaco.editor.IStandaloneCodeEditor;
	let monaco: typeof Monaco;
	let editorContainer: HTMLElement;
	let uri: Monaco.Uri;
	let currentProfile: number;

	type QueryResult = { columns: string[]; rows: string[][] };

	onMount(async () => {
		monaco = (await import('./monaco')).default;
		uri = monaco.Uri.parse('model1');

		editor = monaco.editor.create(editorContainer, {
			minimap: { enabled: false },
			theme: 'vs',
			automaticLayout: true
		});
		const model = monaco.editor.createModel('select * from ....', 'sql', uri);
		editor.setModel(model);

		queryStore.subscribe((v) => ChangeModel(v));

		chosenProfile.subscribe((profileId) => {
			if (profileId != 0) currentProfile = profileId;
		});
	});

	export function ChangeModel(value: string) {
		console.log('received ' + value);
		const model = monaco.editor.getModel(uri);
		model?.setValue(value);
		editor.setModel(model);
	}

	onDestroy(() => {
		monaco?.editor.getModels().forEach((model) => model.dispose());
		editor?.dispose();
	});

	async function SendQuery() {
		let query = monaco.editor.getModel(uri)?.getValue();
		console.log(query);
		let result: QueryResult = await invoke('send_query', {
			query: query,
			profile_id: currentProfile
		});
		console.log(result);
		resultStore.set(result);
	}
</script>

<!-- todo: wrap every editor and result in tabs  -->
<div class="h-screen w-[40%]">
	<div class="group my-2 flex h-8 items-center">
		<button
			class="flex h-full items-center rounded-md bg-green-400 px-2 hover:bg-green-300"
			on:click={SendQuery}
		>
			<span class="group-hover:text-primary-500 text-black-300">Run</span>
			<CaretRightSolid class="group-hover:text-primary-500 text-black-300 ml-2" />
		</button>
	</div>
	<div class="h-[90%] min-h-[40vh]" bind:this={editorContainer} />
</div>
