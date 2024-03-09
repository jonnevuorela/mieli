<script>
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";

    let objects = [];
    let dropped = [];
    let dropped_in = false;
    let drag_over = false;
    let status = "";

    onMount(async () => {
        try {
            const response = await invoke("read_json");
            objects = JSON.parse(response);
            console.log(
                `thoughts in mind component from invoke:`,
                JSON.stringify(objects),
            );
        } catch (error) {
            console.log("Error invoking thoughts", error);
        }
    });
    console.log(`thoughts outside onMount fn: ${objects}`);

    function handleDragEnter(e) {
        status = "You are dragging over the " + e.target.getAttribute("id");
    }

    function handleDragLeave(e) {
        status = "You left the " + e.target.getAttribute("id");
    }

    function handleDragDrop(e) {
        if (drag_over) {
            try {
                var element_id = e.dataTransfer.getData("text");
                dropped = dropped.concat(element_id);
                dropped_in = true;
                status = "You droped " + element_id + " into drop zone";
                console.log(`drag_over: ${drag_over}`);
            } catch (error) {
                console.error("eipä onnnistu", error);
            }
        }
    }

    function handleDragStart(e) {
        status = "Dragging the element " + e.target.getAttribute("id");
        e.dataTransfer.dropEffect = "move";
        e.dataTransfer.setData("text", e.target.getAttribute("id"));
    }
    function handleDragEnd(e) {
        handleDragDrop(e);
    }

    function handleDragOver(e) {
        e.preventDefault();
        drag_over = true;
    }
</script>

<h2 id="app_status">Drag status: {status}</h2>
<h1>Drop Zone</h1>

<div
    on:dragenter={handleDragEnter}
    on:dragleave={handleDragLeave}
    on:drop={handleDragDrop}
    id="drop_zone"
    role="main"
    on:dragover={handleDragOver}
>
    {#each objects.filter((v) => dropped.includes(`${v.id}`)) as { id }, i}
        <div class="objects" {id} style="cursor: auto">
            Object {id}
        </div>
    {/each}
</div>

{#each objects.filter((v) => !dropped.includes(`${v.id}`)) as { id }, i}
    <div
        {id}
        role="main"
        class="objects"
        draggable="true"
        on:dragstart={handleDragStart}
        on:dragend={handleDragEnd}
    >
        Object {id}
    </div>
{/each}
