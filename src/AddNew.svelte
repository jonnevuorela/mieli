<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import Input from "./Input.svelte";
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    export let updateMind;
    export let handledRelated;

    let title = "";
    let x = 4200;
    let y = 4200;

    let jsonOutput;

    let lastId;
    let thoughtId;

    //Fetch last id from backend
    async function fetchLastId() {
        try {
            lastId = await invoke("get_last_id");
        } catch (error) {
            console.error("Error fetching last id", error);
        }
    }
    //incremention
    async function nextId() {
        await fetchLastId();
        if (lastId === undefined) {
            thoughtId = 1;
        } else {
            thoughtId = lastId + 1;
        }
    }

    $: console.log("handledRelated", handledRelated);

    async function okButtonClick(e) {
        console.log("handledRelated from okbuttonclick", handledRelated);
        console.log("add event detail (from AddNew)");

        console.log("add event detail (from AddNew)", e.detail);
        console.log("title (from AddNew)", e.detail.title);

        console.log("relatedId (from AddNew)", handledRelated.id);
        console.log(
            "related coordinates (from AddNew)",
            handledRelated.x,
            handledRelated.y,
        );

        await nextId();

        jsonOutput = JSON.stringify([
            {
                title: e.detail.title,
                id: thoughtId,
                x: handledRelated.x + 100 ? x : x,
                y: handledRelated.y + 100 ? y : y,
                relation_id: handledRelated.id ? handledRelated.id : 0,
            },
        ]);

        //Pass new data to backend
        await invoke("write_json", { data: jsonOutput })
            .then(() => {
                console.log("Data passed to backend");
            })
            .catch((error) => {
                console.error("Error writing data", error);
            });
        updateMind();
    }
</script>

<Input on:ok={okButtonClick} on:cancel={() => dispatch("cancel")} />
