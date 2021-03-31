const worker = new Worker('./terrain-worker.js');

const resolveResponse = worker => new Promise(resolve => {
  function resolver (event) {
    resolve(event.data);
    worker.removeEventListener('message', resolver);
  } 
  worker.addEventListener('message', resolver);
});


async function generate ({ seed = 1234, points = 2**10, seaLevel = 0.39 }={}) {
  const response = resolveResponse(worker);
  worker.postMessage({ action: 'generate', payload: { seed }})
  return await response;
}

export default generate;