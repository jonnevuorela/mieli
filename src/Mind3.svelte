<script>
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api";

    let isDragging = false;
    let mouseOffset = { x: 0, y: 0 };
    let mousePosition = { x: 0, y: 0 };
    let thoughts = [];
    let activeThought;
    // return index of active thought for handleDragMove()
    let activeIndex = null;

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
    function findThoughtIndexById(activeThought) {
        return thoughts.findIndex((thought) => thought.id === activeThought);
    }

    async function handleDragStart(e) {
        isDragging = true;

        const activeThought = event.target.dataset.thoughtId;
        activeIndex = findThoughtIndexById(+activeThought);

        mouseOffset = {
            x: e.clientX - e.target.getBoundingClientRect().left,
            y: e.clientY - e.target.getBoundingClientRect().top,
        };
    }
    function handleDragEnd() {
        isDragging = false;
    }

    async function handleDragMove(e) {
        if (isDragging) {
            mousePosition = {
                x: e.clientX,
                y: e.clientY,
            };

            thoughts[activeIndex].el.style.left =
                `${mousePosition.x - mouseOffset.x}px`;
            thoughts[activeIndex].el.style.top =
                `${mousePosition.y - mouseOffset.y}px`;
        }
    }
</script>

<div id="mind" role="main" on:mousemove={handleDragMove}>
    {#each thoughts as thought (thought.id)}
        <div
            role="main"
            bind:this={thought.el}
            class="thought"
            style="left: {thought.x}px; top: {thought.y}px;"
            on:mousedown={handleDragStart}
            on:mouseup={handleDragEnd}
            data-thought-id={thought.id}
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
    }
</style>
