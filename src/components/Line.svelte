<script>
import { getContext } from 'svelte';
const { context } = getContext('canvas');

export let x1;
export let y1;
export let x2;
export let y2;

$: if ($context) {
  const ctx = $context;

  ctx.beginPath();
  ctx.moveTo(x1, y1);
  ctx.lineTo(x2, y2);
  ctx.stroke();
  ctx.closePath();
}
</script>
