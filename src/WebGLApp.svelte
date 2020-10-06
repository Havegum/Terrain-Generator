<script>
import { onMount } from 'svelte';
import { writable } from 'svelte/store';
import { spring } from 'svelte/motion';
import { TerrainGenerator } from './terrain.js';
import Canvas from './draw-webgl/Canvas.svelte';
import World from './draw-webgl/World.svelte';

let seaLevel = 0.39;
let generator, world;

let mouseDown = false;
let originalX = null;
let originalY = null;

let camera = writable({
  zRot: 0,
  yRot: 0,
  dist: 2,
  x: 0,
  y: 0,
});

let focus = spring({ x: 0, y: 0 }, {
  stiffness: 0.07,
  damping: 0.7,
  precision: 0.0001,
});

$: {
  $camera.x = $focus.x;
  $camera.y = $focus.y;
};

let zoom = spring(2, {
  stiffness: 0.19,
  damping: 1,
  precision: 0.001,
});

$: $camera.dist = $zoom;

onMount(async () => {
  let seed = Math.floor(Math.random() * 1e8);
  // seed = 15043459; // DEBUG THIS ONE
  console.log('seed:', seed);
  generator = new TerrainGenerator({
    points: 2**10,
    seaLevel,
    seed
  });

  world = await generate();
});

async function generate () {
  const w = await generator.generate();
  const heightMap = generator.generateHeightmap(100)
	const triangles = Array(w.voronoiTriangles.length / 3)
  	.fill()
    .map((_, i) => i * 3)
  	.map(j => [w.voronoiTriangles[j + 0], w.voronoiTriangles[j + 1], w.voronoiTriangles[j + 2]]);
  return {
    ...w,
    seaLevel,
    triangles,
    heightMap: new ImageData(await heightMap, 100, 100),
  };
}

const minStep = .005;

function handleKeyDown ({ key }) {
  // TODO: Continouous keydowns

  const moveStep = minStep + $camera.dist / 10;
  let z = $camera.zRot;
  let x = $focus.x;
  let y = $focus.y;

  switch (key) {
    case 'w':
      x += Math.sin(z) * moveStep;
      y += Math.cos(z) * moveStep;
      break;

    case 'a':
      x -= Math.sin(z + Math.PI / 2) * moveStep;
      y -= Math.cos(z + Math.PI / 2) * moveStep;
      break;

    case 's':
      x -= Math.sin(z) * moveStep;
      y -= Math.cos(z) * moveStep;
      break;

    case 'd':
      x += Math.sin(z + Math.PI / 2) * moveStep;
      y += Math.cos(z + Math.PI / 2) * moveStep;
      break;
  }

  focus.set({ x, y });
}

function handleMouseMove (event) {
  if (!mouseDown) return;
  const x = event.clientX - originalX;
  const y = event.clientY - originalY;

  $camera.zRot += x * 2e-3;
  $camera.yRot = Math.min(0, Math.max(Math.PI / -2, $camera.yRot + y * 2e-3));

  originalX = event.clientX;
  originalY = event.clientY;
}

function handleMouseDown (event) {
  originalX = event.clientX;
  originalY = event.clientY;
  mouseDown = true;
}

function handleMouseUp (event) {
  mouseDown = false;
}

function handleScroll (event) {
  $camera.dist = Math.max(0, $camera.dist + event.deltaY * 4e-2);
  zoom.set(Math.max(0, $zoom + event.deltaY * 8e-2));
}
</script>


<svelte:window
  on:keydown={handleKeyDown}
  on:mousedown={handleMouseDown}
  on:mouseup={handleMouseUp}
  on:mousemove={handleMouseMove}
  on:wheel={handleScroll}
/>


<Canvas let:canvas >
  {#if world}
    <World
      {canvas}
      {...world}
      {camera}
    />
  {/if}
</Canvas>
