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
        console.log(result);
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
    let relaxIterations = this.relaxIterations;
    let seed = this.seed;
    let yieldPoints = this.yieldPoints;
    let yieldRelax = this.yieldRelax;
    let yieldHeights = this.yieldHeights;
    await this.wasm;

    let terrainGen = this.terrainGen;

    let world = {};
    world.heightMap = this.generateHeightmap

    let radius = Math.pow(500 / points, 0.5) / 10;

    let { voronoi: rustVoronoi, heights, cellHeights, rivers, triangleHeights, coastLines } = terrainGen.world(radius, seaLevel, extent.width, extent.height);

    world.heights = heights;
    world.cellHeights = cellHeights;
    world.points                                    = rustVoronoi.delaunay.points;
    let circumcenters      = world.circumcenters    = rustVoronoi.circumcenters;
    let voronoiAdjacency   = world.voronoiAdjacency = rustVoronoi.adjacent;
    let voronoiTriangles   = world.voronoiTriangles = rustVoronoi.voronoi_triangles;
    let voronoiPoints      = world.voronoiPoints    = rustVoronoi.voronoi_points;
    let voronoiCellsLookup                          = rustVoronoi.voronoi_cells;
    let neighbors /*  ~  a e s t h e t i c s  ~  */ = rustVoronoi.delaunay.neighbors;

    world.triangleHeights = triangleHeights;

    world.isLand = world.triangleHeights.map(height => height >= seaLevel);

    const getEdgeCoordinates = getPointFrom(circumcenters);
    world.coastLines = coastLines.map(d => d.map(getEdgeCoordinates));
    world.rivers = rivers;

    if (yieldHeights) return world;
  }
}

export { svgRender, TerrainGenerator }
