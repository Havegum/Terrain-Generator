import wasm from './terrain_generator/Cargo.toml';

// import { makeRandomLanguage, makeName } from './language.js';
import { min, max, mean, maxIndex, minIndex } from 'd3-array';

function getPointFrom (points) {
  return i => [points[2 * i], points[2 * i + 1]];
}

function fillBasins (heights, adjacent, seaLevel) {
  // Find all the bodies of water
  let start = 0;
  let frontier = [ start ];
  let queued = [ true ]
  let visited = [];
  let lakes = [];


  while (frontier.length > 0) {
    let i = frontier[0];
    frontier.splice(0, 1);
    if (visited[i]) continue;

    if (heights[i] < seaLevel) {
      let lake = [ i ];
      let lakeFrontier = adjacent[i];

      while (lakeFrontier.length > 0) {
        let i = lakeFrontier[0];
        lakeFrontier.splice(0, 1);
        if (visited[i]) continue;

        let sea = heights[i] < seaLevel;
        if (sea) {
          lake.push(i);
        }

        let selectedFrontier = sea ? lakeFrontier : frontier;
        for (let neighbor of adjacent[i]) {
          if (!visited[neighbor] && !queued[neighbor]) {
            selectedFrontier.push(neighbor);
            queued[neighbor] = true;
          }
        }
        visited[i] = true;
      }
      lakes.push(lake);
    }

    for (let neighbor of adjacent[i]) {
      if (!visited[neighbor] && !queued[neighbor]) {
        frontier.push(neighbor);
        queued[neighbor] = true;
      }
    }
    visited[i] = true;
  }

  // Select the largest body of water
  let l = maxIndex(lakes, lake => lake.length);

  lakes.splice(l, 1);
  for (let lake of lakes) {
    for (let i of lake) {
      heights[i] = seaLevel;
    }
  }

  // check if main water body touches edges
  // find all other bodies of water
  // fill to seaLevel if they or the main body doesn't touch the edge of the map

  // TODO: find COAST PATHS
  // Depth-first search for nodes which didn't come from activenode index
  // If no node left, we found edge. reverse current array, go back to start and look the other way.
  //
  return heights;
}


function plateau (circumcenters, heights) {
  const plateauStart = 0.45 // mean(heights);
  const plateauCap = (1 - plateauStart) / 4; // REVIEW: 4 is magic here.

  const peakIndex = maxIndex(heights);
  const peakX = circumcenters[peakIndex * 2 + 0];
  const peakY = circumcenters[peakIndex * 2 + 1];

  function interpolate (i) {
    return plateauStart + (1 - (1 - (i - plateauStart) / (1 - plateauStart))**2) * plateauCap;
  }

  heights = heights.slice();
  for (var i = 0; i < heights.length; i++) {
    let height = heights[i];
    if (height < plateauStart) continue;

    let x = circumcenters[i * 2 + 0];
    let y = circumcenters[i * 2 + 1];

    // 1.5 is slighty more than sqrt(2), which – because maths – means we're
    // capping the distance at a bit more than half the size of the map
    // Then we're dividing by 1.5 to get [0, 1] range
    let distanceToPeak = Math.min(0.5, Math.sqrt((x - peakX)**2 + (y - peakY)**2)) / 0.5;
    distanceToPeak = distanceToPeak ** 2; // SmoothStart

    // Height is the sum of identity and interpolation, weighted by distance
    heights[i] = (1 - distanceToPeak) * height + (distanceToPeak) * interpolate(height);
  }

  return heights;
}


function erode (oldHeights, adjacent, seaLevel) {
  let heights = fillSinks(oldHeights, adjacent, seaLevel);

  let flux = getFlux(heights, adjacent);
  let n = heights.length;

  let erosionRate = 0.0125;
  // let fluxExponent = 1100 + 0.048 * n;
  let fluxExponent = 1e3;

  heights = heights.map((height, i) => {
    let underwaterDiscount = height < seaLevel ? 1e4 ** (height - seaLevel) : 1;
    let pointFlux = 1 - (1 - flux[i] / flux.length) ** fluxExponent;
    let newHeight = height - pointFlux * erosionRate * pointFlux * underwaterDiscount;
    return newHeight;
  });

  return heights;
}


function getFlux (heights, adjacent) {
  let flux = new Uint8ClampedArray(heights.length);
  let sorted = heights
    .map((height, i) => ({ height, i }))
    .sort((a, b) => Math.sign(b.height - a.height));

  // find downhill for each point.
  for (let k = 0; k < sorted.length; k++) {
    const { height, i } = sorted[k];
    let neighbors = adjacent[i];
    let low = minIndex(neighbors, n => heights[n]);
    if (heights[neighbors[low]] > height) continue;
    flux[neighbors[low]] += 1 + flux[i];
  }

  return flux;
}


function fillSinks (heights, adjacent, seaLevel, epsilon=1e-5) {
  // Mewo implementation details: https://mewo2.com/notes/terrain/
  // Original paper: https://horizon.documentation.ird.fr/exl-doc/pleins_textes/pleins_textes_7/sous_copyright/010031925.pdf

  // All non-water tiles start with infinite heights.
  let newHeights = heights.map(height => height > seaLevel ? Infinity : height);

  // ascending
  let sorted = heights
    .map((height, i) => ({ height, i }))
    .sort((a, b) => Math.sign(a.height - b.height));

  let changed = true;
  while (changed) {
    console.log("sinkfill");
    changed = false;

    for (let k = 0; k < sorted.length; k++) {
      const { height, i } = sorted[k];
      if (newHeights[i] === height) continue;

      let neighbors = adjacent[i];
      for (let n = 0; n < neighbors.length; n++) {
        let otherHeight = newHeights[neighbors[n]] + epsilon;

        if (height >= otherHeight) {
          newHeights[i] = height;
          changed = true;
          break;
        }

        if (newHeights[i] > otherHeight && otherHeight > height) {
          newHeights[i] = otherHeight;
          changed = true;
        }
      }
    }
  }

  return newHeights;
}


function getCellHeight (heights, voronoiPoints) {
  return (_, i) => {
    const points = voronoiPoints[i];
    return points.reduce((a, b) => a + heights[b], 0) / points.length
  }
}


function triangulateHeight (cellHeights, heights, triangles, seaLevel) {
  return (_, i) => {
    const j = i * 3;
    const centerHeight = cellHeights[triangles[j + 0]];
    const height1      =     heights[triangles[j + 1]];
    const height2      =     heights[triangles[j + 2]];

    let mean = (centerHeight + height1 + height2) / 3;
    const borderingSea = centerHeight > seaLevel !== height1 > seaLevel || height1 > seaLevel !== height2 > seaLevel
    // If the triangle is bordering sea we choose to set the value to be
    // what the voronoi height is so land/sea borders are always around the
    // circumference of the voronoi cells.
    if (borderingSea) {
      mean = centerHeight >= seaLevel
        ? Math.max(seaLevel + 1e-3, mean)
        : Math.min(seaLevel - 1e-3, mean);
    }

    return mean;
  }
}

function getCoastCells (heights, neighbors, seaLevel) {
  let coasts = new Array();

  for (let i = 0; i < heights.length; i++) {
    if (heights[i] >= seaLevel) {
      if (neighbors[i].some(n => heights[n] < seaLevel)) {
        coasts.push(i);
      }
    }
  }
  return coasts;
}

function getCoastLines (coastCells, seaLevel, voronoiPoints, cellLookup, cellHeights) {
  let coasts = new Array();

  for (let cellIndex = 0; cellIndex < coastCells.length; cellIndex++) {
    let points = voronoiPoints[coastCells[cellIndex]];
    let prev = points[points.length - 1];
    let prevIsBorder = cellLookup[prev].some(cell => cellHeights[cell] < seaLevel);

    for (let i = 0; i < points.length; i++) {
      let point = points[i];
      let isBorder = cellLookup[point].some(cell => cellHeights[cell] < seaLevel);

      if (isBorder && prevIsBorder) {
        coasts.push([point, prev]);
      }

      prev = point;
      prevIsBorder = isBorder;
    }
  }

  return coasts;
}


function getRivers (heights, adjacent, seaLevel, voronoiCells, cellHeights) {
  // IDEA: Use depth-first search instead. Start from sea and climb with flux
  // Create instances of `River`: "Notable" rivers that might have their own name.
  // While climbing, choose the one with highest flux such that rivers are only ever a single line

  let rivers = new Array(heights.length).fill();

  let flux = getFlux(heights, adjacent);
  function findSlope ({ i, flux })  {
    let low = minIndex(adjacent[i], n => heights[n]);

    if (heights[adjacent[i][low]] > heights[i]) {
      return { points: [i, i], flux, bad: true };
    }
    return { points: [i, adjacent[i][low]], flux };
  }

  rivers = rivers
    .map((_, i) => ({ flux: flux[i], i }))
    .filter((_, i) => voronoiCells[i].filter(c => cellHeights[c] >= seaLevel).length > 1)
    .map(findSlope)
    .filter(o => !o.bad);

  return rivers;
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
        this.voronoiGen = result.get_voronoi;
        resolve(true);
      }).catch(reject)
    );
  }

  // async fractalNoise (x, y) {
  //   await this.wasm;
  //   return this.terrainGen.fractal_noise(x, y);
  // }

  // async noiseHeight (x, y, { min=-1, max=1 }={}) {
  //   await this.wasm;
  //   return this.terrainGen.noise_single(x, y);
  // }

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
    let voronoiGen = this.voronoiGen;

    let world = {};
    world.heightMap = this.generateHeightmap

    let radius = Math.pow(500 / points, 0.5) / 10;

    let { voronoi: rustVoronoi, heights, cellHeights } = terrainGen.world(radius, seaLevel, extent.width, extent.height);

    world.heights = heights;
    world.cellHeights = cellHeights;
    world.points                                    = rustVoronoi.delaunay.points;
    let circumcenters      = world.circumcenters    = rustVoronoi.circumcenters;
    let voronoiAdjacency   = world.voronoiAdjacency = rustVoronoi.adjacent;
    let voronoiTriangles   = world.voronoiTriangles = rustVoronoi.voronoi_triangles;
    let voronoiPoints      = world.voronoiPoints    = rustVoronoi.voronoi_points;
    let voronoiCellsLookup                          = rustVoronoi.voronoi_cells;
    let neighbors /*  ~  a e s t h e t i c s  ~  */ = rustVoronoi.delaunay.neighbors;

    // Calculate triangle heights (such that each triangle is over/under seaLevel as its respective cell)
    world.triangleHeights = Array(voronoiTriangles.length / 3).fill()
        .map(triangulateHeight(world.cellHeights, world.heights, voronoiTriangles, seaLevel));

    world.isLand = world.triangleHeights.map(height => height >= seaLevel);

    const getEdgeCoordinates = getPointFrom(circumcenters);
    world.coastCells = getCoastCells(world.cellHeights, neighbors, seaLevel);
    world.coastLines = getCoastLines(world.coastCells, seaLevel, voronoiPoints, voronoiCellsLookup, world.cellHeights)
      .map(d => d.map(getEdgeCoordinates))

    world.rivers = getRivers(world.heights, voronoiAdjacency, seaLevel, voronoiCellsLookup, world.cellHeights)
      .map(river => {
        let [a, b] = river.points;
        river.points[0] = getEdgeCoordinates(a);
        river.points[1] = getEdgeCoordinates(b);
        return river;
      });

    // TODO: find which edge is facing water and record those points?
    // IDEA: Talk along the edge of one coast cell, removing indices as they're visited
    // in order to draw a single continuous path with no duplicate points
    // End cases:
    // No more neighbors: end
    // Back at start: end
    // More than one coastal neighbor cell: choose first ??


    if (yieldHeights) return world;
  }
}

export { svgRender, TerrainGenerator }
