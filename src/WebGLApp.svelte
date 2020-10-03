<script>
import { onMount } from 'svelte';
import { TerrainGenerator } from './terrain.js';
import draw from './draw-webgl/draw.js';

let canvas;
let canvas2d;
let seaLevel = 0.39;
let generator;
let done = false;

onMount(async () => {
  let seed = Math.floor(Math.random() * 1e8);
  // seed = 30544282;
  // seed = 99951525;
  // seed = 44879021;
  // seed = 15043459; // DEBUG THIS ONE
  console.log('seed:', seed);
  generator = new TerrainGenerator({
    points: 2**14,
    seaLevel,
    seed
  });

  const {
    triangles, points, circumcenters, triangleHeights, coasts, rivers, cellHeights, heights
  } = await generate();

  let now = Date.now();
  draw(canvas, triangles, points, circumcenters, triangleHeights, seaLevel, coasts, rivers, cellHeights, heights);

  canvas2d.getContext('2d').drawImage(canvas, 0, 0);
  console.log(`✓ rendered in ${Date.now() - now}ms`);
  done = true;
});

async function generate () {
  let world = await generator.generate();

	let triangles = Array(world.voronoiTriangles.length / 3)
			.fill()
			.map((_, i) => {
				const j = i * 3;
				const cellIndex = world.voronoiTriangles[j + 0];
				const nodeIndex1 = world.voronoiTriangles[j + 1];
				const nodeIndex2 = world.voronoiTriangles[j + 2];

        return [
          cellIndex,
          nodeIndex1,
          nodeIndex2,
        ];
			});

  return {
    triangleHeights: world.triangleHeights,
    heights: world.heights,
    voronoiAdjacency: world.voronoiAdjacency,
    circumcenters: world.circumcenters,
    rivers: world.rivers,
    coasts: world.coastLines,
    points: world.points,
    triangles,
    cellHeights: world.cellHeights,
    heightMap: new ImageData(await generator.generateHeightmap(100), 100, 100),
  };
}

</script>

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
