<script lang="ts">
    import { GradesColor } from "$lib/grades/grades";

    const c = {
        [GradesColor.Black]: "",
        [GradesColor.Green]: "text-green-700 dark:text-green-500",
        [GradesColor.Red]: "text-red-700 dark:text-red-500",
    };

    export let tClasses: string;
    export let last: boolean = false;
    export let color: GradesColor = GradesColor.Black;

    $: colorClass = c[color];
    $: clss = `${last ? "" : "hidden md:table-cell"} font-bold ${colorClass} ${tClasses}`;
</script>

<td class={clss}>
    <slot />
</td>
