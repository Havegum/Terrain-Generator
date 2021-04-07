<script>
export let renderer;
export let renderOptions;
</script>


<section>
    <h3>Render options</h3>
    
    <div class="render-selector">
        <label class:selected={renderer==='webgl'}>
            <p>Webgl</p>
            <input type="radio" bind:group={renderer} value="webgl"/>
        </label>
        
        <label class:selected={renderer==='svg'}>
            <p>Svg</p>
            <input type="radio" bind:group={renderer} value="svg"/>
        </label>
    </div>

    {#if renderer === 'webgl'}
        <label for="rivers">Minimum river flux</label>
        <input id="rivers" type="range" min="0" max="1000" step="1" bind:value={renderOptions.webgl.riverCap}>
        <label for="rivers">{renderOptions.webgl.riverCap}</label>
    {:else if renderer === 'svg'}
        {#each ['cliffColor', 'hillColor', 'landColor', 'waterColor', 'depthColor'] as c}
            <label for={c}>{c}</label>
            <input id={c} type="text" bind:value={renderOptions.svg[c]}>
            <label for={c}>{renderOptions.svg[c]}</label>
        {/each}
    {/if}
</section>


<style>
.render-selector {
    grid-column: 1 / -1;
    display: grid;
    grid-template-columns: 1fr 1fr;
}


.render-selector label {
    border: 1px solid transparent;
    cursor: pointer;
    background-color: hsla(0, 0%, 100%, 0.075);
    text-align: center;
    color:hsla(0, 0%, 100%, 0.75);
    padding: 7px;
}

.render-selector label:first-child {
    border-radius: 4px 0 0 4px;
}

.render-selector label:last-child {
    border-radius: 0 4px 4px 0;
}

input[type="radio"] {
    display: none;
}

label.selected {
    color: white;
    background-color: hsla(0, 0%, 100%, 0.1);
    border-color: hsla(0, 0%, 100%, 0.25);
}

.render-selector label:hover {
    background-color: hsla(0, 0%, 100%, 0.2);
    color: white;
    border-color: white;
}
</style>