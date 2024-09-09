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
    function connectStart(thoughtId, event) {
        event.stopPropagation();
        if (!isConnecting) {
            isConnecting = true;
            connectingThought = thoughts.find((t) => t.id === thoughtId);
            console.log("connectingThought", connectingThought);
            added_relation_id = connectingThought.id;
        } else if (isConnecting && added_relation_id) {
            console.log("added_relation_id", added_relation_id);
            const target = thoughts.find((t) => t.id === thoughtId);
            console.log("target", target.id);
            invoke("update_json", {
                addedrelationid: Number(added_relation_id),
                targetid: Number(target.id),
            })
                .then(() => {
                    console.log("relation added");
                    added_relation_id = null;
                    isConnecting = false;
                    dispatch("update");
                })
                .catch((error) => {
                    console.error("Error adding relation", error);
                });
        } else if (!added_relation_id) {
            console.error("No starting thought ID found.(connectEnd)");
            isConnecting = false;
            added_relation_id = null;
        }
    }
    function connectEnd() {
        isConnecting = false;
        connectingThought = null;
        added_relation_id = null;
        console.log("connectEnd");
    }
    function handleDelete(thoughtId) {
        const thoughtIndex = thoughts.findIndex((t) => t.id === thoughtId) + 1;
        invoke("delete_entry", { targetid: Number(thoughtIndex) })
            .then(() => {
                console.log("thought deleted");
                dispatch("update");
            })
            .catch((error) => {
                console.error("Error deleting thought", error);
            });
    }
    function handleDisconnect(thoughtId) {
        const thoughtIndex = thoughts.findIndex((t) => t.id === thoughtId);
        thoughts[thoughtIndex].relation_id = 0;
        thoughts[thoughtIndex].added_relation_id = 0;
        console.log("thoughts[thoughtIndex]", thoughts[thoughtIndex]);
        const data = JSON.stringify(thoughts);
        invoke("write_json", { data })
            .then(() => {
                console.log("relation deleted");
                dispatch("update");
            })
            .catch((error) => {
                console.error("Error deleting relation", error);
            });
    }
</script>

<div
    id="mind"
    class="mind"
    role="main"
    bind:this={mind}
    on:mousedown={handleMouseDown}
    on:mouseup={handleMouseUp}
    on:mousemove={handleMouseMove}
    on:wheel={handleWheel}
    on:click={isConnecting ? connectEnd : null}
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
            <div class="row1">
                <button
                    class="connect"
                    on:click={(event) => {
                        connectStart(thought.id, event);
                    }}
                >
                    <img src="/thought_connect.svg" alt="connect" />
                </button>

                <button
                    class="delete"
                    on:click={() => handleDelete(thought.id)}
                >
                    <img src="/thought_delete.svg" alt="delete" />
                </button>
                <button
                    class="disconnect"
                    on:click={() => handleDisconnect(thought.id)}
                >
                    <img src="/thought_disconnect.svg" alt="disconnect" />
                </button>
                <button
                    class="addRelated"
                    on:click={() => handleRelated(thought)}
                >
                </button>
            </div>
            <div class="thoughtTitle">
                {thought.title}
            </div>
        </div>
        {#if isConnecting && connectingThought}
            <svg
                style="position:absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none;"
            >
                <line
                    x1={connectingThought.x + 30}
                    y1={connectingThought.y + 30}
                    x2={trackX}
                    y2={trackY}
                    stroke="rgb(204, 170, 0)"
                />
            </svg>
        {/if}
        {#if thought.relation_id}
            <svg
                class:blurred={currentState === inputMode}
                style="position:absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none;"
            >
                <line
                    x1={thought.x + 100}
                    y1={thought.y + 75}
                    x2={thoughts.find((t) => t.id === thought.relation_id).x +
                        100}
                    y2={thoughts.find((t) => t.id === thought.relation_id).y +
                        75}
                    stroke="rgb(204, 170, 0)"
                />
            </svg>
        {/if}
        {#if thought.added_relation_id}
            <svg
                class:blurred={currentState === inputMode}
                style="position:absolute; top: 0; left: 0; width: 100%; height: 100%;"
            >
                <line
                    x1={thought.x + 100}
                    y1={thought.y + 75}
                    x2={thoughts.find((t) => t.id === thought.added_relation_id)
                        .x + 100}
                    y2={thoughts.find((t) => t.id === thought.added_relation_id)
                        .y + 75}
                    stroke="rgb(204, 170, 0)"
                />
            </svg>
        {/if}
    {/each}
</div>

<style>
    .mind {
        position: absolute;
        background-color: black;
        background-image: url("/mind_background.png");
        background-size: cover;
        background-repeat: no-repeat;
        background-position: center;
        width: 16000px;
        min-height: 9000px;
        transform-origin: 50% 50%;
        border: 1px solid white;
        z-index: 0;
    }
    .mind:active {
        cursor: grabbing;
    }

    .row1 {
        display: flex;
        flex-direction: row;
        margin-left: 35px;
        margin-top: 30px;
        margin-right: 50px;
        justify-content: space-between;
        align-items: center;
        background-color: transparent;
    }
    .thought {
        position: absolute;
        display: flex;
        flex-direction: column;
        width: 250px;
        height: 175px;
        background-color: rgba(0, 0, 0, 0);
        background-image: url("/thought.png");
        background-size: cover;
        background-repeat: no-repeat;
        z-index: 2;
    }
    .thoughtTitle {
        position: absolute;
        bottom: 60px;
        display: flex;
        flex-direction: column;
        background-color: transparent;
        font-size: 38px;
        -webkit-text-stroke: 1px black;
        -webkit-text-fill-color: darkgoldenrod;
        margin: 0;
        padding-top: 15px;
        width: 100%;
        text-align: center;
        -webkit-user-select: none;
    }
    .disconnect {
        position: absolute;
        background-color: transparent;
        background-image: url("/thought_disconnect.png");
        background-size: cover;
        background-repeat: no-repeat;
        border: none;
        top: 100px;
        left: 10px;
        width: 40px;
        height: auto;
    }
    .disconnect:hover {
        -webkit-transform: scale(1.25) rotate(10deg);
    }
    .delete {
        position: absolute;
        background-color: transparent;
        background-image: url("/thought_delete.png");
        background-size: cover;
        background-repeat: no-repeat;
        border: none;
        top: 100px;
        right: 10px;
    }
    .delete:hover {
        -webkit-transform: scale(1.25) rotate(-10deg);
    }
    .addRelated {
        position: absolute;
        background-color: transparent;
        top: 40px;
        right: 35px;
        border: none;
        color: white;
        font-size: 1em;
        width: 1em;
        height: 1em;
    }
    .connect {
        position: absolute;
        background-color: transparent;
        border: none;
        top: 40px;
        left: 35px;
        width: 40px;
        height: 29.2px;
        -webkit-transition:
            transform 0.3s ease,
            top 0.3s ease,
            left 0.3s ease;
    }
    .connect:hover {
        top: 35px;
        -webkit-transform: scale(1.25) rotate(-10deg);
    }
</style>
