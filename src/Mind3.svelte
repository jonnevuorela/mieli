<script>
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api";
    import AddNew from "./AddNew.svelte";

    const passiveMode = "passive";
    const inputMode = "text_area_visible";
    let currentState = passiveMode;
    let title = "";
    let isDragging = false;
    let mouseOffset = { x: 0, y: 0 };
    let mousePosition = { x: 0, y: 0 };
    let thoughts = [];
    let selectedThought = null;
    let activeIndex = null;
    let lastId;

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
        const activeThought = e.target.dataset.thoughtId;
        activeIndex = findThoughtIndexById(+activeThought);

        if (thoughts[activeIndex] != null) {
            isDragging = true;

            thoughts[activeIndex].x = e.target.offsetLeft;
            thoughts[activeIndex].y = e.target.offsetTop;

            mouseOffset = {
                x: e.clientX - e.target.getBoundingClientRect().left,
                y: e.clientY - e.target.getBoundingClientRect().top,
            };
        }
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
    function handleClick(e) {
        console.log("addButtonClick");
        currentState = inputMode;

        selectedThought = thoughts.find(
            (thought) =>
                thought.id === +e.target.parentElement.dataset.thoughtId,
        );
    }

    async function handleClickOk(e) {
        console.log("okButtonClick");
        currentState = passiveMode;

        if (!selectedThought) {
            console.error("No thought selected");
            return;
        }
        console.log("selectedThought.id", selectedThought.id);

        const lastId = await invoke("get_last_id");
        const newId = lastId + 1;

        const jsonOutput = JSON.stringify([
            {
                title,
                id: newId,
                x: selectedThought.x,
                y: selectedThought.y + 100,
                relation_id: selectedThought.id,
            },
        ]);

        await invoke("write_json", { data: jsonOutput })
            .then(() => {
                console.log("Data passed to backend");
            })
            .catch((error) => {
                console.error("Error writing data", error);
            });
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
            <button on:click={handleClick}> + </button>
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
    {#if currentState === inputMode}
        <div id="inputWindow">
            <input type="text" bind:value={title} />
            <button on:click={handleClickOk}> OK </button>
        </div>
    {/if}
</div>

<style>
    #inputWindow {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        background-color: #ff5733;
        padding: 10px;
        z-index: 3;
    }
    #mind {
        position: relative;
        background-color: black;
        width: 150vh;
        height: 80vh;
    }

    .thought {
        position: absolute;
        width: 100px;
        height: 100px;
        background-color: #ff5733;
        z-index: 2;
    }
</style>
