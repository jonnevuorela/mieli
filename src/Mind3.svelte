<script>
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api";

    let x1 = 0;
    let y1 = 0;
    let x2 = 0;
    let y2 = 0;
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
            thoughts.forEach((thought) => {
                if (thought.x === undefined) thought.x = 0;
                if (thought.y === undefined) thought.y = 0;
            });
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

        const activeThought = e.target.dataset.thoughtId;
        activeIndex = findThoughtIndexById(+activeThought);

        thoughts[activeIndex].x = e.target.offsetLeft;
        thoughts[activeIndex].y = e.target.offsetTop;

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

            // track position
            thoughts[activeIndex].el.style.left =
                `${mousePosition.x - mouseOffset.x}px`;
            thoughts[activeIndex].el.style.top =
                `${mousePosition.y - mouseOffset.y}px`;

            // update thoughts array
            thoughts[activeIndex].x = parseInt(
                thoughts[activeIndex].el.style.left,
            );
            thoughts[activeIndex].y = parseInt(
                thoughts[activeIndex].el.style.top,
            );

            const data = JSON.stringify(thoughts);
            await invoke("write_json", { data });
        }
    }
</script>

<div id="mind" role="main" on:mousemove={handleDragMove}>
    {#each thoughts as thought (thought.id)}
        <div
            role="application"
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
        {#if thought.relation_id}
            <svg
                style="position:absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none;"
            >
                <line
                    x1={thought.x + 50}
                    y1={thought.y + 50}
                    x2={thoughts.find((t) => t.id === thought.relation_id).x +
                        50}
                    y2={thoughts.find((t) => t.id === thought.relation_id).y +
                        50}
                    stroke="white"
                />
            </svg>
        {/if}
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
        background-color: #ff5733;
        z-index: 2;
    }
</style>
