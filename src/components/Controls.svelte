<script>
import { slide } from 'svelte/transition';
let expanded = false;
</script>


<div class="controls" class:expanded>
  <button class="expand" on:click={() => expanded = !expanded}>
    <svg viewBox="0 0 1 1" preserveAspectRatio="none">
      <path vector-effect="non-scaling-stroke" d="M0,0 L1,0"/>
      <path vector-effect="non-scaling-stroke" d="M0,0.5 L1,0.5"/>
      <path vector-effect="non-scaling-stroke" d="M0,1 L1,1"/>
    </svg>
  </button>

  {#if expanded}
    <section class="settings" transition:slide={{ duration: 200 }}>
      <h2>Settings</h2>
      <hr>
      <slot/>
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

.expand {
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

.expand:hover {
  background-color: #ffffff11;
}

svg {
  display: block;
  width: 100%;
  height: 100%;
  overflow: visible;
  color: currentColor;
}

.expand:hover path {
  opacity: 1;
}

path {
  stroke: currentColor;
  stroke-width: 3px;
  opacity: .7;
  stroke-linecap: round;
  transition: stroke 300ms;
}

h2 {
  font-size: 18px;
  font-weight: normal;
}

hr {
  border: none;
  border-top: 1px solid currentColor;
}

section :global(section) {
  margin-top: 8px;
  display: grid;
  grid-template-columns: auto 1fr auto;
  grid-gap: 4px 8px;

}

section :global(h3) {
  font-weight: normal;
  font-size: inherit;
  font-size: .75em;
  opacity: 0.75;
  text-transform: uppercase;
  margin-bottom: 4px;
  grid-column: 1 / -1;
}

section :global(label) {
  display: block;
}

section :global(input) {
  width: 100%;
}

.settings :global(button) {
  font-size: inherit;
  border: none;
  padding: 7px;
  border-radius: 4px;
  background-color: hsla(0, 0%, 100%, 0.1);
  color: inherit;
  cursor: pointer;
}

.settings :global(button:hover) {
  background-color: hsla(0, 0%, 100%, 0.2);
}

</style>
