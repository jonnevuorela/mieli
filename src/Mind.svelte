<script>
    import { onMount } from 'svelte';

    let thoughts = [];

    onMount(async ()=>{
        try {
            const response = await fetch('src-tauri/src/thoughts.json');
            thoughts= await response.json();
            console.log(`thoughts in mind component: ${thoughts}`);
        } catch (error) {
            console.error('Error fetching thoughts', error);
        }
        });

        function handleDragStart(event, thought){
            if (event.dataTransfer) {
                event.dataTransfer.setData('text/plain', JSON.stringify(thought));
                event.dataTransfer.setData('thoughtId', thought.id.toString());
                event.target.style.opacity = '0.5';
            }
        };

        function handleDrop(event){
            event.preventDefault();
            const data = event.dataTransfer.getData('text/plain');
            const droppedThought = JSON.parse(data);
            event.target.style.opacity = '';
            const thoughtId = event.dataTransfer.getData('thoughtId');
        };

        function handleDragOver(event){
            event.preventDefault();
        };

</script>

<div id="mind" role="main" on:drop={handleDrop} on:dragover={handleDragOver}>
    {#each thoughts as thought (thought.id)}
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