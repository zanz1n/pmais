<script lang="ts">
    import LeftRail from "../../components/NavRail.svelte";
    import TopBar from "../../components/TopBar.svelte";
    const route = ["Notas"];
</script>


<div class="flex flex-col h-full w-full">
    <TopBar {route} />
    <div class="flex sm:flex-row flex-col gap-4 sm:gap-0 h-full w-full">
        <LeftRail route="grades" />
        <slot />
    </div>
</div>
