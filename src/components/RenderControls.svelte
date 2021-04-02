<script>
export let renderer;
export let renderOptions;
</script>


<section>
    <h3>Render options</h3>
    <label for="rivers">Minimum river flux</label>
    <input id="rivers" type="range" min="0" max="1000" step="1" bind:value={renderOptions.webgl.riverCap}>
    <label for="rivers">{renderOptions.webgl.riverCap}</label>
</section>