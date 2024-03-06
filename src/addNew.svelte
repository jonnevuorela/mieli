<script>
    import { invoke } from "@tauri-apps/api/tauri"
    //const { invoke } = window.__TAURI__.tauri 

    const passiveMode = 'passive';
    const inputMode = 'text_area_visible';

    let currentState = passiveMode;
    let thoughtId = 1;
    
    let title = "";
    let jsonOutput;


    function addButtonClick(){
        console.log("addButtonClick");
        currentState = inputMode;
    
    };
    function okButtonClick(){
        console.log("okButtonClick");
        currentState = passiveMode;
        jsonOutput = JSON.stringify([{
            title,
            id: thoughtId++,
        }]);

        invoke('write_json', {data: jsonOutput}).then(()=>{
            console.log('data passed to backend');
        }).catch((error)=>{
            console.error('Error writing data', error);
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
    <button style="border-radius: 5px" type="submit" on:click={addButtonClick}>
        Add a tought
    </button>
</div>

<pre>{JSON.stringify(jsonOutput, null, 2)}</pre>


