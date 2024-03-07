<script>
    import { invoke } from "@tauri-apps/api/tauri"
    //const { invoke } = window.__TAURI__.tauri 

    const passiveMode = 'passive';
    const inputMode = 'text_area_visible';
    let currentState = passiveMode;

    let title = "";
    let jsonOutput;

    let lastId;
    let thoughtId;

    //Fetch last id from backend
    async function fetchLastId(){
        try{
            lastId = await invoke('get_last_id');
            console.log(`lastId: ${lastId}`);
        }catch(error){
            console.error('Error fetching last id', error);
        }
    };
    //incremention
    async function nextId(){
        await fetchLastId();
        console.log(`lastId in nextId(): ${lastId}`);
        if (lastId === undefined){
            thoughtId = 1;
        } else {
            thoughtId = lastId + 1
        }
    };

    

    function addButtonClick(){
        console.log("addButtonClick");
        currentState = inputMode;
        
        };
        async function okButtonClick(){
            console.log("okButtonClick");
            currentState = passiveMode;
            await nextId();
            jsonOutput = JSON.stringify([{
                title,
                id: thoughtId
            }]);

        //Pass new data to backend
        await invoke("write_json", { data: jsonOutput })
        .then(() => {
            console.log("Data passed to backend");
        })
        .catch((error) => {
            console.error("Error writing data", error);
        });

            let parsedJson = JSON.parse(jsonOutput);
            console.log('parsedJson');
            console.log(parsedJson);
    };




</script>

{#if currentState === inputMode}
    <div id="inputWindow">
        <input type="text" bind:value={title}/>
        <button on:click={okButtonClick}>
            OK
        </button>
    </div>
{/if}
<div >
    <button type="submit" on:click={addButtonClick}>
        Add a tought
    </button>
</div>

<pre>{JSON.stringify(jsonOutput, null, 2)}</pre>
<div>
    {#if lastId > 0}
    <p>LastId: {lastId}</p>
    {/if}
    <button type="submit" id="debug" on:click={fetchLastId}>
        Fetch last id
    </button>
</div>


