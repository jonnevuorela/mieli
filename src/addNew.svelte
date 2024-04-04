<script>
    import { invoke } from "@tauri-apps/api/tauri";

    const passiveMode = "passive";
    const inputMode = "text_area_visible";
    let currentState = passiveMode;

    let title = "";
    let x = 4800;
    let y = 2350;

    let jsonOutput;

    let lastId;
    let thoughtId;

    //Fetch last id from backend
    async function fetchLastId() {
        try {
            lastId = await invoke("get_last_id");
            console.log(`lastId: ${lastId}`);
        } catch (error) {
            console.error("Error fetching last id", error);
        }
    }
    //incremention
    async function nextId() {
        await fetchLastId();
        console.log(`lastId in nextId(): ${lastId}`);
        if (lastId === undefined) {
            thoughtId = 1;
        } else {
            thoughtId = lastId + 1;
        }
    }

    function addButtonClick() {
        console.log("addButtonClick");
        currentState = inputMode;
    }
    async function okButtonClick() {
        console.log("okButtonClick");
        currentState = passiveMode;
        await nextId();

        jsonOutput = JSON.stringify([
            {
                title,
                id: thoughtId,
                x,
                y,
                relation_id: 0,
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

        let parsedJson = JSON.parse(jsonOutput);
        console.log("parsedJson");
        console.log(parsedJson);
    }
</script>
<AddNew
    bind:title
    bind:x
    bind:y
    bind:currentState
    on:ok={okButtonClick}
    on:cancel={() => (currentState = passiveMode)}
