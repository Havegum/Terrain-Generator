<script>
import { onMount } from 'svelte';
import { TerrainGenerator } from './terrain.js';
import draw from './draw-webgl/draw.js';

let canvas;
let generator;
let seaLevel = 0.39;
let isLand = [];
let heights = [];
let voronoiAdjacency = [];
let circumcenters = [];
let rivers = [];
let coasts = [];
let points = [];
let triangles = [];
let heightMap = [];

onMount(async () => {
  let seed = Math.floor(Math.random() * 1e8);
  console.log('seed:', seed);
  generator = new TerrainGenerator({
    points: 2**12,
    seaLevel,
    seed
  });

  await generate();

  draw(canvas, triangles, points, circumcenters, heights);
});

async function generate () {
  let world = await generator.generate();

	isLand = world.isLand;
	heights = world.triangleHeights;
	voronoiAdjacency = world.voronoiAdjacency;
	circumcenters = world.circumcenters;
	rivers = world.rivers;
	coasts = world.coastLines;
	points = world.points;
	triangles = Array(world.voronoiTriangles.length / 3)
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

	heightMap = new ImageData(await generator.generateHeightmap(100), 100, 100);

  let now = Date.now();
}

</script>

<canvas bind:this={canvas} width="500" height="500" />
