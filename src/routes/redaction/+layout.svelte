<script lang="ts">
    import LeftRail from "../../components/NavRail.svelte";
    import TopBar from "../../components/TopBar.svelte";

    const route = ["Redação"];
</script>

<div class="flex flex-col h-full w-full">
    <TopBar {route} />
    <div class="flex flex-row h-full w-full">
        <LeftRail route="redaction" />
        <slot />
    </div>
</div>
