<script>
    import { fly } from "svelte/transition";
</script>

<div id="menu" transition:fly={{ y: 0, x: -200, duration: 500 }}>
    <nav>
        <a href="#">Menu Item 1</a>
        <a href="#">Menu Item 2</a>
        <a href="#">Menu Item 3</a>
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
