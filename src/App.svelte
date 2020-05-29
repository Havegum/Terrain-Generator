<script>
import { onMount } from 'svelte';
import { svgRender, TerrainGenerator } from './terrain.js';
import { interpolateYlGn as interpolateLand, interpolatePuBu as interpolateSea } from 'd3-scale-chromatic';
import { max, min } from 'd3-array';

let svg, width, height;
let canvas;

let generator;

let extent = { width: 1, height: 1 };
let seaLevel = 0.39;

let triangleEdges = [];
let centroids = [];
let heights = [];
let isLand = [];
let cells = [];
let nodes = [];
let nodeAdjacency = [];
let circumcenters = [];
let coastLines = [];
let rivers = [];

let heightMap;
let heightMapVisible = false;

let circumcenterIndex;

$: viewBoxWidth = 1000 * extent.width;
$: viewBoxHeight = 1000 * extent.height;
$: minX = -1000 * extent.width / 2;
$: minY = -1000 * extent.height / 2;
$: viewBox = `0 0 ${viewBoxWidth} ${viewBoxHeight}`;

onMount(() => {
	if (!svg) return;
	let seed = Math.floor(Math.random() * 1e8);
	console.log('seed:', seed);
	generator = new TerrainGenerator({
		// yieldPoints: true,
		points: 2**10,
		seaLevel,
		// seed
		seed: 82013022
		// seed: 0.6427742671532695
		// seed: 0.2459701851370404,
	});
	generate();
});

function revealHeightmap() {
	let ctx = canvas.getContext('2d');
	canvas.width = 100;
	canvas.height = 100;
	ctx.putImageData(heightMap, 0, 0);
	heightMapVisible = true;
}


function hideHeightmap() {
	heightMapVisible = false;
}

async function generate () {
	let world = await generator.generate();

	nodes = world.points;

	cells = world.cells;
	nodes = world.nodes;
	rivers = world.rivers;
	isLand = world.isLand;
	centroids = world.points;
	coastLines = world.coastLines;
	heights = world.triangleHeights;
	nodeAdjacency = world.voronoiAdjacency;
	circumcenters = world.voronoi.circumcenters;

	triangleEdges = Array(world.voronoiTriangles.length / 3)
			.fill()
			.map((_, i) => {
				let j = i * 3;
				let centroidIndex = world.voronoiTriangles[j + 0] * 2;
				let nodeIndex1 = world.voronoiTriangles[j + 1] * 2;
				let nodeIndex2 = world.voronoiTriangles[j + 2] * 2;

				return [
					[world.delaunay.points[centroidIndex], world.delaunay.points[centroidIndex + 1]],
					[circumcenters[nodeIndex1], circumcenters[nodeIndex1 + 1]],
					[circumcenters[nodeIndex2], circumcenters[nodeIndex2 + 1]]
				];
			});

	heightMap = new ImageData(await generator.generateHeightmap(100), 100, 100);
}

function interpolateHeight (i) {
	let height = heights[i];
	if (height === undefined) return 'none';

	return isLand[i]
		? interpolateLand(1 - height)
		: interpolateSea(1 - height);
}
</script>


<div class="control-panel">

	<button on:click={generate}>Generate</button>
	<button on:mousedown={revealHeightmap} on:mouseup={hideHeightmap}>Show Heightmap</button>
	<!-- <button on:click={improve}>ImprovePoints</button>
	<button on:click={delaunay}>Delaunay</button>
	<button on:click={mapHeights}>Map heights</button> -->
	<input type="number" bind:value={circumcenterIndex}>
</div>

<div class="wrap" bind:clientWidth={width} bind:clientHeight={height}>
	<canvas bind:this={canvas} class:visible={heightMapVisible}></canvas>
	<svg bind:this={svg} {viewBox}>



		<g class="cells">
			{#each cells as cell, i}
				<path
					d={svgRender(cell)}
				/>
			{/each}
		</g>
		<g class="triangles">
			{#each triangleEdges as edge, i}
			<!-- fill={triangle.height > sealevel ? interpolateLand(1 - triangle.height) : interpolateSea(1 - triangle.height)} -->
				<path
					d={svgRender(edge)}
					fill={interpolateHeight(i)}
					stroke={interpolateHeight(i)}
					on:click={() => console.log(i, edge, heights[i], isLand[i])}
				/>
			{/each}
		</g>

		<g class="coast">
			{#each coastLines as coast}
				<path
					d={svgRender(coast)}
				/>
			{/each}
		</g>

		<g class="river">
			{#each rivers as river}
			{#if river.flux >= 0}
				<path
					d={svgRender(river.points)}
					stroke-width={Math.log(river.flux + 1) / 2}
				/>
			{/if}
			{/each}
		</g>


		{#if circumcenterIndex !== undefined}
			<circle
				cx={circumcenters[circumcenterIndex * 2 + 0] * 1000}
				cy={circumcenters[circumcenterIndex * 2 + 1] * 1000}
				r="20"
				fill="none"
				stroke="red"
				stroke-width="3"
			/>
			{#each nodeAdjacency[circumcenterIndex] as neighbor}
				<line
					x1={circumcenters[circumcenterIndex * 2 + 0] * 1000}
					y1={circumcenters[circumcenterIndex * 2 + 1] * 1000}
					x2={circumcenters[neighbor * 2 + 0] * 1000}
					y2={circumcenters[neighbor * 2 + 1] * 1000}
					stroke="red"
					stroke-width="2"
				/>
			{/each}
		{/if}
		<!-- <g class="vertices">
			{#each centroids as centroid}
				<rect
					x={centroid[0] * 1000 - 1.5}
					y={centroid[1] * 1000 - 1.5}
					width="3"
					height="3"
					fill="none"
					stroke="white"
				/>
			{/each}

			{#each nodes as node}
				<circle
					cx={node[0] * 1000}
					cy={node[1] * 1000}
					r="1.5"
				/>
			{/each}
		</g> -->
	</svg>
</div>


<style lang="scss">
.wrap {
	place-items: center;
	display: grid;
	width: 100%;
}

svg {
	width: 100%;
	height: 100%;
	display: block;
	// overflow: visible;
}

canvas {
	position: absolute;
	top: 0;
	left: 0;
	width: 100%;
	height: 100%;
	opacity: 0;
	transition: opacity 200ms;
}

canvas.visible {
	opacity: .95;
}

.cells path {
	fill: none;
}

.triangles path {
	stroke-width: 1;
}

.coast path,
.river path {
	stroke-linecap: round;
	stroke-linejoin: round;
}

.river path {
	stroke: rgb(13, 133, 193);
	// stroke-width: 2;
}

.coast path {
	stroke: #133b66;
	stroke-width: 2;
}

.slopes line {
	stroke: black;
	stroke-width: 1;
}
</style>
