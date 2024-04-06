<script lang="ts">
    export let reverse = false;
    export let size = "w-6 h-6";
    export let fill = "currentColor";

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
    {fill}
    viewBox="0 0 24 24"
>
    <slot />
</svg>
