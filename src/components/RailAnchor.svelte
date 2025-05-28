<script lang="ts">
    export let selected = false;

    const cWrapper =
        "w-20 h-20 flex flex-col justify-center items-stretch text-center space-y-1";

    $: classActive = selected ? "bg-primary-active-token" : "";
    $: classesWrapper = `${cWrapper} aspect-square bg-primary-hover-token space-y-1 ${classActive}`;
</script>

<a class="app-rail-anchor unstyled" href={$$props.href}>
    <div class="app-rail-wrapper {classesWrapper}">
        {#if $$slots.lead}<div
                class="app-rail-lead flex justify-center items-center"
            >
                <slot name="lead" />
            </div>
        {/if}
        <div class="app-rail-label font-bold text-xs"><slot /></div>
    </div>
</a>
