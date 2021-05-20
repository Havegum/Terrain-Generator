<script>
export let stale = true;
</script>

<div class="overlay" class:stale />

<style>
.overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  background-color: hsl(216, 3%, 8%);
  opacity: 0;
  transition: opacity 250ms ease-out;
}

.stale {
  opacity: 0.5;
  cursor: progress;
  pointer-events: auto;
  transition: none;
}
</style>