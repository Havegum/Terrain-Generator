<script>
import wasm from './terrain_generator/Cargo.toml';
let extent = { width: 1, height: 1 };
let seaLevel = 0.39;
let radius = 2e-1;

let points = [];
let circumcenters = [];
let triangles = [];

let svg, width, height;
$: viewBoxWidth = 1000 * extent.width;
$: viewBoxHeight = 1000 * extent.height;
$: viewBox = `0 0 ${viewBoxWidth} ${viewBoxHeight}`;

wasm().then(rust => {
  let terrainGen = new rust.TerrainGenerator(1234);

  points = terrainGen.poisson_disc_points(radius, seaLevel, extent.width, extent.height);
  let voronoi = rust.get_voronoi(points);
  triangles = voronoi.voronoi_triangles;
  circumcenters = voronoi.circumcenters;
  console.log(voronoi);
});

function parseTriangle (i) {
  return `
    M${points[triangles[i * 3] * 2]*1000},${points[triangles[i * 3] * 2 + 1]*1000}
    L${circumcenters[triangles[i * 3 + 1] * 2]*1000},${circumcenters[triangles[i * 3 + 1] * 2 + 1]*1000}
    L${circumcenters[triangles[i * 3 + 2] * 2]*1000},${circumcenters[triangles[i * 3 + 2] * 2 + 1]*1000}
  `;
}
</script>



<div class="wrap" bind:clientWidth={width} bind:clientHeight={height}>
	<svg bind:this={svg} {viewBox}>
    {#each Array(points.length / 2).fill() as _, i}
      <circle cx={points[i * 2] * 1000} cy={points[i * 2 + 1] * 1000} r="2"/>
      <circle cx={points[i * 2] * 1000} cy={points[i * 2 + 1] * 1000} r="4" stroke="black" fill="none"/>
    {/each}

    {#each Array(circumcenters.length / 2).fill() as _, i}
      <circle cx={circumcenters[i * 2] * 1000} cy={circumcenters[i * 2 + 1] * 1000} r="2" fill="salmon"/>
    {/each}

    {#each Array(triangles.length / 3).fill() as _, i}
      <path
        fill="none"
        stroke="#00000015"
        d={parseTriangle(i)}
      />
    {/each}
  </svg>
</div>



<style>
.wrap {
	place-items: center;
	display: grid;
	width: 100%;
}

svg {
	width: 100%;
	height: 100%;
	display: block;
	/* overflow: visible; */
}

</style>
