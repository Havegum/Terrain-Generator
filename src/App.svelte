<script>
import { onMount } from 'svelte';
import { writable } from 'svelte/store';
import { spring } from 'svelte/motion';
import TerrainGenerator from './terrain.js';
import Canvas from './draw-webgl/Canvas.svelte';
import World from './draw-webgl/World.svelte';

const seaLevel = process.env.SEA_LEVEL || 0.39;
const points = process.env.WORLD_POINTS || 2**10;
let generator, world;

onMount(async () => {
  const seed = Math.floor(Math.random() * 1e8);
  // seed = 15043459; // DEBUG THIS ONE
  console.log('seed:', seed);
  generator = new TerrainGenerator(seed);
  world = await generator.generate({ points, seaLevel });
});
</script>


<Canvas let:canvas >
  {#if world}
    <World {canvas} {...world} />
  {/if}
</Canvas>
