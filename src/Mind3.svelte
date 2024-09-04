<script>
    import { onMount } from "svelte";
    import { event, invoke } from "@tauri-apps/api";
    import { createEventDispatcher } from "svelte";
    export let thoughts = [];

    const dispatch = createEventDispatcher();

    const passiveMode = "passive";
    const inputMode = "text_area_visible";
    let currentState = passiveMode;

    let isPanning = false;
    let startPanPosition = { x: 0, y: 0 };

    let isDragging = false;
    let mouseOffset = { x: 0, y: 0 };
    let mousePosition = { x: 0, y: 0 };

    let isConnecting = false;
    let connectingThought = null;
    let trackX = 0;
    let trackY = 0;

    let activeIndex = null;

    let mind;
    let scale = 0.5;
    let mouseX = 0;
    let mouseY = 0;

    let targetTranslate = { x: 0, y: 0 };

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
        mind.style.transform = `scale(${scale})`;
    });
    function findThoughtIndexById(activeThought) {
        return thoughts.findIndex((thought) => thought.id === activeThought);
    }

    function checkBorders() {
        const mindRect = mind.getBoundingClientRect();
        const viewportWidth = window.innerWidth;
        const viewportHeight = window.innerHeight;

        if (mindRect.left > 0) {
            targetTranslate.x -= mindRect.left;
        }
        if (mindRect.right < viewportWidth) {
            targetTranslate.x += viewportWidth - mindRect.right;
        }
        if (mindRect.top > 0) {
            targetTranslate.y -= mindRect.top;
        }
        if (mindRect.bottom < viewportHeight) {
            targetTranslate.y += viewportHeight - mindRect.bottom;
        }
    }
    function handleWheel(e) {
        // disable scrolling
        e.preventDefault();
    }
    function handleMouseDown(e) {
        e.preventDefault();
        if (e.target.classList.contains("thought")) {
            handleDragStart(e);
        } else {
            isPanning = true;
            startPanPosition = {
                x: e.clientX,
                y: e.clientY,
            };
        }
    }

    function handleMouseMove(e) {
        //for dragging
        const adjustedMousePosition = {
            x: e.clientX / scale,
            y: e.clientY / scale,
        };
        const svgRect = mind.getBoundingClientRect();
        trackX = (e.clientX - svgRect.left) / scale;
        trackY = (e.clientY - svgRect.top) / scale;

        if (isPanning) {
            const dx = e.clientX - startPanPosition.x;
            const dy = e.clientY - startPanPosition.y;
            startPanPosition = { x: e.clientX, y: e.clientY };
            panMind(dx, dy);
        } else if (isDragging) {
            handleDragMove(adjustedMousePosition);
        } else if (isConnecting) {
            // Ensure line updates
            trackX = trackX;
            trackY = trackY;
        }
    }

    function handleMouseUp() {
        isPanning = false;
    }

    function panMind(dx, dy) {
        const mind = document.getElementById("mind");
        const style = window.getComputedStyle(mind);
        const matrix = new DOMMatrix(style.transform);
        targetTranslate.x = matrix.m41 + dx;
        targetTranslate.y = matrix.m42 + dy;
        checkBorders();
        mind.style.transform = `matrix(${matrix.a}, ${matrix.b}, ${matrix.c}, ${matrix.d}, ${targetTranslate.x}, ${targetTranslate.y})`;
    }

    // listeners for mouse going out of window
    window.addEventListener("mouseout", (event) => {
        if (event.relatedTarget === null) {
            handleMouseUp();
        }
    });
    window.addEventListener("mousemove", (event) => {
        if (!(event.buttons & 1)) {
            handleMouseUp();
        }
    });
    window.addEventListener(
        "wheel",
        function (e) {
            e.preventDefault();
        },
        { passive: false },
    );

    async function handleDragStart(e) {
        e.stopPropagation();

        // same value as adjustedMousePosition in handleMouseMove, but this needs to be independent
        mouseX = e.clientX / scale;
        mouseY = e.clientY / scale;

        const activeThought = e.target.dataset.thoughtId;

        activeIndex = findThoughtIndexById(+activeThought);

        if (thoughts[activeIndex] != null) {
            isDragging = true;

            thoughts[activeIndex].x = e.target.offsetLeft;
            thoughts[activeIndex].y = e.target.offsetTop;

            thoughts[activeIndex].initialPosition = {
                x: thoughts[activeIndex].x,
                y: thoughts[activeIndex].y,
            };

            mouseOffset = {
                x: Math.round(mouseX - thoughts[activeIndex].x),
                y: Math.round(mouseY - thoughts[activeIndex].y),
            };
        }
    }

    async function handleDragMove(adjustedMousePosition) {
        if (isDragging) {
            mousePosition = {
                x: Number.parseInt(adjustedMousePosition.x),
                y: Number.parseInt(adjustedMousePosition.y),
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

    function handleDragEnd() {
        isDragging = false;
    }

    function handleRelated(selectedThought) {
        let related = {
            id: selectedThought.id,
            x: selectedThought.x,
            y: selectedThought.y,
        };
        dispatch("passRelatedEntry", related);
    }
    let added_relation_id = null;
    function connectStart(thoughtId) {
        if (isConnecting) {
            isConnecting = false;
            added_relation_id = null;
        } else {
            isConnecting = true;
            connectingThought = thoughts.find((t) => t.id === thoughtId);
            console.log("connectingThought", connectingThought);
            added_relation_id = connectingThought.id;
        }
    }

    function connectEnd(e) {
        isConnecting = false;
        // Use the added_relation variable to simulate the click
        if (added_relation_id) {
        } else {
            console.error("No starting thought ID found.");
        }
    }
</script>

<div
    id="mind"
    role="main"
    bind:this={mind}
    on:mousedown={handleMouseDown}
    on:mouseup={handleMouseUp}
    on:mousemove={handleMouseMove}
    on:wheel={handleWheel}
>
    {#each thoughts as thought (thought.id)}
        <div
            role="application"
            bind:this={thought.el}
            class="thought"
            id={thought.id}
            style="left: {thought.x}px; top: {thought.y}px;"
            on:mousedown={handleDragStart}
            on:mouseup={handleDragEnd}
            class:blurred={currentState === inputMode}
            data-thought-id={thought.id}
        >
            <button id="addRelated" on:click={() => handleRelated(thought)}>
                <svg width="8" height="8">
                    <line x1="0" y1="4" x2="8" y2="4" stroke="white" />
                    <line x1="4" y1="0" x2="4" y2="8" stroke="white" />
                </svg>
            </button>
            <button
                id="connect"
                draggable="true"
                on:click={() => connectStart(thought.id)}
                on:dragend={connectEnd}
            />

            <div id="thoughtTitle">
                {thought.title}
            </div>
        </div>
        {#if isConnecting && connectingThought}
            <svg
                style="position:absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none;"
            >
                <line
                    x1={connectingThought.x}
                    y1={connectingThought.y}
                    x2={trackX}
                    y2={trackY}
                    stroke="white"
                />
            </svg>
        {/if}
        {#if thought.relation_id}
            <svg
                class:blurred={currentState === inputMode}
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
        {#if thought.added_relation_id}
            <svg
                class:blurred={currentState === inputMode}
                style="position:absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none;"
            >
                <line
                    x1={thought.x + 50}
                    y1={thought.y + 50}
                    x2={thoughts.find((t) => t.id === thought.added_relation_id)
                        .x + 50}
                    y2={thoughts.find((t) => t.id === thought.added_relation_id)
                        .y + 50}
                    stroke="white"
                />
            </svg>
        {/if}
    {/each}
</div>

<style>
    .blurred {
        filter: blur(5px);
    }
    #inputWindow {
        position: fixed;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        background-color: midnightblue;
        border: 0.1em groove black;
        border-radius: 0.5em;
        height: 6em;
        padding: 10px;
        z-index: 3;
    }
    #textInput {
        background-color: transparent;
        border: none;

        color: white;
        font-size: 1em;
        bottom: 0;
        left: 0;
        margin: 0 10px;
        width: 100%;
    }
    #inputOk {
        background-color: transparent;
        border: 1px solid white;
        color: white;
        font-size: 1em;
        bottom: 0;
        right: 0;
        margin: 10px;
        padding: 5px;
        width: fit-content;
        height: fit-content;
    }
    #inputCancel {
        background-color: transparent;
        border: 1px solid white;
        color: white;
        top: 0;
        left: 0;
        font-size: 1em;
        margin: 10px;
        padding: 5px;
        width: fit-content;
        height: fit-content;
    }

    #mind {
        position: absolute;
        background-color: black;
        width: 769vw;
        min-height: 769vh;
        transform-origin: 50% 50%;
        border: 1px solid white;
        z-index: 0;
    }
    #mind:active {
        cursor: grabbing;
    }

    .thought {
        position: absolute;

        font: 1.5em sans-serif;
        width: 4em;
        height: 4em;
        background-color: maroon;
        z-index: 2;
    }
    #thoughtTitle {
        display: flex;
        flex-direction: column;
        background-color: transparent;
        color: white;
        font-size: 1em;
        margin: 0;
        padding: 0;
        width: 100%;
        text-align: center;
    }
    #addRelated {
        background-color: transparent;
        border: none;
        color: white;
        font-size: 1em;
        left: 0;
        top: 0;

        width: 1em;
        height: 1em;
    }
    #connect {
        border: solid 1px black;
        color: white;
        font-size: 1em;
        right: 0;
        top: 0;
        width: 1em;
        height: 1em;
    }
    svg {
        pointer-events: none; /* Prevents SVG from capturing mouse events */
    }
</style>
