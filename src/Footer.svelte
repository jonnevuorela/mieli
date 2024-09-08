<script>
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    function input() {
        console.log("input");
        dispatch("input");
    }
</script>

<div class="footer">
    <button class="add" type="submit" on:click={input}>
        <svg width="16" height="16">
            <line x1="0" y1="8" x2="16" y2="8" stroke="white" />
            <line x1="8" y1="2" x2="8" y2="16" stroke="white" />
        </svg>
    </button>
</div>

<style>
    .footer {
        position: fixed;
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
        bottom: 0;
        background: rgb(0, 0, 0);
        background: radial-gradient(
            circle,
            rgba(0, 0, 0, 1) 20%,
            rgba(0, 0, 0, 0) 100%
        );
        width: 100%;
        height: 2.7em;
        z-index: 1000;
    }
</style>
