<script>
import { writable } from 'svelte/store';
import { spring } from 'svelte/motion';

import Camera from './Camera.svelte';

import draw from './draw.js';

export let canvas;

// world
export let seaLevel;
export let points;
export let circumcenters;
export let coastLines;
export let rivers;
export let cellHeights;
export let heights;
export let voronoiTriangles;
// console.log('World.svelte props:', Object.keys($$props));


const triangles = Array(voronoiTriangles.length / 3)
  .fill()
  .map((_, i) => i * 3)
  .map(j => [voronoiTriangles[j + 0], voronoiTriangles[j + 1], voronoiTriangles[j + 2]]);

const getPointFrom = points => i => [points[2 * i], points[2 * i + 1]];
const getEdgeCoordinates = getPointFrom(circumcenters);
coastLines = coastLines.map(d => d.map(getEdgeCoordinates));


let camera;
const { setCamera } = draw(canvas, triangles, points, circumcenters, seaLevel, coastLines, rivers, cellHeights, heights);
$: if (camera) window.requestAnimationFrame(() => setCamera($camera));
</script>

<Camera bind:camera />
