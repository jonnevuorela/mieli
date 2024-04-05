<script>
    import { createEventDispatcher } from "svelte";
    import AddNew from "./AddNew.svelte";
    import Mind3 from "./Mind3.svelte";
    const dispatch = createEventDispatcher();

    let title = "";
    let x;
    let y;
    let relation_id;

    let relatedId;

    function handleAdd(e) {
        console.log("add event detail (from Input)", e.detail);
        relatedId = e.detail.relatedId;
        x = e.detail.coordinates.x;
        y = e.detail.coordinates.y;
    }

    function cancel() {
        dispatch("cancel");
    }

    function ok(e) {
        if (!relatedId) {
            relation_id = 0;
            x = 3035;
            y = 1940;
        } else {
            relation_id = relatedId;
        }
        dispatch("ok", { title, x, y, relation_id });
        console.log("ok (from Input)", title);
    }
</script>

<Mind3 on:add={handleAdd} />

<div id="inputWindow">
    <button id="inputCancel" on:click={cancel}>
        <p>Xdd</p>
        <svg width="8" height="8" fill="currentColor">
            // cross for cancelling

            <path d="M0 0 L8 8 M8 0 L0 8" stroke="white" stroke-width="2" />
        </svg>
    </button>
    <input id="textInput" type="text" bind:value={title} />
    <button id="inputOk" on:click={ok}> OK </button>
</div>

<style>
    #inputWindow {
        position: fixed;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        background-color: midnightblue;
        border: 0.1em groove black;
        border-radius: 0.5em;
        height: 6em;
        padding: 10px;
        z-index: 3;
    }
</style>
