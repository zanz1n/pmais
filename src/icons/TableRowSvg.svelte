<script lang="ts">
    import Svg from "../components/Svg.svelte";

    export let reverse: boolean | undefined = undefined;
    export let size: string | undefined = undefined;
</script>

<Svg {size} {reverse} fill="currentColor">
    <path
        fill-rule="evenodd"
        d="M2 6a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V6Zm2 8v-2h7v2H4Zm0 2v2h7v-2H4Zm9 2h7v-2h-7v2Zm7-4v-2h-7v2h7Z"
        clip-rule="evenodd"
    />
</Svg>
