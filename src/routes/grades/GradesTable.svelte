<script lang="ts">
    import { type GradesData, type ID, type Period } from "$lib/grades/grades";
    import TableGradeTd from "./TableGradeTd.svelte";

    function getLastPeriodId(periods: Period[]): ID {
        if (0 >= periods.length) {
            return 0;
        }
        return periods[periods.length - 1].id;
    }

    const tClasses = "text-center";

    export let data: GradesData;
    export let onClickSubject: (id: ID) => void;

    $: lastPeriodId = getLastPeriodId(data.periods);
</script>

<div class="table-container max-w-[960px]">
    <table class="table table-hover table-interactive">
        <thead>
            <tr>
                <th class="text-left">Matéria</th>
                <th class={tClasses}>Méd. Aluno</th>
                <th class={tClasses}>Méd. Sala</th>
                <th class={tClasses}>Classif.</th>
                {#each data.periods as period}
                    {#if period.id == lastPeriodId}
                        <th class={tClasses}>
                            <div class="hidden lg:block">
                                {period.name}
                            </div>
                            <div class="lg:hidden">
                                {period.name.substring(0, 6) + "."}
                            </div>
                        </th>
                    {:else}
                        <th class="hidden md:table-cell {tClasses}">
                            <div class="hidden lg:block">
                                {period.name}
                            </div>
                            <div class="lg:hidden">
                                {period.name.substring(0, 6) + "."}
                            </div>
                        </th>
                    {/if}
                {/each}
            </tr>
        </thead>
        <tbody>
            {#each data.subjects as subject}
                <tr
                    on:click={function () {
                        onClickSubject(subject.id);
                    }}
                >
                    {#if subject.final}
                        <td class="text-left font-bold">{subject.name}</td>
                    {:else}
                        <td class="text-left">{subject.name}</td>
                    {/if}

                    <td class={tClasses}>{subject.studentAverage}</td>
                    <td class={tClasses}>{subject.classAverage}</td>
                    <td class={tClasses}>{subject.classification}</td>

                    {#each subject.periods as { id, color, studentAverage }}
                        <TableGradeTd
                            last={id == lastPeriodId}
                            {tClasses}
                            {color}>{studentAverage}</TableGradeTd
                        >
                    {/each}
                </tr>
            {/each}
        </tbody>
        <tfoot></tfoot>
    </table>
</div>
