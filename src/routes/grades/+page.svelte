<script lang="ts">
    import Table from "../../components/TableGrade.svelte";
    import { initializeStores } from '@skeletonlabs/skeleton';
    import { Drawer, getDrawerStore } from '@skeletonlabs/skeleton';
    import type { DrawerSettings, DrawerStore } from '@skeletonlabs/skeleton';

    initializeStores();
    const drawerStore = getDrawerStore();
    const drawerSettings: DrawerSettings = {
        id: 'example-3',
        // Provide your property overrides:
        bgDrawer: 'bg-purple-900 text-white',
        bgBackdrop: 'bg-gradient-to-tr from-indigo-500/50 via-purple-500/50 to-pink-500/50',
        width: 'md:w-[480px] w-full',
        padding: 'p-4',
        rounded: 'rounded-xl',
    };
    
    function openM (){
        drawerStore.open(drawerSettings);
    } 
</script>

<Drawer />

<div class="flex flex-col items-center p-6 w-full">
    <Table />
    <button on:click={openM}>AQUI</button>
</div>