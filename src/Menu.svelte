<script>
    import { fly } from "svelte/transition";
    import { invoke } from "@tauri-apps/api/tauri";
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    function clearData() {
        invoke("delete_data");
        dispatch("clearDataEvent");
        console.log("clear data");
    }
</script>

<div class="menu" transition:fly={{ y: 0, x: -200, duration: 500 }}>
    <nav>
        <a on:click={() => clearData()} href="#">Clear data</a>
    </nav>
</div>

<style>
    .menu {
        position: fixed;
        display: flex;
        flex-direction: column;
        top: 2.7em;
        left: 0;
        background: rgb(0, 0, 0);
        background: linear-gradient(
            90deg,
            rgba(0, 0, 0, 1) 75%,
            rgba(0, 0, 0, 0) 100%
        );
        width: 20%;
        height: 100vh;
        z-index: 1000;
    }
    nav {
        display: flex;
        flex-direction: column;
        padding-top: 1em;
        padding-left: 1em;
        gap: 1em;
        height: 100%;
    }
    a {
        color: white;
        text-decoration: none;
        font-size: 1.5em;
    }
</style>
