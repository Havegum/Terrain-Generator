<script>
let canvas, width, height;
</script>


<canvas
  bind:this={canvas}
  bind:clientWidth={width}
  bind:clientHeight={height}
  {width}
  {height}
/>
{#if canvas}
  <slot {canvas} />
{/if}


<style>
canvas {
  position: absolute;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
  height: 100%;
  width: 100%;
  cursor: crosshair;
  object-fit: cover;
}
</style>
