<script>
export let world;
export let renderOptions;

function arrOfLength (len) {
    return (new Array(len)).fill();
}
</script>


<svg viewBox="-0.1 -0.1 1.2 1.2">
    <rect
        class="bounding-box"
        vector-effect="non-scaling-stroke"
        width="1"
        height="1"
    />

    {#each arrOfLength(world.points.length / 2) as _, i}
        <circle
            cx={world.points[i * 2]}
            cy={1 - world.points[i * 2 + 1]}
            r={0.0025}
        />
    {/each}
</svg>


<style>
svg {
    width: 100%;
    height: 100%;
    position: absolute;
    top: 0;
    left: 0;
}

.bounding-box {
    fill: none;
    stroke: red;
}
</style>