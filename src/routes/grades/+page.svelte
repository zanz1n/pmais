<script lang="ts">
    import { Drawer, getDrawerStore } from "@skeletonlabs/skeleton";
    import type { DrawerSettings } from "@skeletonlabs/skeleton";
    import type { ID, Subject } from "$lib/grades/grades";
    import fakegradesdata from "$lib/grades/fakegradesdata";

    import GradesTable from "./GradesTable.svelte";
    import DrawerContent from "./DrawerContent.svelte";

    const drawerStore = getDrawerStore();
<<<<<<< Updated upstream

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
=======
    const drawerSettings: DrawerSettings = {
        id: 'example-3',
        // Provide your property overrides:
        bgDrawer: 'bg-purple-900 text-white',
        bgBackdrop: 'bg-gradient-to-tr from-indigo-500/50 via-purple-500/50 to-pink-500/50',
        width: 'md:w-[480px] w-full',
        padding: 'p-4',
        // Remove the 'margin' property
        rounded: 'rounded-xl',
    };
    
    function openM (){
>>>>>>> Stashed changes
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
