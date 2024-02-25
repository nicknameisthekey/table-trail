<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import type * as Monaco from 'monaco-editor/esm/vs/editor/editor.api';
	import { queryStore, resultStore } from './store';
	import { invoke } from '@tauri-apps/api/tauri';
	import { Button } from 'flowbite-svelte';

	let editor: Monaco.editor.IStandaloneCodeEditor;
	let monaco: typeof Monaco;
	let editorContainer: HTMLElement;
	let uri: Monaco.Uri;

	onMount(async () => {
		monaco = (await import('./monaco')).default;
		uri = monaco.Uri.parse('model1');

		editor = monaco.editor.create(editorContainer, {
			minimap: { enabled: false},  theme: 'vs-dark',
			automaticLayout: true
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
<!-- todo: wrap every editor and result in tabs  -->
<div class="query-editor h-screen">
	<div class="text-right">
		<Button class="btn variant-filled btn-sm mb-3" on:click={SendQuery}>SEND</Button>
	</div>
	<div class="monaco-editor min-h-[40vh] h-[90%]" bind:this={editorContainer} />
</div>

<style>
	.query-editor {
		max-width: 700px;
		min-width: 300px;
		margin-right: 10px;
	}

	.monaco-editor {
		min-width: 400px;
		width: 700px;
	}
</style>
