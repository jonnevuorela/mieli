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

<div id="menu" transition:fly={{ y: 0, x: -200, duration: 500 }}>
    <nav>
        <a href="#">Menu Item 1</a>
        <a href="#">Menu Item 2</a>
        <a on:click={() => clearData()} href="#">Clear data</a>
    </nav>
</div>

<style>
    #menu {
        position: fixed;
        display: flex;
        flex-direction: column;
        top: 2.7em;
        left: 0;
        background-color: midnightblue;
        border-right: 1px groove black;
        width: 10em;
        height: 100vh;
        z-index: 1000;
    }
    nav {
        display: flex;
        flex-direction: column;
        padding-top: 1em;
        gap: 1em;
        align-items: center;
        height: 100%;
    }
    a {
        color: white;
        text-decoration: none;
        font-size: 1.5em;
    }
</style>
