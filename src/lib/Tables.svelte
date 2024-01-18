<script>
// @ts-nocheck

    import { invoke } from '@tauri-apps/api/tauri'
    import { onMount } from 'svelte';
    let tables = [];
  
    onMount(async ()=>{await get_tables();});

    async function get_tables() {
      tables = await invoke('tables')
    }
</script>
{#each tables as table}
<div>
    {table}
</div>
{/each}  
  