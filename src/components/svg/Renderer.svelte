<script>
import { deviation } from 'd3-array';
export let world;
export let renderOptions;

$: console.log(world)
// console.log(world);
$: sqrtPointCount = Math.sqrt(world.points.length);

function enumerate (n) {
	return (new Array(n)).fill();
}

function renderCellPath (i) {
	const path = world.voronoiPoints[i]
		.map(t => world.circumcenters[t * 2] + ',' + (1 - world.circumcenters[t * 2 + 1]))
		.join('L');
	return `M ${path} Z`;
}

function interpolateColor (c1, c2, i) {
	c1 = c1.replace('#', '');
	c2 = c2.replace('#', '');
	const r1 = parseInt(c1.slice(0, 2), 16);
	const g1 = parseInt(c1.slice(2, 4), 16);
	const b1 = parseInt(c1.slice(4, 6), 16);
	const r2 = parseInt(c2.slice(0, 2), 16);
	const g2 = parseInt(c2.slice(2, 4), 16);
	const b2 = parseInt(c2.slice(4, 6), 16);

	const r = (i * r1 + (1 - i) * r2).toString(16).replace(/\..+$/, '').padStart(2, '0');
	const g = (i * g1 + (1 - i) * g2).toString(16).replace(/\..+$/, '').padStart(2, '0');
	const b = (i * b1 + (1 - i) * b2).toString(16).replace(/\..+$/, '').padStart(2, '0');

	return '#' + r + g + b;
} 

function colorCell (i) {
	const sea = world.seaLevel;
	const h = world.cellHeights[i];
	if (h > sea) {
			const t = (h - sea) / (1 - sea);
			let c = interpolateColor(renderOptions.hillColor, renderOptions.landColor, t);
			const min = 1 / sqrtPointCount;
			const max = 2 / sqrtPointCount;
			const stddev = Math.min(max, deviation(world.voronoiPoints[i], i => world.heights[i]));
			if (stddev < min) return c;
			const u = (stddev - min) / (max - min);
			return interpolateColor(renderOptions.cliffColor, c, u * 0.75);
	} else {
			const t = h / sea;
			return interpolateColor(renderOptions.waterColor, renderOptions.depthColor, t);
	}
}

function renderCoast(world) {
	const { coastLines, circumcenters: c } = world;

	const point = p => c[p * 2] + ',' + (1 - c[p * 2 + 1]);

	let path = coastLines.map(([a, b]) => point(a) + 'L' + point(b)).join('M    ')
	return `M ${path} Z`; 
}

function point (t) {
	const x = world.points[t * 2];
	const y = 1 - world.points[t * 2 + 1];
	const e = 5e-3;
	return { x, y, e };
}

$: indices = enumerate(world.points.length / 2);
</script>


<svg viewBox="-0.1 -0.1 1.2 1.2">
	<g class="cells">
		{#each indices as _, i (i)}
			<path
					vector-effect="non-scaling-stroke"
					d={renderCellPath(i)} fill={colorCell(i, renderOptions)}/>
		{/each}
	</g>

	{#if renderOptions.showTerritory && world.history}
		<g class="territory">
			{#each world.history.board.turn_order.map(id => world.history.board.civs[id]) as civ}
				<g style="--c: {civ.color}">
					{#each civ.territory as i}
						<path d={renderCellPath(i)} vector-effect="non-scaling-stroke" />
					{/each}
					{#each civ.neighbor_territory.map(point) as { x, y, e }}
						<circle
							cx={x}
							cy={y}
							r={e}
							stroke-width="1px"
							class="territory-neighbor"
							vector-effect="non-scaling-stroke"
						/>
					{/each}
				</g>
			{/each}
		</g>
	{/if}
			
	<!-- <g class="cell-centroids">
		{#each indices as _, i (i)}
			<line
				vector-effect="non-scaling-stroke"
				x1={world.points[i * 2]}
				y1={1e-7 + 1 - world.points[i * 2 + 1]}
				x2={world.points[i * 2]}
				y2={-1e-7 + 1 - world.points[i * 2 + 1]}/>
		{/each}
	</g> -->

	<path
		class="coast"
		vector-effect="non-scaling-stroke"
		d={renderCoast(world)}/>

	<rect
		class="bounding-box"
		vector-effect="non-scaling-stroke"
		width="1"
		height="1"/>
</svg>


<style>
svg {
    width: 100%;
    height: 100%;
    position: absolute;
    top: 0;
    left: 0;
}

.bounding-box {
    fill: none;
    stroke: red;
}


.cells path {
    stroke: black;
    stroke-width: 0.125px;
}

/* .cell-centroids line {
    shape-rendering: optimizeSpeed;
    stroke-width: 2px;
    stroke-linecap: square;
    fill: none;
    stroke: black;
} */

.coast {
    fill: none;
    stroke: #133b66;
    stroke-width: 3px;
    stroke-linecap: round;
}

.territory path {
    fill: var(--c);
    stroke: var(--c);
    stroke-width: 2px;
    fill-opacity: .5;
}

.territory-neighbor {
	fill: none;
	stroke: var(--c);
	stroke-width: 1.5px;
}
</style>