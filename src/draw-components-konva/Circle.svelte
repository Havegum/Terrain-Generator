<script>
import { Circle } from 'konva/lib/shapes/Circle';
import { getContext, onDestroy } from 'svelte';

export let x = 0;
export let y = 0;

const { stage } = getContext('konva');
const layer = getContext('layer');
const group = getContext('group');
const target = group || layer;

const circle = new Circle({
  radius: 70,
  fill: 'red',
  stroke: 'black',
  strokeWidth: 4
});

if (target) target.add(circle);

$:{
  circle.position({
    x: $stage.width()  / 2 + x,
    y: $stage.height() / 2 + y,
  });
}

onDestroy(() => circle.destroy());
</script>
