<script>
import { createEventDispatcher } from 'svelte';

export let generationOptions;

const dispatch = createEventDispatcher();
async function regenerate () {
  dispatch('regenerate');
}

async function reseed () {
  dispatch('reseed');
}

async function reseedAndRegenerate () {
    reseed();
    regenerate();
}


let reseedHover = false;
function trackHoverState (node) {
    function hovering () {
        reseedHover = true;
    }

    function notHovering () {
        reseedHover = false;
    }

    node.addEventListener('pointerover', hovering)
    node.addEventListener('focus', hovering)
    node.addEventListener('pointerleave', notHovering)

    return () => {
        node.removeEventListener('pointerover', hovering)
        node.removeEventListener('focus', hovering)
        node.removeEventListener('pointerleave', notHovering)
    }
}
</script>


<section>
    <h3>Generation options</h3>

    <div class="regenerate" class:reseed-hover={reseedHover}>
        <button on:click={regenerate}>Regenerate</button>
        <button on:click={reseedAndRegenerate} use:trackHoverState>... with new seed</button>
    </div>


    <label for="seed">Seed</label>
    <input id="seed" type="number" bind:value={generationOptions.seed}>
    <label for="seed">{generationOptions.seed}</label>

    <label for="points">Points</label>
    <input id="points" type="range" min="250" max="7000" step="50" bind:value={generationOptions.points}>
    <label for="points">{generationOptions.points}</label>

    <label for="sea-level">Sea level</label>
    <input id="sea-level" type="range" min="0" max="1" step="0.01" bind:value={generationOptions.seaLevel}>
    <label for="sea-level">{generationOptions.seaLevel}</label>
</section>


<style>
.regenerate {
    grid-column: 1 / -1;
    display: grid;
    grid-auto-flow: column;
    grid-auto-columns: 1fr;
}

section button {
    border: 1px solid transparent;
}

button:first-child {
    border-radius: 4px 0 0 4px;
}

button:last-child {
    border-radius: 0 4px 4px 0;
}

button:hover,
button:focus,
button:active {
    border: 1px solid white;
}

.reseed-hover button {
    border: 1px solid transparent;
    border-top: 1px solid white;
    border-bottom: 1px solid white;
    background-color: hsla(0, 0%, 100%, 0.2);

}

.reseed-hover button:last-child {
    border-right: 1px solid white;
}

.reseed-hover button:first-child {
    border-left: 1px solid white;
}
</style>