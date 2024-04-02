<script lang="ts">
    import { AppBar, Avatar, LightSwitch } from "@skeletonlabs/skeleton";

    import AngleLeftSvg from "../icons/AngleLeftSvg.svelte";
    import ConfigSvg from "../icons/ConfigSvg.svelte";
    import HomeSvg from "../icons/HomeSvg.svelte";

    export let route: string[];

    function getRouteRp(idx: number): string {
        let nr: string[] = [];
        for (let i = 0; i <= idx; i++) {
            nr.push(route[i].toLowerCase());
        }

        return "/" + nr.join("/");
    }
</script>

<header class="hidden sm:block w-full">
    <AppBar
        gridColumns="grid-cols-3"
        slotDefault="place-self-center"
        slotTrail="place-content-end"
        padding="py-3 px-6"
    >
        <svelte:fragment slot="lead">
            <div class="flex flex-row items-center gap-8">
                <a href={getRouteRp(route.length - 2)}>
                    <button
                        type="button"
                        class="btn-icon variant-filled btn-icon-sm"
                    >
                        <AngleLeftSvg reverse />
                    </button>
                </a>

                <ol class="breadcrumb">
                    <li class="crumb hidden md:block">
                        <a class="anchor" href="/">
                            <HomeSvg />
                        </a>
                    </li>

                    {#each route as unit, i}
                        <li class="crumb-separator" aria-hidden>/</li>
                        {#if i == route.length - 1}
                            <li id="path-crumb-{i}">{unit}</li>
                        {:else}
                            <li class="crumb" id="path-crumb-{i}">
                                <a class="anchor" href={getRouteRp(i)}>{unit}</a
                                >
                            </li>
                        {/if}
                    {/each}
                </ol>
            </div>
        </svelte:fragment>

        <svelte:fragment slot="default">
            <a href="/"><h1 class="text-5xl">P+</h1></a>
        </svelte:fragment>

        <svelte:fragment slot="trail">
            <div class="flex flex-row items-center gap-8">
                <LightSwitch />
                <a href="/config">
                    <ConfigSvg />
                </a>
                <Avatar
                    initials="IR"
                    width="w-12"
                    border="border-2 border-surface-300-600-token hover:!border-primary-500"
                    rounded="rounded-full"
                />
            </div>
        </svelte:fragment>
    </AppBar>
</header>
