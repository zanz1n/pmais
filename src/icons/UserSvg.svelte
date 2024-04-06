<script lang="ts">
    import Svg from "../components/Svg.svelte";

    export let reverse: boolean | undefined;
    export let size: string | undefined;
</script>

<Svg {size} {reverse} fill="currentColor">
    <path
        fill-rule="evenodd"
        d="M12 4a4 4 0 1 0 0 8 4 4 0 0 0 0-8Zm-2 9a4 4 0 0 0-4 4v1a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2v-1a4 4 0 0 0-4-4h-4Z"
        clip-rule="evenodd"
    />
</Svg>
