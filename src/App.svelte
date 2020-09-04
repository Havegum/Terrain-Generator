<script>
import { onMount } from 'svelte';
import { svgRender, TerrainGenerator } from './terrain.js';
import { interpolateYlGn as interpolateLand, interpolatePuBu as interpolateSea } from 'd3-scale-chromatic';

let svg, width, height;
let canvas;

let generator;

let extent = { width: 1, height: 1 };
let seaLevel = 0.39;

let triangleEdges = [];
let heights = [];
let isLand = [];
let points = [];
let voronoiAdjacency = [];
let circumcenters = [];
let coastLines = [];
let rivers = [];
let riverMin = 0;

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
	// seed = 56645420;
	// seed = 82306550;
	console.log('seed:', seed);
	generator = new TerrainGenerator({
		// yieldPoints: true,
		points: 2**12,
		seaLevel,
		seed
		// seed: 82013022
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

function frame (fn) {
	return new Promise(resolve => {
		requestAnimationFrame(() => {
			fn();
			requestAnimationFrame(resolve);
		});
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
	const setTerrain = () => triangleEdges = Array(world.voronoiTriangles.length / 3)
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
 	<input type="range" min="0" max="30" step="1" bind:value={riverMin} /> I like rivers {riverMin < 10 ? "a lot <3" : riverMin < 20 ? "a bit." : "gone!"}
</div>

<div class="wrap" bind:clientWidth={width} bind:clientHeight={height}>
	<canvas bind:this={canvas} class:visible={heightMapVisible}></canvas>
	<svg bind:this={svg} {viewBox}>



		<!-- <g class="cells">
			{#each cells as cell, i}
				<path
					d={svgRender(cell)}
				/>
			{/each}
		</g> -->
		<g class="triangles" class:active={triangleEdges.length > 0}>
			{#each triangleEdges as edge, i}
				<path
					d={svgRender(edge)}
					fill={interpolateHeight(i)}
					stroke={interpolateHeight(i)}
					on:click={() => console.log(i, edge, heights[i], isLand[i])}
				/>
			{/each}
		</g>


		<g class="river" class:active={rivers.length > 0}>
			{#each rivers as river}
				<g>
					{#each Array(river.length - 1).fill() as _, i}
						{#if river[i+1][1] > riverMin}
							<line
								x1={1e3 * circumcenters[river[i][0] * 2]}
								y1={1e3 * circumcenters[river[i][0] * 2 + 1]}
								x2={1e3 * circumcenters[river[i + 1][0] * 2]}
								y2={1e3 * circumcenters[river[i + 1][0] * 2 + 1]}
								stroke-width={Math.log(river[i+1][1] - riverMin) / 2}
							/>
							<!-- stroke-width={Math.log(river[i+1][1] - riverMin + 1) / 2} -->
						{/if}
					{/each}
				</g>
			{/each}
		</g>


		<g class="coast" class:active={coastLines.length > 0}>
			{#each coastLines as coast}
				<path
					d={svgRender(coast)}
				/>
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
			{#each voronoiAdjacency[circumcenterIndex] as neighbor}
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
		<g class="vertices">
			<!-- {#each Array(points.length / 2).fill() as _, i}
				<rect
					x={points[i * 2 + 0] * 1000 - 1.5}
					y={points[i * 2 + 1] * 1000 - 1.5}
					width="3"
					height="3"
					fill="none"
					stroke="cyan"
				/>
			{/each} -->

			<!-- {#each nodes as node}
				<circle
					cx={node[0] * 1000}
					cy={node[1] * 1000}
					r="1.5"
				/>
			{/each} -->
		</g>
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
	display:none;
	transition: opacity 200ms;
}

canvas.visible {
	opacity: .95;
	display: block;
}

.cells path {
	fill: none;
}

.triangles path {
	stroke-width: 1;
}

.triangles, .coast, .river {
	opacity: 0;
	transition: opacity 500ms;
}

.active {
	opacity: 1;
}

.coast path,
.river path {
	stroke-linecap: round;
	stroke-linejoin: round;
}

.river path,
.river line {
	stroke: rgb(13, 133, 193);
// 	// stroke-width: 2;
}

.coast path {
	stroke: #133b66;
	stroke-width: 2;
}

.slopes line {
	stroke: black;
	stroke-width: 1;
}

.control-panel {
	display: flex;
	flex-direction: row;
	align-items: center;
}

.control-panel * {
	margin-right: 1em;
}
</style>
