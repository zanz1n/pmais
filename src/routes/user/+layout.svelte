<script lang="ts">
    import LeftRail from "../../components/NavRail.svelte";
    import TopBar from "../../components/TopBar.svelte";

    const route = ["User"];
</script>

<div class="flex flex-col h-full w-full">
    <TopBar {route} />
    <div class="flex flex-row h-full w-full">
        <LeftRail route="user" />
        <slot />
    </div>
</div>