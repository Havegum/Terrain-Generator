<script>
import { writable } from 'svelte/store';
import { spring } from 'svelte/motion';
const isFirefox = navigator.userAgent.search('Firefox') > -1;

export let canvas;
export let camera = writable({
  zRot: 0,
  yRot: 0,
  dist: 2,
  x: 0,
  y: 0,
});

let mouseDown = false;
let originalX = null;
let originalY = null;

let focus = spring({ x: 0, y: 0 }, {
  stiffness: 0.07,
  damping: 0.7,
  precision: 0.0001,
});

$: {
  $camera.x = $focus.x;
  $camera.y = $focus.y;
};

let zoom = spring(1.7, {
  stiffness: 0.19,
  damping: 1,
  precision: 0.001,
});

$: $camera.dist = $zoom;

const minStep = .01;
const pollRate = Math.round(1000 / 24);
let stepInterval;
const activeKeys = {};

$: Object.keys(activeKeys).length && tickMovement();

async function tickMovement () {
  const boost = activeKeys['shift'] ? 1.8 : 1;
  const moveStep = (minStep + $camera.dist / 25) * boost;
  let z = $camera.zRot;
  let x = $focus.x;
  let y = $focus.y;

  let forward = 0;
  let right = 0;

  if (activeKeys['w']) forward += 1;
  if (activeKeys['s']) forward -= 1;
  if (activeKeys['a']) right -= 1;
  if (activeKeys['d']) right += 1;

  const length = Math.hypot(forward, right);
  if (length !== 0) {
    forward /= length;
    right /= length;
  }

  x += forward * Math.sin(z) * moveStep;
  y += forward * Math.cos(z) * moveStep;
  x +=   right * Math.sin(z + Math.PI / 2) * moveStep;
  y +=   right * Math.cos(z + Math.PI / 2) * moveStep;

  focus.set({ x, y });
  if (!stepInterval) stepInterval = setInterval(tickMovement, pollRate);
}

function handleKeyDown ({ key }) {
  key = key.toLowerCase();
  if (!activeKeys[key]) activeKeys[key] = true;
}

function handleKeyUp ({ key }) {
  delete activeKeys[key.toLowerCase()];
  const hasInterval = stepInterval && Object.keys(activeKeys).length === 0;
  if (hasInterval) stepInterval = clearInterval(stepInterval);
}

function handleMouseMove (event) {
  if (!mouseDown) return;
  const x = event.clientX - originalX;
  const y = event.clientY - originalY;

  $camera.zRot += x * 2e-3;
  $camera.yRot = Math.min(0, Math.max(Math.PI / -2, $camera.yRot + y * 2e-3));

  originalX = event.clientX;
  originalY = event.clientY;
}

function handleMouseDown (event) {
  if (event.target !== canvas) return;
  originalX = event.clientX;
  originalY = event.clientY;
  mouseDown = true;
}

function handleMouseUp (event) {
  mouseDown = false;
}

function handleScroll (event) {
  $camera.dist = Math.max(0, $camera.dist + event.deltaY * 4e-2);
  zoom.set(Math.max(0, $zoom + event.deltaY * (isFirefox ? 8e-2 : 2e-3)));
}

</script>

<svelte:window
  on:keydown={handleKeyDown}
  on:keyup={handleKeyUp}
  on:mousedown={handleMouseDown}
  on:mouseup={handleMouseUp}
  on:mousemove={handleMouseMove}
  on:wheel={handleScroll}
/>
