import WASM from '@/wasm/terrain_generator/Cargo.toml';

const wasm = WASM();

class TerrainGenerator {
  constructor (seed=123456) {
    this.wasm = new Promise((resolve, reject) => wasm
      .then(result => {
        this.terrainGen = new result.TerrainGenerator(seed);
        resolve(true);
      }).catch(reject)
    );
  }

  async generate ({ points = 2**10, seaLevel = 0.39 }={}) {
    await this.wasm;

    let radius = Math.pow(500 / points, 0.5) / 10;
    const world = this.terrainGen.world(radius, seaLevel).as_js_value();
    world.seaLevel = seaLevel;
    world.points           = world.voronoi.delaunay.points;
    world.circumcenters    = world.voronoi.circumcenters;
    world.voronoiAdjacency = world.voronoi.adjacent;
    world.voronoiTriangles = world.voronoi.voronoi_triangles;
    world.voronoiPoints    = world.voronoi.voronoi_points;

    delete world.voronoi
    return world;
  }
}

addEventListener('message', async function (event) {
  const { action, payload } = event.data;
  if (action === 'generate') {
    const { seed, options } = payload;
    const generator = new TerrainGenerator(seed);
    const world = await generator.generate(options);
    postMessage(world);
  }
});