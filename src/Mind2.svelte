<script>
    import { onMount } from "svelte";

    import { invoke } from "@tauri-apps/api/tauri";

    let Mind;
    let thoughts = [];
    let dropped = [];
    let status = "";

    onMount(async () => {
        try {
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

    function handleDragEnter(e) {
        status =
            "you are dragging the " +
            e.target.getAttribute("id").target.getAttribute("id");
    }

    function handleDragLeave(e) {
        status =
            "You left the " +
            e.target.getAttribute("id").target.getAttribute("id");
    }

    function handleDragDrop(e) {
        e.preventDefault();
        const element_id = e.dataTransfer.getData("text");
        const target_id = e.target.id;

        if (element_id !== target_id) {
            const draggedIndex = thoughts.findIndex(
                (thought) => thought.id == target_id,
            );

            const targetIndex = thoughts.findIndex(
                (thought) => thought.id == target_id,
            );

            thoughts.splice(draggedIndex, 1);
            thoughts.splice(targetIndex, 0, thoughts[draggedIndex]);

            dropped = thoughts.map((thought) => thought.id);
            status = "You dropped thought " + element_id + "into the mind";
        }
    }

    function handleDragStart(e) {
        const thoughtData = JSON.parse(e.dataTransfer.getData("text/plain"));
    }

    function handleDragEnd(e) {
        e.preventDefault();
        const element_id = e.dataTransfer.getData("text");
        const target_id = e.target.id;

        if (element_id !== target_id) {
            const draggedIndex = thoughts.findIndex(
                (thought) => thought.id == target_id,
            );

            const targetIndex = thoughts.findIndex(
                (thought) => thought.id == target_id,
            );

            thoughts.splice(draggedIndex, 1);
            thoughts.splice(targetIndex, 0, thoughts[draggedIndex]);

            dropped = thoughts.map((thought) => thought.id);
            status = "You dropped thought " + element_id + "into the mind";
            status = "You let the thought " + e.target.id + "go.";
        }
    }

    function handleDragOver(e) {
        event.preventDefault();
    }
</script>

<h2 id="app_status">Drag status: {status}</h2>

<div
    role="main"
    id="Mind"
    bind:this={Mind}
    on:dragenter={handleDragEnter}
    on:dragleave={handleDragLeave}
    on:drop={handleDragDrop}
    ondragover="return false"
>
    {#each thoughts as thought (thought.id)}
        {#if !dropped.includes(thought.id)}
            <div
                role="cell"
                tabindex={thought.id}
                class="thought"
                id={thought.id}
                style={`cursor:auto; position: absolute; left: ${thought.x}px; top: ${thought.y}px;`}
                draggable="true"
                on:dragstart={(e) => {
                    e.dataTransfer.setData(
                        "text/plain",
                        JSON.stringify(thought),
                    );
                    handleDragStart(e);
                }}
                on:dragend={handleDragEnd}
            >
                <p>{thought.title}</p>
                <p>{thought.id}</p>
            </div>
        {/if}
    {/each}
</div>
{#each dropped as id}
    <div class="thought" {id} style="cursor:auto" draggable="true">
        <p>{thoughts.find((thought) => thought.id == id).title}</p>
        <p>#{id}</p>
    </div>
{/each}
