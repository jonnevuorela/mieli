<script>
    import { createEventDispatcher } from "svelte";
    const dispatch = createEventDispatcher();

    let title = "";

    function cancel() {
        dispatch("cancel");
    }
    function ok() {
        dispatch("ok", { title });
    }
</script>

<div id="inputWindow">
    <button id="inputCancel" on:click={cancel}>
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
