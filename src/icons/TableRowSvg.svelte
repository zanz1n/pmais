<script lang="ts">
    export let reverse = false;
    export let size = "w-6 h-6";

    let color = "text-gray-800 dark:text-white";

    if (reverse) {
        color = "text-white dark:text-gray-800";
    }
</script>

<svg
    class="svg-icon inline-block outline-none {size} {color}"
    aria-hidden="true"
    xmlns="http://www.w3.org/2000/svg"
    width="24"
    height="24"
    fill="currentColor"
    viewBox="0 0 24 24"
>
    <path
        fill-rule="evenodd"
        d="M2 6a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V6Zm2 8v-2h7v2H4Zm0 2v2h7v-2H4Zm9 2h7v-2h-7v2Zm7-4v-2h-7v2h7Z"
        clip-rule="evenodd"
    />
</svg>
