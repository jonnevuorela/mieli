<script>
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";

    let thoughts = [];

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
    console.log(`thoughts outside onMind fn: ${thoughts}`);

    function handleDragStart(event, thought) {
        if (event.dataTransfer) {
            event.dataTransfer.setData("text/plain", JSON.stringify(thought));
            event.dataTransfer.setData("thoughtId", thought.id.toString());
            event.target.style.opacity = "0.5";
        }
    }

    function handleDrop(event) {
        event.preventDefault();
        const data = event.dataTransfer.getData("text/plain");
        const droppedThought = JSON.parse(data);
        event.target.style.opacity = "";
        const thoughtId = event.dataTransfer.getData("thoughtId");
    }

    function handleDragOver(event) {
        event.preventDefault();
    }
</script>

<div id="mind" role="main" on:drop={handleDrop} on:dragover={handleDragOver}>
    {#each }
        <div
            class="thought"
            role="article"
            draggable="true"
            on:dragstart={(e) => handleDragStart(e, thought)}
        >
            <p>{thought.title}</p>
            <p>#{thought.id}</p>
        </div>
    {/each}
</div>
