<script lang="ts">
	import type { ContextMenuState, ContextMenuItem } from '$lib/models';
	import { onMount } from 'svelte';

	const closedState: ContextMenuState = {
		Opened: false,
		Options: [],
		X: 0,
		Y: 0
	};

	export let state: ContextMenuState = closedState;

	function onItemClick(item: ContextMenuItem) {
		item.callback();
		state = closedState;
	}

	function handleClickOutside(event: MouseEvent) {
		const menu = document.getElementById('context-menu');
		if (menu && !menu.contains(event.target as any)) {
			state.Opened = false;
		}
	}

	onMount(() => {
		window.addEventListener('click', handleClickOutside);
		// Remove event listener when component is destroyed
		return () => {
			window.removeEventListener('click', handleClickOutside);
		};
	});
</script>

{#if state.Opened}
	<div id="context-menu">
		<ul
			class="border-r-1 absolute w-32 list-inside rounded border border-primary-400 bg-primary-200 pb-2 pt-2 shadow-lg"
			style="top: {state.Y}px; left: {state.X}px;"
		>
			{#each state.Options as option}
				<li class="group mx-2 flex justify-start rounded hover:bg-primary-300">
					<button on:click={() => onItemClick(option)} class="w-full items-start rounded px-2 pl-2">
						<div class="text-s text-left text-primary-500">{option.text}</div>
					</button>
				</li>
			{/each}
		</ul>
	</div>
{/if}
