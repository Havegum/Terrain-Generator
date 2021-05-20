<script>
import generate from './utils/terrain.js';
import Canvas from './components/webgl/Canvas.svelte';
import Controls from './components/Controls.svelte';
import RenderControls from './components/RenderControls.svelte';
import GenerationControls from './components/GenerationControls.svelte';

import RenderFrame from './components/RenderFrame.svelte';
import StaleOverlay from './components/StaleOverlay.svelte';
import ResizeHandle from './components/ResizeHandle.svelte';
import WebglRenderer from './components/webgl/World.svelte';
import SvgRenderer from './components/svg/Renderer.svelte';

import HISTORY_WASM from '@/wasm/history_generator/Cargo.toml';
const historyGenerator = HISTORY_WASM();
let history;

const rng = () => Math.floor(Math.random() * 1e8);
const seaLevel = process.env.SEA_LEVEL || 0.39;
const points = process.env.WORLD_POINTS || 2 ** 10;
const seed = process.env.SEED || rng();

const renderers = ['svg', 'webgl'];
let renderer = 'svg';


let generationOptions = {
  seaLevel,
  points,
  seed,
};

let renderOptions = {
  webgl: {
    riverCap: 80,
  },
  svg: {
    hillColor: '#e8ffcd',
    cliffColor: '#2f2b29',
    landColor: '#338a3e',
    waterColor: '#11acc6',
    depthColor: '#041a2d',
    showTerritory: true,
  }
};

let world, stale;
async function gen () {
  stale = true;
  // seed = 15043459; // DEBUG THIS ONE
  console.log('seed:', generationOptions.seed);
  world = await generate(generationOptions);

  const adjacencies = world.neighbors;

  const worldArg = {
    adjacencies,
    heights: world.cellHeights,
    seaLevel: world.seaLevel
  };

  history = await historyGenerator
    .then(({ Simulation }) => new Simulation(worldArg, 1234, 4))
    .then(sim => sim);
    
  
  world.history = history.as_js_value();

  stale = false;
}

async function playRounds ({ detail: rounds }) {
  if (!history) return;
  history = history.playRounds(rounds);
  world.history = history.as_js_value();
}

async function undoRounds ({ detail: rounds }) {
  if (!history) return;
  history = history.revertRounds(rounds);
  world.history = history.as_js_value();
}

async function playTurns ({ detail: turns }) {
  if (!history) return;
  history = history.playTurns(turns);
  world.history = history.as_js_value();
}

async function undoTurns ({ detail: turns }) {
  if (!history) return;
  history = history.revertTurns(turns);
  world.history = history.as_js_value();
}

gen();


function keyboardShortcuts (e) {
  if (e.key === 'Tab') {
    e.preventDefault();
    const i = renderers.indexOf(renderer);
    renderer = renderers[(i + 1) % renderers.length];
  }
}

let viewportSize = 0.8;
</script>


<svelte:window
  on:keydown={keyboardShortcuts}
/>

<div class="app" style="--viewport-size: {viewportSize * 100}%">
  <RenderFrame>
    {#if world}
      {#if renderer === 'webgl'}
        <Canvas let:canvas let:width let:height >
          <WebglRenderer {canvas} {width} {height} {...world} renderOptions={renderOptions.webgl} />
        </Canvas>
      {:else if renderer === 'svg'}
        <SvgRenderer {world} renderOptions={renderOptions.svg} />
      {/if}
    {/if}
    <StaleOverlay {stale}/>
  </RenderFrame>

  <ResizeHandle bind:viewportSize min={0.2} max={0.9} />

  <Controls>
    <RenderControls bind:renderer bind:renderOptions />
    <GenerationControls
      bind:generationOptions
      on:reseed={() => generationOptions.seed = rng()}
      on:regenerate={gen}
      on:playRounds={playRounds}
      on:undoRounds={undoRounds}
      on:playTurns={playTurns}
      on:undoTurns={undoTurns}
    />
  </Controls>
</div>


<style>
.app {
  display: grid;
  height: 100vh;
  grid-template-rows: var(--viewport-size) auto 1fr;
}
</style>