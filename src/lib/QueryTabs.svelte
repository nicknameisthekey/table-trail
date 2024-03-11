<script lang="ts">
	import { CloseSolid, PlusSolid } from 'flowbite-svelte-icons';

	type QueryTab = {
		name: String;
		query: String;
		active: Boolean;
	};

	const activeTabClasses = 'bg-primary-100 text-primary-800';
	const inactiveTabClasses = 'bg-primary-300 text-primary-500';
	const tabClasses =
		'flex items-center px-3 h-8 min-w-32 max-w-48 w-auto text-xs group hover:cursor-pointer';

	const getTabClasses = (active: Boolean) =>
		(active ? activeTabClasses : inactiveTabClasses) + ' ' + tabClasses;

	const emptyTab = (active: Boolean): QueryTab => {
		return {
			name: 'Query big big big big' + new Date().toLocaleTimeString(),
			query: '',
			active: active
		};
	};

	const addTab = (active: Boolean) => {
		tabs.forEach((t) => (t.active = false));
		tabs = [...tabs, emptyTab(active)];
	};

	const activate = (tab: QueryTab) => {
		tabs.forEach((t) => (t.active = false));
		tab.active = true;
		tabs = tabs; //hack to force reactivity
	};

	const close = (tab: QueryTab) => {
		console.log('closing tab', tab);
		tabs = tabs.filter((t) => t !== tab);
	};

	let tabs: QueryTab[] = [emptyTab(true)];
</script>

<div class="bg-primary-100 divide-primary-500 border-primary-500 flex divide-x border-b">
	{#each tabs as tab}
		<!-- svelte-ignore a11y-click-events-have-key-events -->
		<!-- svelte-ignore a11y-no-static-element-interactions -->
		<div class={getTabClasses(tab.active)} on:click={() => activate(tab)}>
			<div class="truncate px-3">{tab.name}</div>
			<div
				class="hover:border-primary-800 invisible rounded border border-transparent hover:border group-hover:visible"
				on:click={() => close(tab)}
			>
				<CloseSolid class="text-primary-800 size-3" />
			</div>
		</div>
	{/each}

	<button class="bg-primary-100 w-8 content-center" on:click={() => addTab(true)}>
		<div class="border-primary-500 flex items-center justify-center">
			<PlusSolid class="text-primary-500 size-4" />
		</div>
	</button>
</div>
