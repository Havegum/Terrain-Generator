<script>
import { onMount } from 'svelte';
import { TerrainGenerator } from './terrain.js';
import draw from './draw-webgl/draw.js';

let canvas;
let canvas2d;
let seaLevel = 0.39;
let generator;
let done = false;
let translateCamera = () => {};
let rotateCamera = () => {};

let mouseDown = false;
let originalX = null;
let originalY = null;

onMount(async () => {
  let seed = Math.floor(Math.random() * 1e8);
  // seed = 30544282;
  // seed = 99951525;
  // seed = 44879021;
  // seed = 15043459; // DEBUG THIS ONE
  console.log('seed:', seed);
  generator = new TerrainGenerator({
    points: 2**12,
    seaLevel,
    seed
  });

  const {
    triangles, points, circumcenters, triangleHeights, coastLines, rivers, cellHeights, heights
  } = await generate();

  let now = Date.now();

  ({ translateCamera, rotateCamera } = draw(canvas, triangles, points, circumcenters, triangleHeights, seaLevel, coastLines, rivers, cellHeights, heights));

  canvas2d.getContext('2d').drawImage(canvas, 0, 0);
  console.log(`✓ rendered in ${Date.now() - now}ms`);
  done = true;
});

async function generate () {
  const world = await generator.generate();
  const heightMap = generator.generateHeightmap(100)

	const triangles = Array(world.voronoiTriangles.length / 3)
			.fill()
      .map((_, i) => i * 3)
			.map(j => [
        world.voronoiTriangles[j + 0],
        world.voronoiTriangles[j + 1],
        world.voronoiTriangles[j + 2],
      ]);

  return {
    ...world,
    triangles,
    heightMap: new ImageData(await heightMap, 100, 100),
  };
}

$: handleKeyDown = ({ key }) => translateCamera(key);

$: handleMouseMove = event => {
  if (!mouseDown) return;
  const x = event.clientX - originalX;
  const y = event.clientY - originalY;

  // window.requestAnimationFrame(() => {
    rotateCamera(x, y)
    originalX = event.clientX;
    originalY = event.clientY;
  // });
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
  window.requestAnimationFrame(() => rotateCamera(0, 0, event.deltaY));
}
</script>


<svelte:window
  on:keydown={handleKeyDown}
  on:mousedown={handleMouseDown}
  on:mouseup={handleMouseUp}
  on:mousemove={handleMouseMove}
  on:wheel={handleScroll}
/>

<canvas bind:this={canvas}  class="webgl" class:done width="1000" height="1000" />
<canvas bind:this={canvas2d} class="view" class:done width="1000" height="1000" />

<style>
.webgl.done {
  max-height: 100%;
  width: 100%;
  object-fit: cover;
  /* display: none; */
}
.view:not(.done) {
  display: none;
}
.view {
  display:none;
}
</style>
