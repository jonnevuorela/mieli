<script>

    let userInput = "";

    export async function createNewComponent(){
        try {
            const newComponent = (await import("./newComponent.svelte")).default;

            const newComponentInstance = new newComponent({
                target: document.getElementById("main"),
                props: {userInput},
            });
        } catch (error) {
            console.error("Error creating new component:", error);
        }
    }

    export { userInput };
    
</script>

<div>
    <input type="text" bind:value={userInput} placeholder="Enter your thought" />
    <button on:click={createNewComponent}>Add a thought</button>
</div>

<div >
    <button style="border-radius: 5px" type="submit" on:submit={createNewComponent}>
        Add a tought
    </button>
</div>
