<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import App from "./App.svelte";
    import Input from "./Input.svelte";
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();
    export let updateMind;

    let title = "";
    let x = 3035;
    let y = 1940;

    let jsonOutput;

    let lastId;
    let thoughtId;

    let relatedId;
    let coordinates;

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

    function handleAdd(e) {
        relatedId = e.detail.relatedId;
        coordinates = e.detail.coordinates;
    }

    async function okButtonClick(e) {
        console.log("add event detail (from AddNew)", e.detail);
        console.log("title (from AddNew)", e.detail.title);
        console.log("relatedId (from AddNew)", relatedId);
        console.log("coordinates (from AddNew)", coordinates);

        await nextId();

        jsonOutput = JSON.stringify([
            {
                title: e.detail.title,
                id: thoughtId,
                x: coordinates ? coordinates.x : x,
                y: coordinates ? coordinates.y : y,
                relation_id: relatedId ? relatedId : 0,
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

<App on:addRelated={handleAdd} />
<Input on:ok={okButtonClick} on:cancel={() => dispatch("cancel")} />
