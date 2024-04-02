<script lang="ts">
    import "../app.pcss";

    import LeftRail from "../components/LeftRail.svelte";
    import TopBar from "../components/TopBar.svelte";

    console.log();

    const route = ["Agenda"];

    $: routeJoined = "agenda" as any;
</script>

<div class="flex flex-col h-full w-full">
    <TopBar {route} />
    <div class="flex flex-row h-full w-full">
        <LeftRail route={routeJoined} />
        <slot />
    </div>
</div>
