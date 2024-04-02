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
    fill="none"
    viewBox="0 0 24 24"
>
    <path
        stroke="currentColor"
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
        d="m15 19-7-7 7-7"
    />
</svg>
