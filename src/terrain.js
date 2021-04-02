const worker = new Worker(new URL('./terrain-worker.js', import.meta.url), {
  name: 'terrain-worker',
  type: 'module'
});


const resolveResponse = worker => new Promise(resolve => {
  function resolver (event) {
    resolve(event.data);
    worker.removeEventListener('message', resolver);
  } 
  worker.addEventListener('message', resolver);
});


async function generate ({ seed = 1234, points = 2**10, seaLevel = 0.39 }={}) {
  const response = resolveResponse(worker);
  console.log('sending to worker', worker);
  worker.postMessage({
    action: 'generate',
    payload: {
      seed,
      options: { points, seaLevel }
    }
  });
  return await response;
}

export default generate;