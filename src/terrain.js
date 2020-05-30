import wasm from './terrain_generator/Cargo.toml';

import SimplexNoise from 'simplex-noise';
import { Delaunay } from 'd3-delaunay';
import { randomUniform } from 'd3-random';
import { makeRandomLanguage, makeName } from './language.js';
import { min, max, mean, maxIndex, minIndex } from 'd3-array';
import { scaleLinear } from 'd3-scale';
import seedrandom from 'seedrandom';

let simplex;
let simplexTheta;
let simplexLength;
let simplexResources;

let rngUniform;

function fractalNoise (x, y) {
  const octaves = 8;
  let value = 0;
  let max = 0;

  let force = 0.3;
  let theta = simplexTheta.noise2D(x * (1 - force), y * (1 - force));
  let length = simplexLength.noise2D(x * (1 - force), y * (1 - force));

  y += Math.sin(theta) * length * force;
  x += Math.cos(theta) * length * force;

  for (let i = 0; i < octaves; i++) {
    const weight = 1 << i; // [1, 2, 4, 8, ...] === 2 ** n
    value += simplex.noise2D(x * weight, y * weight) / weight;
    max += 1 / weight;
  }

  return value / max;
}

function noisyResources (points) {
// TODO:
// IDEA: ridge? simplex range?
}

const clamp = n => Math.max(0, Math.min(1, n));

function getPointFrom (points) {
  return i => [points[2 * i], points[2 * i + 1]];
}

function getVoronoiAdjacencies (voronoi) {
  // Do this for each point.
  // From https://github.com/d3/d3-delaunay/blob/ffeab8a15853a7c0a2305166376d0a36d8a9f36f/src/voronoi.js#L168
  const { circumcenters, delaunay: { inedges, halfedges, triangles } } = voronoi;
  let adjacent = []; // Voronoi circumcenters adjacencies
  let voronoiTriangles = []; // list of [index to centroid, index for p1, p2]
  let voronoiPoints = []; // voronoi cell index => Array<circumcenter index>
  let voronoiCells = []; // circumcenter index => Array<voronoi cell index>

  for (let i = 0; i < inedges.length; i++) {
    const e0 = inedges[i];
    if (e0 === -1) return null; // coincident point
    let e = e0;
    let t, previousT = null;
    voronoiPoints[i] = [];

    do {
      t = Math.floor(e / 3);

      if (!adjacent[t]) adjacent[t] = [];
      if (!voronoiCells[t]) voronoiCells[t] = [];
      voronoiCells[t].push(i);
      voronoiPoints[i].push(t);

      // Index `t` is neighbour of the previous `t`
      if (previousT !== null) {
        if (!adjacent[t].includes(previousT)) adjacent[t].push(previousT);
        if (!adjacent[previousT].includes(t)) adjacent[previousT].push(t);
        voronoiTriangles.push(i, t, previousT);
      }
      previousT = t;

      e = e % 3 === 2 ? e - 2 : e + 1;
      if (triangles[e] !== i) break; // bad triangulation
      e = halfedges[e];
    } while (e !== e0 && e !== -1);
    voronoiTriangles.push(i, Math.floor(e / 3), previousT);
  }

  // each element in the array is an index to circumcenters
  // circumcenters[t * 2], circumcenters[t * 2 + 1]
  return { voronoiAdjacency: adjacent, voronoiTriangles, voronoiPoints, voronoiCells };
}

function improvePoints (points, iterations=1, { width=1, height=1 } = {}) {
  let centroids, polygons;
  let extent = [0, 0, width, height];
  // let voronoi = Delaunay.from(points).voronoi(extent);
  let voronoi = (new Delaunay(points)).voronoi(extent);

  function coordSum (sum, next) {
    if (!sum) sum = [0, 0];
    sum[0] += next[0];
    sum[1] += next[1];
    return sum;
  }

  for (let i = 0; i < iterations; i++) {
    centroids = [...voronoi.cellPolygons()].map(hull => hull.slice(1).reduce(coordSum).map(d => d / (hull.length - 1)));
    voronoi = Delaunay.from(centroids).voronoi(extent);
    polygons = [...voronoi.cellPolygons()];
  }

  return { voronoi, delaunay: voronoi.delaunay, centroids, cells: polygons };
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

  let erosionRate = 0.01;
  let fluxExponent = 1100 + 0.048 * n;

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

  // All non-deepwater tiles start with infinite heights.
  let newHeights = heights.map(height => height > seaLevel ? Infinity : height);

  // ascending
  let sorted = heights
    .map((height, i) => ({ height, i }))
    .sort((a, b) => Math.sign(a.height - b.height));

  let changed = true;
  while (changed) {
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

function getCoastCells (heights, delaunay, seaLevel) {
  let coasts = new Array();

  for (let i = 0; i < heights.length; i++) {
    if (heights[i] >= seaLevel) {
      if ([...delaunay.neighbors(i)].some(n => heights[n] < seaLevel)) {
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
    .filter((_, i) => voronoiCells[i].every(c => cellHeights[c] >= seaLevel))
    .map(findSlope)
    .filter(o => !o.bad);

  return rivers;
}

// For heightmap displaying
// function generateHeightmap (size) {
//   let heights = Array(size * size).fill(0);
//   let points = Array(size * size * 2).fill(0);
//   for (let i = 0; i < size * size; i++) {
//     points[i * 2 + 0] = ((i % size) / size) - 0.5;
//     points[i * 2 + 1] = (Math.floor(i / size) / size) - 0.5;
//   }
//   heights = noisyHeights(points, heights);
//   heights = plateau(points, heights);
//
//   let pixels = new Uint8ClampedArray(size * size * 4);
//   for (let i = 0; i < size * size; i++) {
//     let x = i % size;
//     let y = Math.floor(i / size);
//     let val = heights[i];
//     pixels[i * 4 + 0] = Math.floor(val * 255);
//     pixels[i * 4 + 1] = Math.floor(val * 255);
//     pixels[i * 4 + 2] = Math.floor(val * 255);
//     pixels[i * 4 + 3] = 255;
//   }
//   return pixels;
// }


function generateResources (size) {
  let heights = Array(size * size).fill(0);
  let points = Array(size * size * 2).fill(0);
  for (let i = 0; i < size * size; i++) {
    points[i * 2 + 0] = ((i % size) / size) - 0.5;
    points[i * 2 + 1] = (Math.floor(i / size) / size) - 0.5;
  }

  heights = noisyResources(points);

  let pixels = new Uint8ClampedArray(size * size * 4);
  for (let i = 0; i < size * size; i++) {
    let x = i % size;
    let y = Math.floor(i / size);
    let val = heights[i];
    pixels[i * 4 + 0] = Math.floor(val * 255);
    pixels[i * 4 + 1] = Math.floor(val * 255);
    pixels[i * 4 + 2] = Math.floor(val * 255);
    pixels[i * 4 + 3] = 255;
  }
  return pixels;
}


function normalize (arr, get=d => d) {
  let minValue = min(arr, get);
  let maxValue = max(arr, get) - minValue;

  return arr.map(d => (get(d) - minValue) / maxValue);
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
    this.rust = new Promise((resolve, reject) => wasm()
      .then(results => resolve(new results.TerrainGenerator(seed)))
      .catch(reject)
    );
  }

  async fractalNoise (x, y) {
    return (await this.rust).fractal_noise(x, y);
  }

  async noiseHeight (x, y, { min=-1, max=1 }={}) {
    return (await this.rust).noise_single(x, y);
  }

  async noisyHeights (points, heights) {
    return Array.from((await this.rust).noise_array(points, heights));
  }

  async generateHeightmap (size) {
    let heights = Array(size * size).fill(0);
    let points = Array(size * size * 2).fill(0);
    for (let i = 0; i < size * size; i++) {
      points[i * 2 + 0] = ((i % size) / size);// - 0.5;
      points[i * 2 + 1] = (Math.floor(i / size) / size);// - 0.5;
    }
    heights = await this.noisyHeights(points, heights);
    heights = plateau(points, heights);

    let pixels = new Uint8ClampedArray(size * size * 4);
    for (let i = 0; i < size * size; i++) {
      let x = i % size;
      let y = Math.floor(i / size);
      let val = heights[i];
      pixels[i * 4 + 0] = Math.floor(val * 255);
      pixels[i * 4 + 1] = Math.floor(val * 255);
      pixels[i * 4 + 2] = Math.floor(val * 255);
      pixels[i * 4 + 3] = 255;
    }
    return pixels;
  }


  // async poissonDiscPoints (radius=0.05, seaLevel=0.4, width=1, height=1) {
  //   return (await this.rust).poisson_disc_points(radius, seaLevel, width, height);
  //   console.log('Returned from wasm:', (await this.rust).poisson_disc_points(0.1414, seaLevel, width, height));
  //   // await (this.rust).poisson_disc_points(radius, seaLevel, width, height);
  //   // https://www.youtube.com/watch?v=flQgnCUxHlw
  //   let rng = rngUniform(0, 1);
  //
  //   let k = 30;
  //   let size = radius / Math.sqrt(2);
  //
  //   let cols = Math.floor(width / size);
  //   let rows = Math.floor(height / size);
  //
  //   let grid = Array(cols * rows).fill(undefined);
  //   let active = [];
  //   let points = [];
  //
  //   let x = rng() * width;
  //   let y = rng() * height;
  //   let i = Math.floor(x / size);
  //   let j = Math.floor(y / size);
  //   let pos = { x, y };
  //   grid[i + j * cols] = [ pos ];
  //   active.push(pos);
  //   points.push(pos);
  //
  //
  //   const euclidean = (a, b) => Math.sqrt((a.x - b.x)**2 + (a.y - b.y)**2);
  //   const offsetMagnitude = h => h > seaLevel
  //     ? h
  //     : 1 - h;
  //
  //   while (active.length > 0) {
  //     let randomIndex = Math.floor(rng() * active.length);
  //     let point = active[randomIndex];
  //     let minOffset = size * offsetMagnitude((fractalNoise(point.x, point.y) + 1 )/ 2);
  //
  //     pointHunt:
  //     for (let n = 0; n < k; n++) {
  //       let theta = rng() * Math.PI * 2;
  //       let offset = size + rng() * minOffset;
  //
  //       let x = point.x + Math.cos(theta) * offset;
  //       let y = point.y + Math.sin(theta) * offset;
  //
  //       if (0 > x || x > width || 0 > y || y > height) continue pointHunt;
  //
  //       let sample = { x, y };
  //
  //       let col = Math.floor(x / size);
  //       let row = Math.floor(y / size);
  //
  //       // Loop through adjacent cells
  //       for (let i = -1; i <= 1; i++) {
  //         for (let j = -1; j <= 1; j++) {
  //           let neighborIndex = row + i + cols * (col + j)
  //           let neighbors = grid[neighborIndex];
  //           if (!neighbors) continue;
  //           // Consider each neighbor
  //           for (let neighbor of neighbors) {
  //             let dist = euclidean(sample, neighbor);
  //             if (dist < minOffset) continue pointHunt;
  //           }
  //         }
  //       }
  //       if (!grid[row + col * cols]) {
  //         grid[row + col * cols] = [ sample ];
  //       } else {
  //         grid[row + col * cols].push(sample);
  //       }
  //
  //       active.push(sample);
  //       points.push(sample);
  //     }
  //     active.splice(randomIndex, 1);
  //   }
  //
  //   points.push({ x: 0, y: 0 });
  //   points.push({ x: 0, y: height });
  //   points.push({ x: width, y: 0 });
  //   points.push({ x: width, y: height });
  //   return points.map(p => [p.x - width/2, p.y - height/2]);
  // }


  async generate () {
    let points = this.points;
    let extent = this.extent;
    let seaLevel = this.seaLevel;
    let relaxIterations = this.relaxIterations;
    let seed = this.seed;
    let yieldPoints = this.yieldPoints;
    let yieldRelax = this.yieldRelax;
    let yieldHeights = this.yieldHeights;

    simplex = new SimplexNoise(seed);
    simplexTheta = new SimplexNoise(seed + 'Theta');
    simplexLength = new SimplexNoise(seed + 'Length');
    simplexResources = new SimplexNoise(seed + 'Resources');
    rngUniform = randomUniform.source(seedrandom(seed));

    let world = {};
    world.heightMap = this.generateHeightmap

    let radius = Math.pow(500 / points, 0.5) / 10;

    let timer = Date.now();
    world.points = (await this.rust).poisson_disc_points(radius, seaLevel, extent.width, extent.height);
    console.log(Date.now() - timer, 'ms');
    if (yieldPoints) return world;

    const { voronoi, delaunay, centroids, cells } = improvePoints(world.points, relaxIterations, extent);
    world.cells = cells;
    world.points = centroids;
    world.delaunay = delaunay;
    world.voronoi = voronoi;
    if (yieldRelax) return world;

    // A list of indexes such that each consecutive triplet is a triangle:
    // [i0, j0, k0, ..., in, jn, kn]
    // where `i` is an index in `world.points` and `j, k` are indices in `world.circumcenters`
    const {
      voronoiAdjacency,
      voronoiTriangles,
      voronoiPoints,
      voronoiCells: voronoiCellsLookup
    } = getVoronoiAdjacencies(voronoi);

    world.voronoiAdjacency = voronoiAdjacency;
    world.voronoiTriangles = voronoiTriangles;
    world.voronoiPoints = voronoiPoints;

    // Noise, then plateau
    world.heights = Array(voronoi.circumcenters.length / 2).fill(0);
    world.heights = await this.noisyHeights(voronoi.circumcenters, world.heights);
    world.heights = plateau(voronoi.circumcenters, world.heights);
    // world.heights = fillBasins(world.heights, voronoiAdjacency, seaLevel);

    //  Erode `n` times
    for (let i = 0; i < 10; i++) {
      world.heights = erode(world.heights, voronoiAdjacency, seaLevel);
    }


    // Propagate heights to voronoi centers
    world.nodes = voronoi.circumcenters;
    const getNodeCoordinates = getPointFrom(world.nodes)
    world.nodes = Array(world.nodes.length / 2)
      .fill()
      .map((_, i) => getNodeCoordinates(i));


    world.cellHeights = Array(world.points.length).fill(0)
      .map(getCellHeight(world.heights, voronoiPoints))

    // Calculate triangle heights (such that each triangle is over/under seaLevel as its respective cell)
    world.triangleHeights = Array(voronoiTriangles.length / 3).fill()
        .map(triangulateHeight(world.cellHeights, world.heights, voronoiTriangles, seaLevel));

    world.isLand = world.triangleHeights.map(height => height >= seaLevel);

    const getEdgeCoordinates = getPointFrom(voronoi.circumcenters);
    world.coastCells = getCoastCells(world.cellHeights, delaunay, seaLevel);
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
