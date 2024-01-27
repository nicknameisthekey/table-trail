<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import type * as Monaco from 'monaco-editor/esm/vs/editor/editor.api';
	import { queryStore, resultStore } from './store';
	import { invoke } from '@tauri-apps/api/tauri';

	let editor: Monaco.editor.IStandaloneCodeEditor;
	let monaco: typeof Monaco;
	let editorContainer: HTMLElement;
	let uri: Monaco.Uri;

	onMount(async () => {
		monaco = (await import('./monaco')).default;
		uri = monaco.Uri.parse('model1');

		editor = monaco.editor.create(editorContainer, {
			minimap: { enabled: false }
		});
		const model = monaco.editor.createModel('select * from ....', 'sql', uri);
		editor.setModel(model);

		queryStore.subscribe((v) => ChangeModel(v));
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
		let result: Record<string, string>[] = await invoke('send_query', { query: query });
		console.log(result);
		resultStore.set(result);
	}
</script>

<div>
	<div>
		<button class="btn variant-filled btn-sm" on:click={SendQuery}>SEND</button>
	</div>
	<div class="container" bind:this={editorContainer} />
</div>

<style>
	.container {
		width: 500px;
		height: 250px;
	}
</style>
