<script lang="ts">
    import Svg from "../components/Svg.svelte";

    export let reverse: boolean | undefined;
    export let size: string | undefined;
</script>

<Svg {size} {reverse} fill="currentColor">
    <path
        fill-rule="evenodd"
        d="M11.293 3.293a1 1 0 0 1 1.414 0l6 6 2 2a1 1 0 0 1-1.414 1.414L19 12.414V19a2 2 0 0 1-2 2h-3a1 1 0 0 1-1-1v-3h-2v3a1 1 0 0 1-1 1H7a2 2 0 0 1-2-2v-6.586l-.293.293a1 1 0 0 1-1.414-1.414l2-2 6-6Z"
        clip-rule="evenodd"
    />
</Svg>
