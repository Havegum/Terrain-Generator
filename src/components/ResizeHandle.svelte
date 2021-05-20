<script>
export let viewportSize = 0.8;
export let min = 0.2;
export let max = 0.9;



function handleResize () {
  const height = document.body.clientHeight;
  function resizeViewport (e) {
    viewportSize = Math.max(min, Math.min(max, e.clientY / height));
  }
  const remove = () => window.removeEventListener('pointermove', resizeViewport);
  window.addEventListener('pointermove', resizeViewport);
  window.addEventListener('pointerup', remove, { once: true })
}
</script>


<label for="resizer" class="resizer" draggable on:pointerdown={handleResize}>
  <input id="resizer" type="range" min="0.2" max="0.8" step="0.001" bind:value={viewportSize}/>
</label>


<style>
.resizer {
  height: 1px;
  padding: 0.25em 0;
  margin: -0.25em 0;
  position: relative;
  cursor: ns-resize;
}

.resizer::before {
  background-color: transparent;
  display: block;
  content: '';
  height: 1px;
  position: relative;
  bottom: 2px;
  border-top:    1px solid transparent;
  border-bottom: 1px solid transparent;
}

.resizer:hover::before,
.resizer:focus::before,
.resizer:active::before {
  background-color: #212121;
  border-color: #5f6163;
}

.resizer input {
  visibility: hidden;
}
</style>