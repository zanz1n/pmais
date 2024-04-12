<script lang="ts">
    import type { Subject } from "$lib/grades/grades";
    import AngleLeftSvg from "../../icons/AngleLeftSvg.svelte";

    export let data: Subject;
    export let onClose: () => void;
</script>

<div class="flex flex-col p-6 w-full h-full">
    <div class="flex items-center gap-4">
        <button
            type="button"
            class="btn-icon variant-filled btn-icon-sm"
            on:click={onClose}
        >
            <AngleLeftSvg reverse />
        </button>
        <h1 class="h1 place-self-start">{data.name}</h1>
    </div>
</div>
