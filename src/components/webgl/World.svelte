<script>
import Camera from './Camera.svelte';

import initDraw from './draw.js';

export let canvas;
export let width;
export let height;

// world
export let seaLevel;
export let points;
export let circumcenters;
export let coastLines;
export let rivers;
export let cellHeights;
export let heights;
export let voronoiTriangles;
export let renderOptions;


$: triangles = Array(voronoiTriangles.length / 3)
  .fill()
  .map((_, i) => i * 3)
  .map(j => [voronoiTriangles[j + 0], voronoiTriangles[j + 1], voronoiTriangles[j + 2]]);

$: getPointFrom = points => i => [points[2 * i], points[2 * i + 1]];
$: getEdgeCoordinates = getPointFrom(circumcenters);
$: coastLines = coastLines.map(d => d.map(getEdgeCoordinates));


let camera;
$: draw = initDraw({ canvas, width, height }, triangles, points, circumcenters, seaLevel, coastLines, rivers, cellHeights, heights);
$: window.requestAnimationFrame(() => draw({ settings: renderOptions }));
$: if (camera) window.requestAnimationFrame(() => draw({ camera: $camera }));
</script>

<Camera {canvas} bind:camera />
