<script>
import { slide } from 'svelte/transition';

export let controlSettings = {
  riverCap: 80
};

let riverCap = 80;
$: controlSettings.riverCap = riverCap;

let expanded = false;
</script>

<div class="controls" class:expanded>
  <button on:click={() => expanded = !expanded}>
    <svg viewBox="0 0 1 1" preserveAspectRatio="none">
      <path d="M0,0 L1,0"/>
      <path d="M0,0.5 L1,0.5"/>
      <path d="M0,1 L1,1"/>
    </svg>
  </button>

  {#if expanded}
    <section class="settings" transition:slide={{ duration: 200 }}>
      <h2>Settings</h2>
      <hr>
      <label for="rivers">
        <p>Minimum river flux</p>
        <input id="rivers" type="range" min="0" max="200" step="1" bind:value={riverCap}>
        <p>{riverCap}</p>
      </label>
    </section>
  {/if}
</div>

<style>
.controls {
  color: #f2f2f2;
  position: absolute;
  left: 8px;
  top: 8px;
  padding: 8px;
  background-color: #33333300;
  transition: background-color 200ms;
  width: 20em;
}

.expanded {
  background-color: #333333db;
}

@supports (backdrop-filter: blur(10px)) {
  .expanded {
    background-color: #3333337a;
    backdrop-filter: blur(10px);
  }
}

.settings {
  padding: 8px;
  display: grid;
  grid-gap: 8px;
}

button {
  background-color: transparent;
  border: none;
  color: inherit;
  display: block;
  width: 2.8em;
  height: 2.2em;
  padding: 8px;
  cursor: pointer;
  border-radius: 8px;
}

button:hover {
  background-color: #ffffff11;
}

svg {
  display: block;
  width: 100%;
  height: 100%;
  overflow: visible;
  color: currentColor;
}

h2 {
  font-size: 18px;
  font-weight: normal;
}

hr {
  border: none;
  border-top: 1px solid currentColor;
}

label {
  display: grid;
  grid-template-columns: auto 1fr 3em;
  grid-gap: 6px;
}

label p,
label input {
  display: block;
}

input {
  width: 100%;
}

button:hover path {
  opacity: 1;
}

path {
  stroke: currentColor;
  stroke-width: 3px;
  opacity: .7;
  stroke-linecap: round;
  vector-effect: non-scaling-stroke;
  transition: stroke 300ms;
}
</style>
