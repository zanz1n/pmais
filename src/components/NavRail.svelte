<script lang="ts">
    import RailAnchor from "./RailAnchor.svelte";

    import CalendarMonthSvg from "../icons/CalendarMonthSvg.svelte";
    import TableRowSvg from "../icons/TableRowSvg.svelte";
    import NewsPaperSvg from "../icons/NewsPaper.svelte";
    import ConfigSvg from "../icons/ConfigSvg.svelte";
    import UserSvg from "../icons/UserSvg.svelte";

    type Tiles = "agenda" | "grades" | "saep" | "redaction" | "user" | "config";

    export let route: Tiles;

    const svgSize = "w-8 h-8";
    const railBase = "bg-surface-backdrop-token gap-0";
    const railFlex = "flex flex-row sm:flex-col sm:justify-between";
    const railOverflow =
        "sm:overflow-y-auto overflow-y-hidden sm:overflow-x-hidden overflow-x-auto";
    const railSize = "sm:w-20 h-20 sm:h-full w-full";
    const regionFlex = "flex flex-row sm:flex-col";
</script>

<aside>
    <div class="{railBase} {railOverflow} {railSize} {railFlex}">
        <div class={regionFlex}>
            <RailAnchor
                href="/agenda"
                selected={route == "agenda"}
                title="Agenda"
            >
                <svelte:fragment slot="lead">
                    <CalendarMonthSvg size={svgSize} />
                </svelte:fragment>
                <span>Agenda</span>
            </RailAnchor>

            <RailAnchor
                href="/grades"
                selected={route == "grades"}
                title="Notas"
            >
                <svelte:fragment slot="lead">
                    <TableRowSvg size={svgSize} />
                </svelte:fragment>
                <span>Notas</span>
            </RailAnchor>

            <RailAnchor
                href="/saep"
                selected={route == "saep"}
                title="Simulados"
            >
                <svelte:fragment slot="lead">
                    <NewsPaperSvg size={svgSize} />
                </svelte:fragment>
                <span>Simulados</span>
            </RailAnchor>

            <RailAnchor
                href="/redaction"
                selected={route == "redaction"}
                title="Redação"
            >
                <svelte:fragment slot="lead">
                    <NewsPaperSvg size={svgSize} />
                </svelte:fragment>
                <span>Redação</span>
            </RailAnchor>

            <hr class="opacity-60 sm:block hidden" />
        </div>

        <div class={regionFlex}>
            <RailAnchor href="/user" selected={route == "user"} title="Usuário">
                <svelte:fragment slot="lead">
                    <UserSvg size={svgSize} />
                </svelte:fragment>
                <span>Usuário</span>
            </RailAnchor>

            <RailAnchor
                href="/config"
                selected={route == "config"}
                title="Configurações"
            >
                <svelte:fragment slot="lead">
                    <ConfigSvg size={svgSize} />
                </svelte:fragment>
                <span>Configs</span>
            </RailAnchor>
        </div>
    </div>
</aside>
