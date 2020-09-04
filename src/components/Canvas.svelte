<script>
import { onMount, setContext } from 'svelte';
import { writable } from 'svelte/store';

let canvas = writable(null);
let context = writable(null);

setContext('canvas', { canvas, context });

function setCanvas (node) {
  canvas.set(node);
  context.set(node.getContext('2d'));
}
</script>


<canvas use:setCanvas/>
<slot />


<style>
canvas {
  width: 100%;
}
</style>
