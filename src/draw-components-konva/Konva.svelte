<script>
import Konva from 'konva/lib/Core';
import { onMount, setContext } from 'svelte';
import { writable } from 'svelte/store';

let container;
let stage = writable(null);
let layer = writable(null);
setContext('konva', { stage, layer });

onMount(() => {
  stage.set(new Konva.Stage({
    container,
    width: 500,
    height: 500,
  }));

  layer.set(new Konva.Layer());

  $stage.add($layer);
});
</script>


<div bind:this={container}>
  {#if $stage}
    <slot/>
  {/if}
</div>
