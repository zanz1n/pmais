<script lang="ts">
    import Svg from "../components/Svg.svelte";

    export let reverse: boolean | undefined = undefined;
    export let size: string | undefined = undefined;
</script>

<Svg {size} {reverse} fill="none">
    <path
        stroke="currentColor"
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
        d="m15 19-7-7 7-7"
    />
</Svg>
