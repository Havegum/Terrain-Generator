<script>
import { onMount } from 'svelte';
import { scaleLinear } from 'd3-scale';

// Konva & Svelte tutorial here:
// https://geoexamples.com/svelte/2020/08/05/svelte-konva-mapping.html
import Konva from './draw-components/Konva.svelte';
import Layer from './draw-components/Layer.svelte';
import Group from './draw-components/Group.svelte';
import Circle from './draw-components/Circle.svelte';
import Polygon from './draw-components/Polygon.svelte';

import { svgRender, TerrainGenerator } from './terrain.js';
import { interpolateYlGn as interpolateLand, interpolatePuBu as interpolateSea } from 'd3-scale-chromatic';

let canvas, width, height, generator;
let seaLevel = 0.39;
let extent = { width: 1, height: 1 };

const scaleSingle = scaleLinear()
  .domain([0, 1])
  .range([0, 500]);

const scalePair = pair => pair.map(scaleSingle);
const scaleCoords = coords => coords.map(scalePair);

onMount(() => {
  console.log('hey');
  let seed = Math.floor(Math.random() * 1e8);

  generator = new TerrainGenerator({
		points: 2**10,
		seaLevel,
		seed
	});

  generate();
});

let isLand;
let heights;
let voronoiAdjacency;
let circumcenters;
let rivers;
let coastLines;
let points;
let triangles = [];
let heightMap;

function interpolateHeight (i) {
	let height = heights[i];
	if (height === undefined) return 'none';

	return isLand[i]
		? interpolateLand(1 - height)
		: interpolateSea(1 - height);
}

function frame (fn) {
	return new Promise(resolve => {
		requestAnimationFrame(() =>
			fn(),
			requestAnimationFrame(resolve)
		);
	});
}

async function generate () {
	let world = await generator.generate();

	isLand = world.isLand;
	heights = world.triangleHeights;
	voronoiAdjacency = world.voronoiAdjacency;
	circumcenters = world.circumcenters;

	const setRivers = () => rivers = world.rivers;
	const setCoasts = () => coastLines = world.coastLines;
	const setPoints = () => points = world.points;
	const setTerrain = () => triangles = Array(world.voronoiTriangles.length / 3)
			.fill()
			.map((_, i) => {
				let j = i * 3;
				let cellIndex = world.voronoiTriangles[j + 0] * 2;
				let nodeIndex1 = world.voronoiTriangles[j + 1] * 2;
				let nodeIndex2 = world.voronoiTriangles[j + 2] * 2;

				return [
					[world.points[cellIndex], world.points[cellIndex + 1]],
					[circumcenters[nodeIndex1], circumcenters[nodeIndex1 + 1]],
					[circumcenters[nodeIndex2], circumcenters[nodeIndex2 + 1]]
				];
			});


	const setHeightmap = async () => heightMap = new ImageData(await generator.generateHeightmap(100), 100, 100);

	frame(setTerrain)
		.then(() => frame(setCoasts))
		.then(() => frame(setRivers))
		.then(() => frame(setHeightmap))
		.then(() => frame(setPoints));
}

$: if (triangles[0]) console.log(triangles[0], scaleCoords(triangles[0]));
</script>

<Konva>
  <Layer name="geography">
    <Group name="triangles">
      {#each triangles as coords, i}
        <Polygon
          coordinates={scaleCoords(coords)}
          fill={interpolateHeight(i)}
          stroke={interpolateHeight(i)}
          strokeWidth={1}
        />
      {/each}
    </Group>
  </Layer>
</Konva>
