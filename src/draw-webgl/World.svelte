<script>
import draw from './draw.js';

export let canvas;

// world
export let seaLevel;
export let triangles;
export let points;
export let circumcenters;
export let triangleHeights;
export let coastLines;
export let rivers;
export let cellHeights;
export let heights;
console.log('world props:', Object.keys($$props));

export let camera;

const { setCamera } = draw(canvas, triangles, points, circumcenters, triangleHeights, seaLevel, coastLines, rivers, cellHeights, heights);

$: window.requestAnimationFrame(() => setCamera($camera));
</script>
