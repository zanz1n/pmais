<script lang="ts">
    import LeftRail from "../../components/LeftRail.svelte";
    import TopBar from "../../components/TopBar.svelte";

    const route = ["Simulado"];
</script>

<div class="flex flex-col h-full w-full">
    <TopBar {route} />
    <div class="flex flex-row h-full w-full">
        <LeftRail route="saep" />
        <slot />
    </div>
</div>