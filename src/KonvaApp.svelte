<script>
import { onMount } from 'svelte';
import { scaleLinear } from 'd3-scale';

// Konva & Svelte tutorial here:
// https://geoexamples.com/svelte/2020/08/05/svelte-konva-mapping.html
import Konva from './draw-components-konva/Konva.svelte';
import Layer from './draw-components-konva/Layer.svelte';
import Group from './draw-components-konva/Group.svelte';
import Circle from './draw-components-konva/Circle.svelte';
import Polygon from './draw-components-konva/Polygon.svelte';
import Path from './draw-components-konva/Path.svelte';

import { svgRender, TerrainGenerator } from './terrain.js';
import { interpolateYlGn as interpolateLand, interpolatePuBu as interpolateSea } from 'd3-scale-chromatic';

let generator;
let seaLevel = 0.39;
let extent = { width: 1, height: 1 };
let riverMin = 0;

const scaleSingle = scaleLinear()
  .domain([0, 1])
  .range([0, 500]);

const scalePair = pair => pair.map(scaleSingle);
const scale = coords => coords.map(scalePair);

onMount(() => {
  let seed = Math.floor(Math.random() * 1e8);
  console.log('seed:', seed);
  generator = new TerrainGenerator({
		points: 2**12,
		seaLevel,
		seed
	});

  generate();
});

let isLand;
let heights;
let voronoiAdjacency;
let circumcenters;
let rivers = [];
let coasts = [];
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
	const setCoasts = () => coasts = world.coastLines;
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

  let now = Date.now();
	frame(setTerrain)
		.then(() => frame(setCoasts))
		.then(() => frame(setRivers))
		.then(() => frame(setHeightmap))
		.then(() => frame(setPoints))
    .then(() => console.log('ms:', Date.now() - now));
}

function getRiverCoords (river, i) {
  // let line =
  return [
    [circumcenters[river[i + 0][0] * 2], circumcenters[river[i + 0][0] * 2 + 1]],
    [circumcenters[river[i + 1][0] * 2], circumcenters[river[i + 1][0] * 2 + 1]]
  ];
}
</script>


<Konva>
  <Layer name="geography">
    <Group zIndex={0} name="triangles">
      {#each triangles as coords, i}
        <Polygon
          coordinates={scale(coords)}
          fill={interpolateHeight(i)}
          stroke={interpolateHeight(i)}
          strokeWidth={1}
        />
      {/each}
    </Group>

    <Group zIndex={2} name="coast">
      {#each coasts as coast}
        <Path
          coordinates={scale(coast)}
          stroke="#133b66"
        />
      {/each}
    </Group>

    <Group zIndex={1}>
      {#each rivers as river}
        <Group>
          {#each Array(river.length - 1).fill() as _, i}
            {#if river[i+1][1] > riverMin}
              <Path
                coordinates={scale(getRiverCoords(river, i))}
                strokeWidth={Math.log(river[i+1][1] - riverMin + 1) / 2}
                stroke="#0d85c1"
              />
            {/if}
          {/each}
        </Group>
      {/each}
    </Group>
  </Layer>
</Konva>
