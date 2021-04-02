<script>
import generate from './terrain.js';
import Canvas from './draw-webgl/Canvas.svelte';
import Controls from './draw-webgl/Controls.svelte';
import World from './draw-webgl/World.svelte';

const rng = () => Math.floor(Math.random() * 1e8);
const seaLevel = import.meta.env.SNOWPACK_PUBLIC_SEA_LEVEL || 0.39;
const points = import.meta.env.SNOWPACK_PUBLIC_WORLD_POINTS || 2 ** 10;
const seed = import.meta.env.SNOWPACK_PUBLIC_SEED || rng();


let controlSettings = { riverCap: 80 };

let world, stale;
async function gen (seed) {
  stale = true;
  // seed = 15043459; // DEBUG THIS ONE
  console.log('seed:', seed);
  world = await generate({ seed, points, seaLevel });
  stale = false;
}

gen(seed);
</script>

<Canvas let:canvas >
  <div class="overlay" class:stale/>
  {#if world}
    <World {canvas} {...world} {controlSettings} />
    <Controls bind:controlSettings on:regenerate={() => gen(rng())} />
  {/if}
</Canvas>


<style>
.overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  background-color: hsl(216, 3%, 8%);
  opacity: 0;
  transition: opacity 250ms ease-out;
}

.stale {
  opacity: 0.5;
  cursor: progress;
  pointer-events: auto;
  transition: none;
}
</style>