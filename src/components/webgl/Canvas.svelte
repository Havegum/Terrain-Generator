<script>
let canvas, width, height, timeout, w, h;

$: timeoutSizeUpdates(w, h);

function timeoutSizeUpdates (w, h)  {
  if (!width) width = w;
  if (!height) height = h;

  clearTimeout(timeout);
  timeout = setTimeout(function () {
    width = w;
    height = h;
  }, 150);
}
</script>


<div
  class="sizer"
  bind:clientWidth={w}
  bind:clientHeight={h}
>
  <canvas
    bind:this={canvas}
    width="{width}px"
    height="{height}px"
  />
</div>


{#if canvas}
  <slot {canvas} {width} {height} />
{/if}


<style>
.sizer {
  position: absolute;
  top: 0;
  left: 0;
}

canvas, .sizer {
  height: 100%;
  width: 100%;
}
canvas {
  cursor: crosshair;
  object-fit: cover;
}
</style>
