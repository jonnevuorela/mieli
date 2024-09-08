<script>
    import { createEventDispatcher } from "svelte";
    const dispatch = createEventDispatcher();

    function toggleMenu() {
        dispatch("toggleMenu");
        console.log("toggleMenu dispatched");
    }
</script>

<div class="header">
    <button on:click={toggleMenu} class="menuButton">
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            fill="currentColor"
        >
            <path
                d="M2.5 11.5A.5.5 0 0 0 3 12h10a.5.5 0 0 0 0-1H3a.5.5 0 0 0-.5.5zm0-4A.5.5 0 0 0 3 8h10a.5.5 0 0 0 0-1H3a.5.5 0 0 0-.5.5zm0-4A.5.5 0 0 0 3 4h10a.5.5 0 0 0 0-1H3a.5.5 0 0 0-.5.5z"
            />
        </svg>
    </button>
    <button class="settingsButton">
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            fill="currentColor"
            viewBox="0 0 16 16"
        >
            <path
                d="M14.59 9.535a1 1 0 0 1 .364 1.366l-1.036 1.794a1 1 0 0 1-1.342.488l-1.927-.703a3.99 3.99 0 0 1-1.735.991l-.286 2.054a1 1 0 0 1-.97.875H6.354a1 1 0 0 1-.97-.875l-.286-2.054a3.99 3.99 0 0 1-1.735-.991l-1.927.703a1 1 0 0 1-1.342-.488L1.046 10.9a1 1 0 0 1 .364-1.366l1.794-1.036a3.99 3.99 0 0 1 0-1.998L1.41 6.465a1 1 0 0 1-.364-1.366l1.036-1.794a1 1 0 0 1 1.342-.488l1.927.703a3.99 3.99 0 0 1 1.735-.991l.286-2.054a1 1 0 0 1 .97-.875h2.132a1 1 0 0 1 .97.875l.286 2.054a3.99 3.99 0 0 1 1.735.991l1.927-.703a1 1 0 0 1 1.342.488l1.036 1.794a1 1 0 0 1-.364 1.366l-1.794 1.036a3.99 3.99 0 0 1 0 1.998l1.794 1.036zM8 11a3 3 0 1 0 0-6 3 3 0 0 0 0 6z"
            />
        </svg>
    </button>
</div>

<style>
    .header {
        position: fixed;
        display: flex;
        flex-direction: row;
        top: 0;
        background-color: black;
        width: 100%;
        height: 2.7em;
        z-index: 1000;
    }

    .header::after {
        content: "";
        display: table;
        clear: both;
    }

    .menuButton {
        position: absolute;
        left: 0;
        border: none;
        color: white;
        padding: 10px 20px;
        text-align: center;
        text-decoration: none;
        display: inline-block;
        font-size: 16px;
        margin: 4px 2px;
        cursor: pointer;
    }

    .settingsButton {
        position: absolute;
        right: 0;
        border: none;
        color: white;
        padding: 10px 20px;
        text-align: center;
        text-decoration: none;
        display: inline-block;
        font-size: 16px;
        margin: 4px 2px;
        cursor: pointer;
    }
</style>
