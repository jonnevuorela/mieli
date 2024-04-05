<script>
    import Menu from "./Menu.svelte";
    import AddNew from "./AddNew.svelte";
    import Mind3 from "./Mind3.svelte";
    import Header from "./Header.svelte";
    import Footer from "./Footer.svelte";

    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    let showMenu = false;
    let inputWindow = false;
</script>

<main id="main" class="container">
    <Header on:toggleMenu={() => (showMenu = !showMenu)} />

    {#if showMenu}
        <Menu />
    {/if}

    {#if inputWindow}
        <AddNew on:cancel={() => (inputWindow = !inputWindow)} />
    {/if}

    <Mind3
        on:add={(e) => {
            dispatch("addRelated", {
                relatedId: e.detail.relatedId,
                coordinates: e.detail.coordinates,
            });
            inputWindow = !inputWindow;
        }}
    />

    <Footer on:input={() => (inputWindow = !inputWindow)} />
</main>

<style>
    .container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        height: 100vh;
        margin: 0;
        padding: 0;
    }
</style>
