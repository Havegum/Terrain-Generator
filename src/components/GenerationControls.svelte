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

let rounds = 1;
let turns = 1;


async function playRounds () {
    dispatch('playRounds', rounds);
}

async function undoRounds () {
    dispatch('undoRounds', rounds);
}

async function playTurns () {
    dispatch('playTurns', turns);
}

async function undoTurns () {
    dispatch('undoTurns', turns);
}


async function reseedAndRegenerate () {
    reseed();
    regenerate();
}


let reseedHover = false;
function trackHoverState (node) {
    function hovering () { reseedHover = true }
    function notHovering () { reseedHover = false }

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
    <h3>Generation</h3>

    <div class="regenerate" class:reseed-hover={reseedHover}>
        <button on:click={regenerate}>Regenerate</button>
        <button on:click={reseedAndRegenerate} use:trackHoverState>... with new seed</button>
    </div>


    <label for="seed" class="col-1">Seed</label>
    <input class="span" id="seed" type="number" bind:value={generationOptions.seed}>
    <!-- <label for="seed">{generationOptions.seed}</label> -->
    
    <hr/>
    <h4>Terrain generation</h4>
    <label for="points" class="col-1">Points</label>
    <input id="points" type="range" min="250" max="7000" step="50" bind:value={generationOptions.points}>
    <label for="points" class="col-3">{generationOptions.points}</label>

    <label for="sea-level" class="col-1">Sea level</label>
    <input id="sea-level" type="range" min="0" max="1" step="0.01" bind:value={generationOptions.seaLevel}>
    <label for="sea-level" class="col-3">{generationOptions.seaLevel}</label>

    <hr/>
    <h4>History generation</h4>
    <div>
        <h5>Rounds</h5>
        <input type="number" bind:value={rounds} />
        <button on:click={undoRounds}>Undo {rounds} rounds</button>
        <button on:click={playRounds}>Play {rounds} rounds</button>
    </div>
    <div>
        <h5>Turns</h5>
        <input type="number" bind:value={turns} />
        <button on:click={undoTurns}>Undo {turns} turns</button>
        <button on:click={playTurns}>Play {turns} turns</button>
    </div>
</section>


<style>
.regenerate {
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

.span {
    /* grid-column-end: span 2; */
    background-color: transparent;
    border: none;
    border-bottom: 1px solid #888;
    padding: 2px 0;
    color: inherit;
    font-size: inherit;
    -moz-appearance: textfield;
    outline: none;
}

.span:focus,
.span:active {
    border-color: white;
}

.span::-webkit-outer-spin-button,
.span::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
}
</style>