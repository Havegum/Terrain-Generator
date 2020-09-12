<script>
import { Path } from 'konva/lib/shapes/Path';
import { getContext, onDestroy } from 'svelte';

const layer = getContext('layer');
const group = getContext('group');
const target = group || layer;

export let coordinates = [];
export let fill = 'red';
export let stroke = 'black';
export let strokeWidth = 2;

const polygon = new Path({
  data: toSvgDataString(coordinates),
  fill,
  stroke,
  strokeWidth,
});

function toSvgDataString(coordinates) {
  return 'M' + coordinates.map(pair => pair.join(',')).join('L') + 'Z';
}

if (target) target.add(polygon);
onDestroy(() => polygon.destroy());
</script>
