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
		monaco.editor.defineTheme('app-theme', {
			base: 'vs',
			inherit: true,
			rules: [],
			encodedTokensColors: [],
			colors: {
				focusBorder: '#22a2c966',
				foreground: '#5e6687',
				'widget.shadow': '#979db4',
				'selection.background': '#3d8fd199',
				descriptionForeground: '#293256',
				'textLink.foreground': '#22a2c9CC',
				'textLink.activeForeground': '#22a2c9',
				'input.background': '#dfe2f1',
				'input.foreground': '#293256',
				'input.border': '#979db480',
				'input.placeholderForeground': '#5e6687AA',
				'inputOption.activeBorder': '#22a2c999',
				'inputValidation.infoBorder': '#6679cc',
				'inputValidation.infoBackground': '#f5f7ff',
				'inputValidation.warningBackground': '#c08b30',
				'inputValidation.warningBorder': '#c08b3088',
				'inputValidation.errorBackground': '#c94922',
				'inputValidation.errorBorder': '#c9492288',
				'punctuation.definition.generic.begin.html': '#9c637a',
				errorForeground: '#c9492222',
				'badge.background': '#3d8fd1',
				'badge.foreground': '#f5f7ff',
				'progress.background': '#979db4',
				'progressBar.background': '#3d8fd188',
				'dropdown.background': '#f5f7ff',
				'dropdown.foreground': '#293256',
				'dropdown.border': '#22a2c999',
				'button.background': '#3d8fd1',
				'button.hoverBackground': '#3d8fd180',
				'button.foreground': '#f5f7ff',
				'welcomePage.buttonBackground': '#dfe2f1',
				'welcomePage.buttonHoverBackground': '#dfe2f180',
				'list.activeSelectionBackground': '#979db480',
				'list.activeSelectionForeground': '#202746',
				'list.focusBackground': '#979db440',
				'list.focusForeground': '#293256',
				'list.hoverBackground': '#979db440',
				'list.hoverForeground': '#293256',
				'list.inactiveSelectionBackground': '#979db426',
				'list.inactiveSelectionForeground': '#293256',
				'list.dropBackground': '#979db477',
				'list.highlightForeground': '#22a2c9',
				'list.errorForeground': '#c94922',
				'list.warningForeground': '#c94922',
				'list.invalidItemForeground': '#9c637a',
				'gitDecoration.addedResourceForeground': '#ac9739',
				'gitDecoration.modifiedResourceForeground': '#22a2c9',
				'gitDecoration.untrackedResourceForeground': '#293256',
				'gitDecoration.deletedResourceForeground': '#c9492290',
				'gitDecoration.ignoredResourceForeground': '#979db480',
				'gitDecoration.conflictingResourceForeground': '#c08b30',
				'scrollbar.shadow': '#979db4',
				'scrollbarSlider.activeBackground': '#979db4',
				'scrollbarSlider.background': '#dfe2f1',
				'scrollbarSlider.hoverBackground': '#979db480',
				'editorGutter.background': '#dfe2f14D',
				'editorGutter.modifiedBackground': '#c08b30',
				'editorGutter.addedBackground': '#ac973980',
				'editorGutter.deletedBackground': '#c76b29',
				'diffEditor.insertedTextBackground': '#ac973923',
				'diffEditor.insertedTextBorder': '#ac973933',
				'diffEditor.removedTextBackground': '#ac973933',
				'diffEditor.removedTextBorder': '#ac97394D',
				'editorWidget.background': '#dfe2f1',
				'editorWidget.border': '#dfe2f180',
				'editorSuggestWidget.background': '#dfe2f1',
				'editorSuggestWidget.border': '#dfe2f180',
				'editorSuggestWidget.foreground': '#5e6687',
				'editorSuggestWidget.highlightForeground': '#c76b29',
				'editorSuggestWidget.selectedBackground': '#979db440',
				'editorHoverWidget.background': '#dfe2f1',
				'editorHoverWidget.border': '#dfe2f180',
				'debugExceptionWidget.background': '#dfe2f1',
				'debugExceptionWidget.border': '#dfe2f180',
				'editor.background': '#f1f5f9',
				'editor.foreground': '#5e6687',
				'editorLineNumber.foreground': '#5e668759',
				'editorLineNumber.activeForeground': '#898ea4',
				'editorCursor.foreground': '#5e6687',
				'editorCursor.background': '#f5f7ff',
				'editorWhitespace.foreground': '#5e668764',
				'editor.lineHighlightBackground': '#dfe2f1',
				'editor.rangeHighlightBackground': '#dfe2f1',
				'editor.selectionBackground': '#dfe2f1E6',
				'editor.selectionHighlightBackground': '#dfe2f18C',
				'editor.inactiveSelectionBackground': '#5e66878C',
				'editorIndentGuide.background': '#5e668726',
				'editorIndentGuide.activeBackground': '#5e668780',
				'editorRuler.foreground': '#5e668780',
				'editorCodeLens.foreground': '#5e6687BF',
				'editorMarkerNavigation.background': '#5e6687',
				'editorMarkerNavigationError.background': '#c94922',
				'editorMarkerNavigationWarning.background': '#c08b30',
				'editor.findMatchBackground': '#979db440',
				'editor.findMatchHighlightBackground': '#898ea440',
				'editor.hoverHighlightBackground': '#979db464',
				'editor.wordHighlightBackground': '#dfe2f1A6',
				'editor.wordHighlightStrongBackground': '#dfe2f18C',
				'editorBracketMatch.background': '#dfe2f1',
				'editorBracketMatch.border': '#5e668780',
				'editorOverviewRuler.currentContentForeground': '#9c637a',
				'editorOverviewRuler.incomingContentForeground': '#9c637a',
				'editorOverviewRuler.commonContentForeground': '#9c637a',
				'editorError.foreground': '#c76b29',
				'editorWarning.foreground': '#c08b30',
				'peekViewEditor.background': '#f5f7ff',
				'peekViewTitle.background': '#f5f7ffF2',
				'peekView.border': '#5e6687',
				'peekViewEditor.matchHighlightBackground': '#6679cc4D',
				'peekViewResult.background': '#f5f7ff',
				'peekViewResult.fileForeground': '#293256',
				'peekViewResult.lineForeground': '#293256',
				'peekViewResult.matchHighlightBackground': '#202746CC',
				'peekViewResult.selectionBackground': '#dfe2f1',
				'peekViewResult.selectionForeground': '#5e6687',
				'peekViewTitleDescription.foreground': '#5e6687',
				'peekViewTitleLabel.foreground': '#5e6687',
				'merge.currentHeaderBackground': '#5e6687',
				'merge.incomingHeaderBackground': '#6679cc',
				'editorGroup.background': '#5e6687',
				'editorGroup.border': '#dfe2f1',
				'editorGroup.dropBackground': '#22a2c944',
				'editorGroupHeader.tabsBackground': '#dfe2f1F2',
				'editorGroupHeader.noTabsBackground': '#dfe2f1F2',
				'editorGroupHeader.tabsBorder': '#dfe2f1F2',
				'tab.activeForeground': '#293256',
				'tab.activeBackground': '#f5f7ff',
				'tab.inactiveForeground': '#898ea4',
				'tab.unfocusedActiveForeground': '#898ea4',
				'tab.unfocusedInactiveForeground': '#898ea4',
				'tab.inactiveBackground': '#dfe2f1',
				'tab.border': '#f5f7ff',
				'tab.activeBorder': '#f5f7ff',
				'tab.hoverBackground': '#f5f7ff',
				'tab.unfocusedActiveBorder': '#dfe2f1',
				'breadcrumbPicker.background': '#dfe2f1F2',
				'activityBar.background': '#979db480',
				'activityBar.foreground': '#5e6687',
				'activityBar.border': '#979db440',
				'activityBar.dropBackground': '#dfe2f1E6',
				'activityBarBadge.background': '#3d8fd1',
				'activityBarBadge.foreground': '#f5f7ff',
				'panel.background': '#979db440',
				'panel.border': '#dfe2f1',
				'panelTitle.activeBorder': '#6679cc',
				'panelTitle.activeForeground': '#202746CC',
				'panelTitle.inactiveForeground': '#6b7394',
				'panel.dropBackground': '#22a2c940',
				'sideBar.background': '#f5f7ff',
				'sideBar.foreground': '#979db4',
				'sideBarTitle.foreground': '#5e6687',
				'sideBarSectionHeader.background': '#979db459',
				'sideBarSectionHeader.foreground': '#5e6687',
				'sideBar.border': '#979db480',
				'statusBar.foreground': '#5e6687',
				'statusBar.background': '#979db480',
				'statusBar.border': '#979db440',
				'statusBar.debuggingBackground': '#898ea4',
				'statusBar.debuggingBorder': '#6b7394',
				'statusBar.noFolderBackground': '#979db440',
				'statusBar.noFolderBorder': '#979db480',
				'statusBarItem.prominentBackground': '#6b739440',
				'statusBarItem.prominentHoverBackground': '#6b739480',
				'statusBarItem.activeBackground': '#5e6687',
				'statusBarItem.hoverBackground': '#6b739426',
				'titleBar.activeBackground': '#f5f7ff',
				'titleBar.activeForeground': '#293256',
				'titleBar.inactiveBackground': '#dfe2f1',
				'notifications.background': '#dfe2f1',
				'notifications.foreground': '#293256',
				'notificationLink.foreground': '#22a2c9',
				'extensionButton.prominentBackground': '#ac9739CC',
				'extensionButton.prominentHoverBackground': '#ac9739',
				'extensionButton.prominentForeground': '#202746',
				'pickerGroup.foreground': '#293256',
				'pickerGroup.border': '#dfe2f1',
				'debugToolBar.background': '#f5f7ff',
				'walkThrough.embeddedEditorBackground': '#dfe2f1',
				'source.elm': '#979db480',
				'string.quoted.single.js': '#202746',
				'meta.objectliteral.js': '#6679cc',
				'terminal.background': '#dfe2f1',
				'terminal.foreground': '#202746',
				'terminal.ansiBlack': '#293256',
				'terminal.ansiRed': '#c94922',
				'terminal.ansiGreen': '#ac9739',
				'terminal.ansiYellow': '#c08b30',
				'terminal.ansiBlue': '#3d8fd1',
				'terminal.ansiMagenta': '#9c637a',
				'terminal.ansiCyan': '#22a2c9',
				'terminal.ansiWhite': '#5e6687',
				'terminal.ansiBrightBlack': '#dfe2f1',
				'terminal.ansiBrightRed': '#c76b29',
				'terminal.ansiBrightGreen': '#5e6687',
				'terminal.ansiBrightYellow': '#6b7394',
				'terminal.ansiBrightBlue': '#898ea4',
				'terminal.ansiBrightMagenta': '#6679cc',
				'terminal.ansiBrightCyan': '#979db4',
				'terminal.ansiBrightWhite': '#202746'
			}
		});

		editor = monaco.editor.create(editorContainer, {
			minimap: { enabled: false },
			theme: 'app-theme',
			automaticLayout: true,
			inDiffEditor: false,
			overviewRulerLanes: 0,
			selectionHighlight: false,
			renderLineHighlight: 'none'
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
			<span class="text-black-300 group-hover:text-primary-500">Run</span>
			<CaretRightSolid class="text-black-300 ml-2 group-hover:text-primary-500" />
		</button>
	</div>
	<div class="h-[90%] min-h-[40vh] border border-primary-400" bind:this={editorContainer} />
</div>
