"use strict";

import { Delaunay } from 'd3-delaunay';
import { voronoi as d3Voronoi } from 'd3-voronoi';
import { path as d3Path } from 'd3-path';
import { selectAll as d3SelectAll } from 'd3-selection';

import {
  interpolateViridis as d3InterpolateViridis,
  interpolateYlGn as interpolateLand,
  interpolatePuBu as interpolateSea
} from 'd3-scale-chromatic';

import { mean, min, max,
  ascending as d3Ascending,
  descending as d3Descending,
  quantile as d3Quantile,
  scan as d3Scan
} from 'd3-array';

import { randomNormal, randomUniform } from 'd3-random';

// import PriorityQueue from 'js-priority-queue';
import { makeRandomLanguage, makeName } from './language.js';

function runif(lo, hi) {
  // return randomUniform(lo, hi)();
  hi -= lo;
  return Math.random() * hi + lo;
  // return lo + Math.random() * (hi - lo);
}

// let rnorm = (function () {
//   let z2 = null;
//   function rnorm() {
//     if (z2 != null) {
//       let tmp = z2;
//       z2 = null;
//       return tmp;
//     }
//     let x1 = 0;
//     let x2 = 0;
//     let w = 2.0;
//     while (w >= 1) {
//       x1 = runif(-1, 1);
//       x2 = runif(-1, 1);
//       w = x1 * x1 + x2 * x2;
//     }
//     w = Math.sqrt(-2 * Math.log(w) / w);
//     z2 = x2 * w;
//     return x1 * w;
//   }
//   return rnorm;
// })();
const rnorm = randomNormal();

function randomVector(scale) {
  return [scale * rnorm(), scale * rnorm()];
}

let defaultExtent = {
  width: 1,
  height: 1
};

function generatePoints(n, extent) {
  extent = extent || defaultExtent;
  const randomCoords = () => [(Math.random() - 0.5) * extent.width, (Math.random() - 0.5) * extent.height];
  return Array(n).fill().map(randomCoords);
}

function centroid(pts) {
  let x = 0;
  let y = 0;
  for (let i = 0; i < pts.length; i++) {
    x += pts[i][0];
    y += pts[i][1];
  }
  return [x/pts.length, y/pts.length];
}

function improvePoints(pts, n, extent) {
  n = n || 1;
  extent = extent || defaultExtent;
  for (let i = 0; i < n; i++) {
    pts = voronoi(pts, extent)
        .polygons(pts)
        .map(centroid);
  }
  return pts;
}

function generateGoodPoints(n, extent) {
  extent = extent || defaultExtent;
  let pts = generatePoints(n, extent).sort((a, b) => a[0] - b[0]);
  return improvePoints(pts, 1, extent);
}

function voronoi(pts, extent) {
  extent = extent || defaultExtent;
  let w = extent.width/2;
  let h = extent.height/2;
  console.log([[-w, -h], [w, h]]);
  let voronoi = d3Voronoi().extent([[-w, -h], [w, h]])(pts);
  const delaunay = Delaunay.from(pts);
  let delaunayVoronoi = delaunay.voronoi([-w, -h, w, h]);

  console.log('delaunay:', delaunayVoronoi);
  const splitAndDigitize = render => render.split(/M/).slice(1).map(d => d.split('L').map(d => d.split(',').map(d => +d)));
  delaunayVoronoi.edges = splitAndDigitize(delaunayVoronoi.render());
  console.log(voronoi.edges);
  return voronoi;
}

function* edges(voronoi, context) {
  const {
    delaunay: { halfedges, inedges, hull },
    circumcenters,
    vectors
  } = voronoi;
  if (hull.length <= 1) return null;

  for (let i = 0, n = halfedges.length; i < n; ++i) {
    const j = halfedges[i];
    if (j < i) continue;
    const ti = Math.floor(i / 3) * 2;
    const tj = Math.floor(j / 3) * 2;
    const xi = circumcenters[ti];
    const yi = circumcenters[ti + 1];
    const xj = circumcenters[tj];
    const yj = circumcenters[tj + 1];

    const path = context || d3.path();
    voronoi._renderSegment(xi, yi, xj, yj, path);
    yield "" + path;
  }

  let h0,
    h1 = hull[hull.length - 1];
  for (let i = 0; i < hull.length; ++i) {
    (h0 = h1), (h1 = hull[i]);
    const t = Math.floor(inedges[h1] / 3) * 2;
    const x = circumcenters[t];
    const y = circumcenters[t + 1];
    const v = h0 * 4;
    const p = voronoi._project(x, y, vectors[v + 2], vectors[v + 3]);

    if (p) {
      const path = context || d3.path();
      voronoi._renderSegment(x, y, p[0], p[1], path);
      yield "" + path;
    }
  }
}

function makeMesh(pts, extent) {
  extent = extent || defaultExtent;
  let vor = voronoi(pts, extent);
  console.log('vor', vor);
  let vxs = [];
  let vxids = {};
  let adj = [];
  let edges = [];
  let tris = [];

  for (let i = 0; i < vor.edges.length; i++) {
    let e = vor.edges[i];
    if (e === undefined) continue;

    let e0 = vxids[e[0]];
    let e1 = vxids[e[1]];
    if (e0 === undefined) {
      e0 = vxs.length;
      vxids[e[0]] = e0;
      vxs.push(e[0]);
    }
    if (e1 === undefined) {
      e1 = vxs.length;
      vxids[e[1]] = e1;
      vxs.push(e[1]);
    }

    adj[e0] = adj[e0] || [];
    adj[e0].push(e1);
    adj[e1] = adj[e1] || [];
    adj[e1].push(e0);
    edges.push([e0, e1, e.left, e.right]);
    tris[e0] = tris[e0] || [];
    if (!tris[e0].includes(e.left)) tris[e0].push(e.left);
    if (e.right && !tris[e0].includes(e.right)) tris[e0].push(e.right);
    tris[e1] = tris[e1] || [];
    if (!tris[e1].includes(e.left)) tris[e1].push(e.left);
    if (e.right && !tris[e1].includes(e.right)) tris[e1].push(e.right);
  }

  let mesh = { pts, vor, vxs, adj, tris, edges, extent };

  mesh.map = function (f) {
    let mapped = vxs.map(f);
    mapped.mesh = mesh;
    return mapped;
  }

  return mesh;
}


function generateGoodMesh(n, extent) {
  extent = extent || defaultExtent;
  let pts = generateGoodPoints(n, extent);
  return makeMesh(pts, extent);
}

function isedge(mesh, i) {
  return (mesh.adj[i].length < 3);
}

function isnearedge(mesh, i) {
  let x = mesh.vxs[i][0];
  let y = mesh.vxs[i][1];
  let w = mesh.extent.width;
  let h = mesh.extent.height;
  return x < -0.45 * w || x > 0.45 * w || y < -0.45 * h || y > 0.45 * h;
}

function neighbours(mesh, i) {
  let onbs = mesh.adj[i];
  let nbs = [];
  for (let i = 0; i < onbs.length; i++) {
    nbs.push(onbs[i]);
  }
  return nbs;
}

function distance(mesh, i, j) {
  let p = mesh.vxs[i];
  let q = mesh.vxs[j];
  return Math.sqrt((p[0] - q[0]) * (p[0] - q[0]) + (p[1] - q[1]) * (p[1] - q[1]));
}

function quantile(h, q) {
  let sortedh = [];
  for (let i = 0; i < h.length; i++) {
    sortedh[i] = h[i];
  }
  sortedh.sort(d3Ascending);
  return d3Quantile(sortedh, q);
}

function zero(mesh) {
  let z = [];
  for (let i = 0; i < mesh.vxs.length; i++) {
    z[i] = 0;
  }
  z.mesh = mesh;
  return z;
}

function slope(mesh, direction) {
  return mesh.map(function (x) {
    return x[0] * direction[0] + x[1] * direction[1];
  });
}

function cone(mesh, slope) {
  return mesh.map(function (x) {
    return Math.pow(x[0] * x[0] + x[1] * x[1], 0.5) * slope;
  });
}

function map(h, f) {
  let newh = h.map(f);
  newh.mesh = h.mesh;
  return newh;
}

function normalize(h) {
  let lo = min(h);
  let hi = max(h);
  return map(h, function (x) {return (x - lo) / (hi - lo)});
}

function peaky(h) {
  return map(normalize(h), Math.sqrt);
}

function add() {
  let n = arguments[0].length;
  let newvals = zero(arguments[0].mesh);
  for (let i = 0; i < n; i++) {
    for (let j = 0; j < arguments.length; j++) {
      newvals[i] += arguments[j][i];
    }
  }
  return newvals;
}

function mountains(mesh, n, r) {
  r = r || 0.05;
  const randomCoords = () => [mesh.extent.width * (Math.random() - 0.5), mesh.extent.height * (Math.random() - 0.5)];
  let mounts = Array(n).fill().map(randomCoords);

  let newvals = zero(mesh);
  for (let i = 0; i < mesh.vxs.length; i++) {
    let p = mesh.vxs[i];
    for (let j = 0; j < n; j++) {
      let m = mounts[j];
      newvals[i] += Math.pow(Math.exp(-((p[0] - m[0]) * (p[0] - m[0]) + (p[1] - m[1]) * (p[1] - m[1])) / (2 * r * r)), 2);
    }
  }
  return newvals;
}

function relax(h) {
  let newh = zero(h.mesh);
  for (let i = 0; i < h.length; i++) {
    let nbs = neighbours(h.mesh, i);
    if (nbs.length < 3) {
      newh[i] = 0;
      continue;
    }
    newh[i] = mean(nbs.map(function (j) {return h[j]}));
  }
  return newh;
}

function downhill(h) {
  // Calculate this just once at the beginning
  if (h.downhill) return h.downhill;
  function downfrom(i) {
    if (isedge(h.mesh, i)) return -2;
    let best = -1;
    let besth = h[i];
    let nbs = neighbours(h.mesh, i);
    for (let j = 0; j < nbs.length; j++) {
      if (h[nbs[j]] < besth) {
        besth = h[nbs[j]];
        best = nbs[j];
      }
    }
    return best;
  }
  let downs = [];
  for (let i = 0; i < h.length; i++) {
    downs[i] = downfrom(i);
  }
  h.downhill = downs;
  return downs;
}

function findSinks(h) {
  let dh = downhill(h);
  let sinks = [];
  for (let i = 0; i < dh.length; i++) {
    let node = i;
    while (true) {
      if (isedge(h.mesh, node)) {
        sinks[i] = -2;
        break;
      }
      if (dh[node] == -1) {
        sinks[i] = node;
        break;
      }
      node = dh[node];
    }
  }
}

function fillSinks(h, epsilon) {
  epsilon = epsilon || 1e-5;
  let infinity = 999999;
  let newh = zero(h.mesh);
  for (let i = 0; i < h.length; i++) {
    if (isnearedge(h.mesh, i)) {
      newh[i] = h[i];
    } else {
      newh[i] = infinity;
    }
  }
  while (true) {
    let changed = false;
    for (let i = 0; i < h.length; i++) {
      if (newh[i] == h[i]) continue;
      let nbs = neighbours(h.mesh, i);
      for (let j = 0; j < nbs.length; j++) {
        if (h[i] >= newh[nbs[j]] + epsilon) {
          newh[i] = h[i];
          changed = true;
          break;
        }
        let oh = newh[nbs[j]] + epsilon;
        if ((newh[i] > oh) && (oh > h[i])) {
          newh[i] = oh;
          changed = true;
        }
      }
    }
    if (!changed) return newh;
  }
}

function getFlux(h) {
  let dh = downhill(h);
  let idxs = [];
  let flux = zero(h.mesh);
  for (let i = 0; i < h.length; i++) {
    idxs[i] = i;
    flux[i] = 1/h.length;
  }
  idxs.sort(function (a, b) {
    return h[b] - h[a];
  });
  for (let i = 0; i < h.length; i++) {
    let j = idxs[i];
    if (dh[j] >= 0) {
      flux[dh[j]] += flux[j];
    }
  }
  return flux;
}

function getSlope(h) {
    let dh = downhill(h);
    let slope = zero(h.mesh);
    for (let i = 0; i < h.length; i++) {
        let s = trislope(h, i);
        slope[i] = Math.sqrt(s[0] * s[0] + s[1] * s[1]);
        continue;
        if (dh[i] < 0) {
            slope[i] = 0;
        } else {
            slope[i] = (h[i] - h[dh[i]]) / distance(h.mesh, i, dh[i]);
        }
    }
    return slope;
}

function erosionRate(h) {
    let flux = getFlux(h);
    let slope = getSlope(h);
    let newh = zero(h.mesh);
    for (let i = 0; i < h.length; i++) {
        let river = Math.sqrt(flux[i]) * slope[i];
        let creep = slope[i] * slope[i];
        let total = 1000 * river + creep;
        total = total > 200 ? 200 : total;
        newh[i] = total;
    }
    return newh;
}

function erode(h, amount) {
    let er = erosionRate(h);
    let newh = zero(h.mesh);
    let maxr = max(er);
    for (let i = 0; i < h.length; i++) {
        newh[i] = h[i] - amount * (er[i] / maxr);
    }
    return newh;
}

function doErosion(h, amount, n) {
    n = n || 1;
    h = fillSinks(h);
    for (let i = 0; i < n; i++) {
        h = erode(h, amount);
        h = fillSinks(h);
    }
    return h;
}

function setSeaLevel(h, q) {
    let newh = zero(h.mesh);
    let delta = quantile(h, q);
    for (let i = 0; i < h.length; i++) {
        newh[i] = h[i] - delta;
    }
    return newh;
}

function cleanCoast(h, iters) {
  for (let iter = 0; iter < iters; iter++) {
    let changed = 0;
    let newh = zero(h.mesh);
    for (let i = 0; i < h.length; i++) {
      newh[i] = h[i];
      let nbs = neighbours(h.mesh, i);
      if (h[i] <= 0 || nbs.length != 3) continue;
      let count = 0;
      let best = -999999;
      for (let j = 0; j < nbs.length; j++) {
        if (h[nbs[j]] > 0) {
          count++;
        } else if (h[nbs[j]] > best) {
          best = h[nbs[j]];
        }
      }
      if (count > 1) continue;
      newh[i] = best / 2;
      changed++;
    }
    h = newh;
    newh = zero(h.mesh);
    for (let i = 0; i < h.length; i++) {
      newh[i] = h[i];
      let nbs = neighbours(h.mesh, i);
      if (h[i] > 0 || nbs.length != 3) continue;
      let count = 0;
      let best = 999999;
      for (let j = 0; j < nbs.length; j++) {
        if (h[nbs[j]] <= 0) {
          count++;
        } else if (h[nbs[j]] < best) {
          best = h[nbs[j]];
        }
      }
      if (count > 1) continue;
      newh[i] = best / 2;
      changed++;
    }
    h = newh;
  }
  return h;
}

function trislope(h, i) {
  let nbs = neighbours(h.mesh, i);
  if (nbs.length !== 3) return [0,0];
  let p0 = h.mesh.vxs[nbs[0]];
  let p1 = h.mesh.vxs[nbs[1]];
  let p2 = h.mesh.vxs[nbs[2]];

  let x1 = p1[0] - p0[0];
  let x2 = p2[0] - p0[0];
  let y1 = p1[1] - p0[1];
  let y2 = p2[1] - p0[1];

  let det = x1 * y2 - x2 * y1;
  let h1 = h[nbs[1]] - h[nbs[0]];
  let h2 = h[nbs[2]] - h[nbs[0]];

  return [
    (y2 * h1 - y1 * h2) / det,
    (-x2 * h1 + x1 * h2) / det
  ];
}

function cityScore(h, cities) {
    let score = map(getFlux(h), Math.sqrt);
    for (let i = 0; i < h.length; i++) {
        if (h[i] <= 0 || isnearedge(h.mesh, i)) {
            score[i] = -999999;
            continue;
        }
        score[i] += 0.01 / (1e-9 + Math.abs(h.mesh.vxs[i][0]) - h.mesh.extent.width/2)
        score[i] += 0.01 / (1e-9 + Math.abs(h.mesh.vxs[i][1]) - h.mesh.extent.height/2)
        for (let j = 0; j < cities.length; j++) {
            score[i] -= 0.02 / (distance(h.mesh, cities[j], i) + 1e-9);
        }
    }
    return score;
}
function placeCity(render) {
    render.cities = render.cities || [];
    let score = cityScore(render.h, render.cities);
    let newcity = d3Scan(score, d3Descending);
    render.cities.push(newcity);
}

function placeCities(render) {
    let params = render.params;
    let h = render.h;
    let n = params.ncities;
    for (let i = 0; i < n; i++) {
        placeCity(render);
    }
}

function contour(h, level) {
    level = level || 0;
    let edges = [];
    for (let i = 0; i < h.mesh.edges.length; i++) {
        let e = h.mesh.edges[i];
        if (e[3] == undefined) continue;
        if (isnearedge(h.mesh, e[0]) || isnearedge(h.mesh, e[1])) continue;
        if ((h[e[0]] > level && h[e[1]] <= level) ||
            (h[e[1]] > level && h[e[0]] <= level)) {
            edges.push([e[2], e[3]]);
        }
    }
    return mergeSegments(edges);
}

function getRivers(h, limit) {
    let dh = downhill(h);
    let flux = getFlux(h);
    let links = [];
    let above = 0;
    for (let i = 0; i < h.length; i++) {
        if (h[i] > 0) above++;
    }
    limit *= above / h.length;
    for (let i = 0; i < dh.length; i++) {
        if (isnearedge(h.mesh, i)) continue;
        if (flux[i] > limit && h[i] > 0 && dh[i] >= 0) {
            let up = h.mesh.vxs[i];
            let down = h.mesh.vxs[dh[i]];
            if (h[dh[i]] > 0) {
                links.push([up, down]);
            } else {
                links.push([up, [(up[0] + down[0])/2, (up[1] + down[1])/2]]);
            }
        }
    }
    return mergeSegments(links).map(relaxPath);
}

function getTerritories(render) {
    let h = render.h;
    let cities = render.cities;
    let n = render.params.nterrs;
    if (n > render.cities.length) n = render.cities.length;
    let flux = getFlux(h);
    let terr = [];
    let queue = new PriorityQueue({comparator: function (a, b) {return a.score - b.score}});
    function weight(u, v) {
        let horiz = distance(h.mesh, u, v);
        let vert = h[v] - h[u];
        if (vert > 0) vert /= 10;
        let diff = 1 + 0.25 * Math.pow(vert/horiz, 2);
        diff += 100 * Math.sqrt(flux[u]);
        if (h[u] <= 0) diff = 100;
        if ((h[u] > 0) != (h[v] > 0)) return 1000;
        return horiz * diff;
    }
    for (let i = 0; i < n; i++) {
        terr[cities[i]] = cities[i];
        let nbs = neighbours(h.mesh, cities[i]);
        for (let j = 0; j < nbs.length; j++) {
            queue.queue({
                score: weight(cities[i], nbs[j]),
                city: cities[i],
                vx: nbs[j]
            });
        }
    }
    while (queue.length) {
        let u = queue.dequeue();
        if (terr[u.vx] != undefined) continue;
        terr[u.vx] = u.city;
        let nbs = neighbours(h.mesh, u.vx);
        for (let i = 0; i < nbs.length; i++) {
            let v = nbs[i];
            if (terr[v] != undefined) continue;
            let newdist = weight(u.vx, v);
            queue.queue({
                score: u.score + newdist,
                city: u.city,
                vx: v
            });
        }
    }
    terr.mesh = h.mesh;
    return terr;
}

function getBorders(render) {
    let terr = render.terr;
    let h = render.h;
    let edges = [];
    for (let i = 0; i < terr.mesh.edges.length; i++) {
        let e = terr.mesh.edges[i];
        if (e[3] == undefined) continue;
        if (isnearedge(terr.mesh, e[0]) || isnearedge(terr.mesh, e[1])) continue;
        if (h[e[0]] < 0 || h[e[1]] < 0) continue;
        if (terr[e[0]] != terr[e[1]]) {
            edges.push([e[2], e[3]]);
        }
    }
    return mergeSegments(edges).map(relaxPath);
}

function mergeSegments(segs) {
    let adj = {};
    for (let i = 0; i < segs.length; i++) {
        let seg = segs[i];
        let a0 = adj[seg[0]] || [];
        let a1 = adj[seg[1]] || [];
        a0.push(seg[1]);
        a1.push(seg[0]);
        adj[seg[0]] = a0;
        adj[seg[1]] = a1;
    }
    let done = [];
    let paths = [];
    let path = null;
    while (true) {
        if (path == null) {
            for (let i = 0; i < segs.length; i++) {
                if (done[i]) continue;
                done[i] = true;
                path = [segs[i][0], segs[i][1]];
                break;
            }
            if (path == null) break;
        }
        let changed = false;
        for (let i = 0; i < segs.length; i++) {
            if (done[i]) continue;
            if (adj[path[0]].length == 2 && segs[i][0] == path[0]) {
                path.unshift(segs[i][1]);
            } else if (adj[path[0]].length == 2 && segs[i][1] == path[0]) {
                path.unshift(segs[i][0]);
            } else if (adj[path[path.length - 1]].length == 2 && segs[i][0] == path[path.length - 1]) {
                path.push(segs[i][1]);
            } else if (adj[path[path.length - 1]].length == 2 && segs[i][1] == path[path.length - 1]) {
                path.push(segs[i][0]);
            } else {
                continue;
            }
            done[i] = true;
            changed = true;
            break;
        }
        if (!changed) {
            paths.push(path);
            path = null;
        }
    }
    return paths;
}

function relaxPath(path) {
  let newpath = [path[0]];
  for (let i = 1; i < path.length - 1; i++) {
    let newpt = [0.25 * path[i-1][0] + 0.5 * path[i][0] + 0.25 * path[i+1][0],
                 0.25 * path[i-1][1] + 0.5 * path[i][1] + 0.25 * path[i+1][1]];
    newpath.push(newpt);
  }
  newpath.push(path[path.length - 1]);
  return newpath;
}

function visualizePoints(svg, pts) {
    let circle = svg.selectAll('circle').data(pts);
    circle.enter()
        .append('circle');
    circle.exit().remove();
    d3SelectAll('circle')
        .attr('cx', function (d) {return 1000*d[0]})
        .attr('cy', function (d) {return 1000*d[1]})
        .attr('r', 100 / Math.sqrt(pts.length));
}

function makeD3Path(path) {
    let p = d3Path();
    p.moveTo(1000*path[0][0], 1000*path[0][1]);
    for (let i = 1; i < path.length; i++) {
        p.lineTo(1000*path[i][0], 1000*path[i][1]);
    }
    return p.toString();
}

function visualizeVoronoi(svg, field, lo, hi) {
    if (hi == undefined) hi = max(field) + 1e-9;
    if (lo == undefined) lo = min(field) - 1e-9;
    let sea = -lo / (hi - lo);

    let mappedvals = field.map(x => x > hi ? 1 : x < lo ? 0 : (x - lo) / (hi - lo));
    let tris = svg.selectAll('path.field').data(field.mesh.tris)
    tris.enter()
        .append('path')
        .classed('field', true);

    tris.exit()
        .remove();

    svg.selectAll('path.field')
        .attr('d', makeD3Path)
        .style('fill', function (d, i) {
          let val = mappedvals[i];
          let interpolation = val > sea ? interpolateLand : interpolateSea;
          return interpolation(1 - val);
        });


    let vor = svg.selectAll('line.voronoi').data(field.mesh.vor.edges.filter(d => !!d));
    vor.enter()
      .append('line')
      .classed('voronoi', true);

    vor.exit().remove();

    svg.selectAll('line.voronoi')
      .attr('x1', d => 1000 * d[0][0])
      .attr('y1', d => 1000 * d[0][1])
      .attr('x2', d => 1000 * d[1][0])
      .attr('y2', d => 1000 * d[1][1])
}

function visualizeDownhill(h) {
    let links = getRivers(h, 0.01);
    drawPaths('river', links);
}

function drawPaths(svg, cls, paths) {
    let _paths = svg.selectAll('path.' + cls).data(paths)
    _paths.enter()
            .append('path')
            .classed(cls, true)
    _paths.exit()
            .remove();
    svg.selectAll('path.' + cls)
        .attr('d', makeD3Path);
}

function calcSlopes(h) {
  let strokes = [];
  let r = 0.25 / Math.sqrt(h.length);
  for (let i = 0; i < h.length; i++) {
      if (h[i] <= 0 || isnearedge(h.mesh, i)) continue;
      let nbs = neighbours(h.mesh, i);
      nbs.push(i);
      let s = 0;
      let s2 = 0;
      for (let j = 0; j < nbs.length; j++) {
          let slopes = trislope(h, nbs[j]);
          s += slopes[0] / 10;
          s2 += slopes[1];
      }
      s /= nbs.length;
      s2 /= nbs.length;
      if (Math.abs(s) < runif(0.1, 0.4)) continue;
      let l = r * runif(1, 2) * (1 - 0.2 * Math.pow(Math.atan(s), 2)) * Math.exp(s2/100);
      let x = h.mesh.vxs[i][0];
      let y = h.mesh.vxs[i][1];
      if (Math.abs(l*s) > 2 * r) {
          let n = Math.floor(Math.abs(l*s/r));
          l /= n;
          if (n > 4) n = 4;
          for (let j = 0; j < n; j++) {
              let u = rnorm() * r;
              let v = rnorm() * r;
              strokes.push([[x+u-l, y+v+l*s], [x+u+l, y+v-l*s]]);
          }
      } else {
          strokes.push([[x-l, y+l*s], [x+l, y-l*s]]);
      }
  }

  return strokes;
}

function visualizeSlopes(svg, render) {
  let strokes = calcSlopes(render.h);
  let lines = svg.selectAll('line.slope').data(strokes)
  lines.enter()
          .append('line')
          .classed('slope', true);
  lines.exit()
          .remove();
  svg.selectAll('line.slope')
      .attr('x1', function (d) {return 1000*d[0][0]})
      .attr('y1', function (d) {return 1000*d[0][1]})
      .attr('x2', function (d) {return 1000*d[1][0]})
      .attr('y2', function (d) {return 1000*d[1][1]})
}


function visualizeContour(h, level) {
  level = level || 0;
  let links = contour(h, level);
  drawPaths('coast', links);
}

function visualizeBorders(h, cities, n) {
  let links = getBorders(h, getTerritories(h, cities, n));
  drawPaths('border', links);
}


function visualizeCities(svg, render) {
  let cities = render.cities;
  let h = render.h;
  let n = render.params.nterrs;

  let circs = svg.selectAll('circle.city').data(cities);
  circs.enter()
          .append('circle')
          .classed('city', true);
  circs.exit()
          .remove();
  svg.selectAll('circle.city')
      .attr('cx', function (d) {return 1000*h.mesh.vxs[d][0]})
      .attr('cy', function (d) {return 1000*h.mesh.vxs[d][1]})
      .attr('r', function (d, i) {return i >= n ? 4 : 10})
      .style('fill', 'white')
      .style('stroke-width', 5)
      .style('stroke-linecap', 'round')
      .style('stroke', 'black')
      .raise();
}

function dropEdge(h, p) {
  p = p || 4
  let newh = zero(h.mesh);
  for (let i = 0; i < h.length; i++) {
    let v = h.mesh.vxs[i];
    let x = 2.4*v[0] / h.mesh.extent.width;
    let y = 2.4*v[1] / h.mesh.extent.height;
    newh[i] = h[i] - Math.exp(10*(Math.pow(Math.pow(x, p) + Math.pow(y, p), 1/p) - 1));
  }
  return newh;
}

function generateCoast(params) {
  let mesh = generateGoodMesh(params.npts, params.extent);
  let h = add(
          slope(mesh, randomVector(4)),
          cone(mesh, runif(-1, -1)),
          mountains(mesh, 50)
          );
  for (let i = 0; i < 10; i++) {
    h = relax(h);
  }
  h = peaky(h);
  h = doErosion(h, runif(0, 0.1), 5);
  h = setSeaLevel(h, runif(0.2, 0.6));
  h = fillSinks(h);
  h = cleanCoast(h, 3);
  return h;
}

function terrCenter(h, terr, city, landOnly) {
  let x = 0;
  let y = 0;
  let n = 0;
  for (let i = 0; i < terr.length; i++) {
    if (terr[i] != city) continue;
    if (landOnly && h[i] <= 0) continue;
    x += terr.mesh.vxs[i][0];
    y += terr.mesh.vxs[i][1];
    n++;
  }
  return [x/n, y/n];
}

function drawLabels(svg, render) {
    let params = render.params;
    let h = render.h;
    let terr = render.terr;
    let cities = render.cities;
    let nterrs = render.params.nterrs;
    let avoids = [render.rivers, render.coasts, render.borders];
    let lang = makeRandomLanguage();
    let citylabels = [];
    function penalty(label) {
        let pen = 0;
        if (label.x0 < -0.45 * h.mesh.extent.width) pen += 100;
        if (label.x1 > 0.45 * h.mesh.extent.width) pen += 100;
        if (label.y0 < -0.45 * h.mesh.extent.height) pen += 100;
        if (label.y1 > 0.45 * h.mesh.extent.height) pen += 100;
        for (let i = 0; i < citylabels.length; i++) {
            let olabel = citylabels[i];
            if (label.x0 < olabel.x1 && label.x1 > olabel.x0 &&
                label.y0 < olabel.y1 && label.y1 > olabel.y0) {
                pen += 100;
            }
        }

        for (let i = 0; i < cities.length; i++) {
            let c = h.mesh.vxs[cities[i]];
            if (label.x0 < c[0] && label.x1 > c[0] && label.y0 < c[1] && label.y1 > c[1]) {
                pen += 100;
            }
        }
        for (let i = 0; i < avoids.length; i++) {
            let avoid = avoids[i];
            for (let j = 0; j < avoid.length; j++) {
                let avpath = avoid[j];
                for (let k = 0; k < avpath.length; k++) {
                    let pt = avpath[k];
                    if (pt[0] > label.x0 && pt[0] < label.x1 && pt[1] > label.y0 && pt[1] < label.y1) {
                        pen++;
                    }
                }
            }
        }
        return pen;
    }
    for (let i = 0; i < cities.length; i++) {
        let x = h.mesh.vxs[cities[i]][0];
        let y = h.mesh.vxs[cities[i]][1];
        let text = makeName(lang, 'city');
        let size = i < nterrs ? params.fontsizes.city : params.fontsizes.town;
        let sx = 0.65 * size/1000 * text.length;
        let sy = size/1000;
        let posslabels = [
        {
            x: x + 0.8 * sy,
            y: y + 0.3 * sy,
            align: 'start',
            x0: x + 0.7 * sy,
            y0: y - 0.6 * sy,
            x1: x + 0.7 * sy + sx,
            y1: y + 0.6 * sy
        },
        {
            x: x - 0.8 * sy,
            y: y + 0.3 * sy,
            align: 'end',
            x0: x - 0.9 * sy - sx,
            y0: y - 0.7 * sy,
            x1: x - 0.9 * sy,
            y1: y + 0.7 * sy
        },
        {
            x: x,
            y: y - 0.8 * sy,
            align: 'middle',
            x0: x - sx/2,
            y0: y - 1.9*sy,
            x1: x + sx/2,
            y1: y - 0.7 * sy
        },
        {
            x: x,
            y: y + 1.2 * sy,
            align: 'middle',
            x0: x - sx/2,
            y0: y + 0.1*sy,
            x1: x + sx/2,
            y1: y + 1.3*sy
        }
        ];
        let label = posslabels[d3Scan(posslabels, function (a, b) {return penalty(a) - penalty(b)})];
        label.text = text;
        label.size = size;
        citylabels.push(label);
    }
    let texts = svg.selectAll('text.city').data(citylabels);
    texts.enter()
        .append('text')
        .classed('city', true);
    texts.exit()
        .remove();
    svg.selectAll('text.city')
        .attr('x', function (d) {return 1000*d.x})
        .attr('y', function (d) {return 1000*d.y})
        .style('font-size', function (d) {return d.size})
        .style('text-anchor', function (d) {return d.align})
        .text(function (d) {return d.text})
        .raise();

    let reglabels = [];
    for (let i = 0; i < nterrs; i++) {
        let city = cities[i];
        let text = makeName(lang, 'region');
        let sy = params.fontsizes.region / 1000;
        let sx = 0.6 * text.length * sy;
        let lc = terrCenter(h, terr, city, true);
        let oc = terrCenter(h, terr, city, false);
        let best = 0;
        let bestscore = -999999;
        for (let j = 0; j < h.length; j++) {
            let score = 0;
            let v = h.mesh.vxs[j];
            score -= 3000 * Math.sqrt((v[0] - lc[0]) * (v[0] - lc[0]) + (v[1] - lc[1]) * (v[1] - lc[1]));
            score -= 1000 * Math.sqrt((v[0] - oc[0]) * (v[0] - oc[0]) + (v[1] - oc[1]) * (v[1] - oc[1]));
            if (terr[j] != city) score -= 3000;
            for (let k = 0; k < cities.length; k++) {
                let u = h.mesh.vxs[cities[k]];
                if (Math.abs(v[0] - u[0]) < sx &&
                    Math.abs(v[1] - sy/2 - u[1]) < sy) {
                    score -= k < nterrs ? 4000 : 500;
                }
                if (v[0] - sx/2 < citylabels[k].x1 &&
                    v[0] + sx/2 > citylabels[k].x0 &&
                    v[1] - sy < citylabels[k].y1 &&
                    v[1] > citylabels[k].y0) {
                    score -= 5000;
                }
            }
            for (let k = 0; k < reglabels.length; k++) {
                let label = reglabels[k];
                if (v[0] - sx/2 < label.x + label.width/2 &&
                    v[0] + sx/2 > label.x - label.width/2 &&
                    v[1] - sy < label.y &&
                    v[1] > label.y - label.size) {
                    score -= 20000;
                }
            }
            if (h[j] <= 0) score -= 500;
            if (v[0] + sx/2 > 0.5 * h.mesh.extent.width) score -= 50000;
            if (v[0] - sx/2 < -0.5 * h.mesh.extent.width) score -= 50000;
            if (v[1] > 0.5 * h.mesh.extent.height) score -= 50000;
            if (v[1] - sy < -0.5 * h.mesh.extent.height) score -= 50000;
            if (score > bestscore) {
                bestscore = score;
                best = j;
            }
        }
        reglabels.push({
            text: text,
            x: h.mesh.vxs[best][0],
            y: h.mesh.vxs[best][1],
            size:sy,
            width:sx
        });
    }
    texts = svg.selectAll('text.region').data(reglabels);
    texts.enter()
        .append('text')
        .classed('region', true);
    texts.exit()
        .remove();
    svg.selectAll('text.region')
        .attr('x', function (d) {return 1000*d.x})
        .attr('y', function (d) {return 1000*d.y})
        .style('font-size', function (d) {return 1000*d.size})
        .style('text-anchor', 'middle')
        .text(function (d) {return d.text})
        .raise();

}
function drawMap(svg, render) {
    visualizeVoronoi(svg, render.h)

    render.rivers = getRivers(render.h, 0.01);
    render.coasts = contour(render.h, 0);
    render.terr = getTerritories(render);
    render.borders = getBorders(render);
    console.log(render);
    drawPaths(svg, 'river', render.rivers);
    drawPaths(svg, 'coast', render.coasts);
    drawPaths(svg, 'border', render.borders);
    visualizeSlopes(svg, render);
    visualizeCities(svg, render);
    drawLabels(svg, render);
}


function doMap(svg, params) {
    let render = {
        params
    };

    const width = svg.attr('width');
    svg.attr('height', width * params.extent.height / params.extent.width);
    svg.attr('viewBox', -1000 * params.extent.width / 2  + ' ' +
                        -1000 * params.extent.height / 2 + ' ' +
                         1000 * params.extent.width    + ' ' +
                         1000 * params.extent.height);
    svg.selectAll().remove();
    render.h = params.generator(params);
    placeCities(render);
    drawMap(svg, render);
}

let defaultParams = {
    extent: defaultExtent,
    generator: generateCoast,
    npts: 16384,
    ncities: 15,
    nterrs: 5,
    fontsizes: {
        region: 40,
        city: 25,
        town: 20
    }
}

export {
  runif,
  randomVector,
  generatePoints,
  centroid,
  improvePoints,
  generateGoodPoints,
  voronoi,
  makeMesh,
  generateGoodMesh,
  isedge,
  isnearedge,
  neighbours,
  distance,
  quantile,
  zero,
  slope,
  cone,
  map,
  normalize,
  peaky,
  add,
  mountains,
  relax,
  downhill,
  findSinks,
  fillSinks,
  getFlux,
  getSlope,
  erosionRate,
  erode,
  doErosion,
  setSeaLevel,
  cleanCoast,
  trislope,
  cityScore,
  placeCity,
  placeCities,
  contour,
  getRivers,
  getTerritories,
  getBorders,
  mergeSegments,
  relaxPath,
  visualizePoints,
  makeD3Path,
  visualizeVoronoi,
  visualizeDownhill,
  drawPaths,
  calcSlopes,
  visualizeSlopes,
  visualizeContour,
  visualizeCities,
  dropEdge,
  generateCoast,
  terrCenter,
  drawLabels,
  drawMap,
  doMap,
  defaultParams
}
