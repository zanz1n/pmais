<script lang="ts">
    import {
        AppRail,
        AppRailTile,
        AppRailAnchor,
    } from "@skeletonlabs/skeleton";

    import CalendarMonthSvg from "../icons/CalendarMonthSvg.svelte";
    import TableRowSvg from "../icons/TableRowSvg.svelte";
    import NewsPaperSvg from "../icons/NewsPaper.svelte";
    import ConfigSvg from "../icons/ConfigSvg.svelte";

    type Tiles = "agenda" | "grades" | "saep" | "redaction" | "user" | "config";

    export let route: Tiles;

    const svgSize = "w-8 h-8";
</script>

<aside>
    <AppRail background="bg-surface-backdrop-token">
        <AppRailAnchor
            href="/agenda"
            selected={route == "agenda"}
            title="Agenda"
        >
            <svelte:fragment slot="lead">
                <CalendarMonthSvg size={svgSize} />
            </svelte:fragment>
            <span>Agenda</span>
        </AppRailAnchor>

        <AppRailAnchor
            href="/grades"
            selected={route == "grades"}
            title="Notas"
        >
            <svelte:fragment slot="lead">
                <TableRowSvg size={svgSize} />
            </svelte:fragment>
            <span>Notas</span>
        </AppRailAnchor>

        <AppRailAnchor
            href="/saep"
            selected={route == "saep"}
            title="Simulados"
        >
            <svelte:fragment slot="lead">
                <NewsPaperSvg size={svgSize} />
            </svelte:fragment>
            <span>Simulados</span>
        </AppRailAnchor>

        <AppRailAnchor
            href="/redaction"
            selected={route == "redaction"}
            title="Redação"
        >
            <svelte:fragment slot="lead">
                <NewsPaperSvg size={svgSize} />
            </svelte:fragment>
            <span>Redação</span>
        </AppRailAnchor>

        <hr class="opacity-30" />

        <svelte:fragment slot="trail">
            <AppRailAnchor
                href="/config"
                selected={route == "config"}
                title="Configurações"
            >
                <svelte:fragment slot="lead">
                    <ConfigSvg size={svgSize} />
                </svelte:fragment>
                <span>Configs</span>
            </AppRailAnchor>
        </svelte:fragment>
    </AppRail>
</aside>
