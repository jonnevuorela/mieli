<script>
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api";

    let isDragging = false;
    let mouseOffset = { x: 0, y: 0 };
    let mousePosition = { x: 0, y: 0 };
    let thoughts = [];

    onMount(async () => {
        try {
            document.addEventListener("mouseup", function () {
                isDragging = false;
            });
            const response = await invoke("read_json");
            thoughts = JSON.parse(response);
            console.log(
                `thoughts in mind component from invoke:`,
                JSON.stringify(thoughts),
            );
        } catch (error) {
            console.log("Error invoking thoughts", error);
        }
    });

    function handleDragStart(e) {
        isDragging = true;
        mouseOffset = {
            x: e.clientX - e.target.getBoundingClientRect().left,
            y: e.clientY - e.target.getBoundingClientRect().top,
        };
    }
    function handleDragEnd() {
        isDragging = false;
    }

    function handleDragMove(e) {
        if (isDragging) {
            mousePosition = {
                x: e.clientX,
                y: e.clientY,
            };

            thoughts[0].el.style.left = `${mousePosition.x - mouseOffset.x}px`;
            thoughts[0].el.style.top = `${mousePosition.y - mouseOffset.y}px`;
        }
    }
</script>

<div id="mind" on:mousemove={handleDragMove}>
    {#each thoughts as thought (thought.id)}
        <div
            role="main"
            bind:this={thought.el}
            class="thought"
            style="left: {thought.x}px; top: {thought.y}px;"
            on:mousedown={handleDragStart}
            on:mouseup={handleDragEnd}
        >
            {thought.title}
            #{thought.id}
        </div>
    {/each}
</div>

<style>
    #mind {
        position: relative;
        background-color: black;
        width: 100vh;
        height: 50vh;
    }

    .thought {
        position: absolute;
        width: 100px;
        height: 100px;
        background-color: #ff5733; /* some color for the thoughts */
        cursor: move;
    }
</style>
