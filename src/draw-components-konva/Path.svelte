<script>
import { Path } from 'konva/lib/shapes/Path';
import { getContext, onDestroy } from 'svelte';

const layer = getContext('layer');
const group = getContext('group');
const target = group || layer;

export let coordinates = [];
export let stroke = 'black';
export let strokeWidth = 1;

const path = new Path({
  data: toSvgDataString(coordinates),
  fill: 'none',
  stroke,
  strokeWidth,
});

function toSvgDataString(coordinates) {
  return 'M' + coordinates.map(pair => pair.join(',')).join('L') + 'Z';
}



if (target) target.add(path);
onDestroy(() => path.destroy());
</script>
