<script>
import generate from './utils/terrain.js';
import Canvas from './components/webgl/Canvas.svelte';
import Controls from './components/Controls.svelte';
import RenderControls from './components/RenderControls.svelte';
import GenerationControls from './components/GenerationControls.svelte';

import World from './components/webgl/World.svelte';
import SvgRenderer from './components/svg/Renderer.svelte';

const rng = () => Math.floor(Math.random() * 1e8);
const seaLevel = process.env.SEA_LEVEL || 0.39;
const points = process.env.WORLD_POINTS || 2 ** 10;
const seed = process.env.SEED || rng();

let renderer = 'svg';

let generationOptions = {
  seaLevel,
  points,
  seed,
};

let renderOptions = {
  webgl: {
    riverCap: 80,
  },
  svg: {
    hillColor: '#e8ffcd',
    cliffColor: '#2f2b29',
    landColor: '#338a3e',
    waterColor: '#11acc6',
    depthColor: '#041a2d',
  }
};

let world, stale;
async function gen () {
  stale = true;
  // seed = 15043459; // DEBUG THIS ONE
  console.log('seed:', generationOptions.seed);
  world = await generate(generationOptions);
  stale = false;
}

gen();
</script>


{#if world}
  {#if renderer === 'webgl'}
    <Canvas let:canvas >
      <World {canvas} {...world} renderOptions={renderOptions.webgl} />
    </Canvas>
  {:else if renderer === 'svg'}
    <SvgRenderer {world} renderOptions={renderOptions.svg} />
  {/if}
{/if}

<div class="overlay" class:stale />

<Controls>
  <RenderControls bind:renderer bind:renderOptions />
  <GenerationControls
    bind:generationOptions
    on:reseed={() => generationOptions.seed = rng()}
    on:regenerate={gen}
  />
</Controls>


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