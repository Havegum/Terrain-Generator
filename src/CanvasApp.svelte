<script>
import { onMount } from 'svelte';
import Canvas from './components/Canvas.svelte';
import Line from './components/Line.svelte';


// TODO: use Konva
// great tutorial here:
// https://geoexamples.com/svelte/2020/08/05/svelte-konva-mapping.html
import { svgRender, TerrainGenerator } from './terrain.js';
import {
  interpolateYlGn as interpolateLand,
  interpolatePuBu as interpolateSea
} from 'd3-scale-chromatic';

let canvas, width, height, generator;
let seaLevel = 0.39;
let extent = { width: 1, height: 1 };


onMount(() => {
	if (!canvas) return;
  let seed = Math.floor(Math.random() * 1e8);

  generator = new TerrainGenerator({
		points: 2**12,
		seaLevel,
		seed
	});

  generate();
});

async function generate () {

}
</script>

<Canvas>
  <Line x1="10" y1="10" x2="20" y2="30" />
</Canvas>
