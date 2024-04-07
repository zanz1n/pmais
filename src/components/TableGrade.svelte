<script>
    export const tableArr = [
        { name: "Matemática A", mediafinal: 9.5, medsala: 5.6, qualif: 5, tri1: 8, tri2: 6.7, tri3: 7 },
        { name: "Matemática B", mediafinal: 8.7, medsala: 3.5, qualif: 17, tri1: 8, tri2: 6.7, tri3: 8 },
        { name: "Matemática C", mediafinal: 7.6, medsala: 4.5, qualif: 49, tri1: 7.8, tri2: 8.7, tri3: 9 },
        { name: "Gramática", mediafinal: 8.5, medsala: 6.5, qualif: 5, tri1: 9, tri2: 8.7, tri3: 9.5 }
    ]; 
</script>

<div class="table-container">
    <!-- Native Table Element -->
    <table class="table table-hover ">
        <thead>
            <tr>
                <th>Matéria</th>
                <th>Méd Aluno</th>
                <th>Méd Sala</th>
                <th>Qualif</th>
                <th>1° Trimestre</th>
                {#if tableArr[0].tri2 != null}
                    <th>2° Trimestre</th>
                {/if}
                {#if tableArr[0].tri3 != null}
                    <th>3° Trimestre</th>
                {/if}
            </tr>
        </thead>
        <tbody>
            {#each tableArr as row, i}
                <tr>
                    <td>{row.name}</td>
                    <td>{row.mediafinal}</td>
                    <td>{row.medsala}</td>
                    <td>{row.qualif}</td>
                    <td>{row.tri1}</td>
                    {#if row.tri2 != null}
                        <td>{row.tri2}</td>
                    {/if}
                    {#if row.tri3 != null}
                        <td>{row.tri3}</td>
                    {/if}
                </tr>
            {/each}
        </tbody>
        <tfoot>
            <tr>
            </tr>
        </tfoot>
    </table>
</div>
