<script lang="ts">
    import { Drawer, getDrawerStore } from "@skeletonlabs/skeleton";
    import type { DrawerSettings } from "@skeletonlabs/skeleton";
    import type { ID, Subject } from "$lib/grades/grades";
    import fakegradesdata from "$lib/grades/fakegradesdata";

    import GradesTable from "./GradesTable.svelte";
    import DrawerContent from "./DrawerContent.svelte";

    const drawerStore = getDrawerStore();

    let drawerSettings: DrawerSettings = {
        id: "subject-drawer",
        bgBackdrop:
            "bg-gradient-to-tr from-gray-500/50 via-gray-500/50 to-gray-500/50",
        width: "md:w-[520px] w-full",
        padding: "p-4",
        rounded: "rounded-xl",
        position: "right",
    } satisfies DrawerSettings;

    function getSubjectById(id: ID, subjects: Subject[]): Subject | null {
        return (
            subjects.find(function (s) {
                return s.id == id;
            }) ?? null
        );
    }

    function openDrawer(id: ID) {
        const sub = getSubjectById(id, fakegradesdata.subjects);
        if (!sub) {
            return;
        }
        console.log(id);

        drawerSettings.meta = sub;
        drawerStore.open(drawerSettings);
    }

    function closeDrawer() {
        drawerStore.close();
    }
</script>

<Drawer>
    <DrawerContent data={$drawerStore.meta} onClose={closeDrawer} />
</Drawer>

<div class="flex flex-col items-center p-6 w-full">
    <GradesTable onClickSubject={openDrawer} data={fakegradesdata} />
</div>
