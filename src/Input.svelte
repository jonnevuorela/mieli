<script>
    import { createEventDispatcher } from "svelte";
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

    function ok() {
        if (!relatedId) {
            relation_id = 0;
            x = 3035;
            y = 1940;
        } else {
            relation_id = relatedId;
        }
        dispatch("ok", { title, x, y, relation_id });
        console.log("ok (from Input)", title);
        cancel();
    }
</script>

<Mind3 on:add={handleAdd} />

<div class="inputWindow">
    <button class="inputCancel" on:click={cancel}>
        <p>X</p>
    </button>
    <input class="textInput" type="text" bind:value={title} />
    <button class="inputOk" on:click={ok}> OK </button>
</div>

<style>
    .inputWindow {
        position: fixed;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        background-color: rgba(20, 20, 20, 0.8);
        height: 6em;
        width: 20em;
        z-index: 3;
    }
    .inputCancel {
        position: absolute;
        top: 0;
        left: 0;
        border: none;
        width: 1.5em;
        height: 1.5em;
        font-size: 1em;
        color: white;
        background: transparent;
        padding-bottom: 1.5em;
    }
    .textInput {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        width: 80%;
        height: 1.5em;
        font-size: 1em;
        border: none;
    }
    .inputOk {
        position: absolute;
        bottom: 0;
        right: 0;
        border: none;
        height: 1.5em;
        font-size: 1em;
        color: white;
        background: transparent;
        padding-bottom: 1.5em;
    }
</style>
