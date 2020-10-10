import wasm from './terrain_generator/Cargo.toml';
// import { makeRandomLanguage, makeName } from './language.js';
import { min, max, mean, maxIndex, minIndex } from 'd3-array';

function getPointFrom (points) {
  return i => [points[2 * i], points[2 * i + 1]];
}

function svgRender (d) {
	return 'M' + d.map(d => 1000 * d[0] + ',' + 1000 * d[1]).join('L') + 'Z';
}

class TerrainGenerator {
  constructor ({
      points = 2**6,
      extent = { width: 1, height: 1 },
      seaLevel = 0.3,
      relaxIterations = 1,
      seed = 123456,
      yieldPoints = false,
      yieldRelax = false,
      yieldHeights = true
    }={}
  ) {
    this.points = points;
    this.extent = extent;
    this.seaLevel = seaLevel;
    this.relaxIterations = relaxIterations;
    this.seed = seed;
    this.yieldPoints = yieldPoints;
    this.yieldRelax = yieldRelax;
    this.yieldHeights = yieldHeights;
    this.wasm = new Promise((resolve, reject) => wasm()
      .then(result => {
        this.terrainGen = new result.TerrainGenerator(seed);
        resolve(true);
      }).catch(reject)
    );
  }

  async noisyHeights (points, heights) {
    await this.wasm;
    return Array.from(this.terrainGen.heightmap(points, heights));
  }

  async generateHeightmap (size) {
    let heights = Array(size * size).fill(0);
    let points = Array(size * size * 2).fill(0);
    for (let i = 0; i < size * size; i++) {
      points[i * 2 + 0] = ((i % size) / size);// - 0.5;
      points[i * 2 + 1] = (Math.floor(i / size) / size);// - 0.5;
    }
    heights = await this.noisyHeights(points, heights);

    let pixels = new Uint8ClampedArray(size * size * 4);
    for (let i = 0; i < size * size; i++) {
      let val = heights[i];
      pixels[i * 4 + 0] = Math.floor(val * 255);
      pixels[i * 4 + 1] = Math.floor(val * 255);
      pixels[i * 4 + 2] = Math.floor(val * 255);
      pixels[i * 4 + 3] = 255;
    }
    return pixels;
  }

  async generate () {
    let points = this.points;
    let extent = this.extent;
    let seaLevel = this.seaLevel;
    let seed = this.seed;
    await this.wasm;

    let terrainGen = this.terrainGen;

    let world = {};
    world.heightMap = this.generateHeightmap

    let radius = Math.pow(500 / points, 0.5) / 10;

    let { voronoi, heights, cellHeights, rivers, triangleHeights, coastLines } = terrainGen.world(radius, seaLevel, extent.width, extent.height).as_js_value();

    world.heights = heights;
    world.points = voronoi.delaunay.points;
    world.circumcenters    = voronoi.circumcenters;
    world.voronoiAdjacency = voronoi.adjacent;
    world.voronoiTriangles = voronoi.voronoi_triangles;
    world.voronoiPoints    = voronoi.voronoi_points;
    world.triangleHeights  = triangleHeights;

    const getEdgeCoordinates = getPointFrom(world.circumcenters);
    world.coastLines = coastLines.map(d => d.map(getEdgeCoordinates));
    world.rivers = rivers;
    world.cellHeights = cellHeights;
    
    return world;
  }
}

export { svgRender, TerrainGenerator }
