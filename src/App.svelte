<script>
    import Menu from "./Menu.svelte";
    import Input from "./Input.svelte";
    import AddNew from "./AddNew.svelte";
    import Mind3 from "./Mind3.svelte";
    import Header from "./Header.svelte";
    import Footer from "./Footer.svelte";

    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    let showMenu = false;
    let inputWindow = false;

    function passInput(e) {
        console.log("passInput", e.detail.title);
        dispatch("input", { title: e.detail.title });
    }
</script>

<main id="main" class="container">
    <Header on:toggleMenu={() => (showMenu = !showMenu)} />

    {#if showMenu}
        <Menu />
    {/if}

    {#if inputWindow}
        <Input
            on:cancel={() => (inputWindow = !inputWindow)}
            on:ok={passInput}
        />
    {/if}

    <Mind3 />

    <Footer on:addButtonClick={() => (inputWindow = !inputWindow)} />
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
