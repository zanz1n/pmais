<script lang="ts">
    import LeftRail from "../../components/LeftRail.svelte";
    import TopBar from "../../components/TopBar.svelte";

    const route = ["Notas"];
</script>

<div class="flex flex-col h-full w-full">
    <TopBar {route} />
    <div class="flex flex-row h-full w-full">
        <LeftRail route="grades" />
        <slot />
    </div>
</div>