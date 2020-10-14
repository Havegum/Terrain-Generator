import REGL from 'regl';
import { mat4 } from 'gl-matrix';
import { extent } from 'd3-array';

import { color, normal } from './utils.js';
import getRenderers from './renderers.js';


export default function initDraw (canvas, triangles, points, circumcenters, seaLevel, coastLines, rivers, cellHeights, heights) {
  const regl = REGL({ canvas, extensions: ['ANGLE_instanced_arrays'] });

  const fieldOfView = 20 * Math.PI / 180 // in radians
  const aspect = canvas.clientWidth / canvas.clientHeight;
  const zNear = 0.1;
  const zFar = 100.0;
  const projectionMatrix = mat4.perspective(mat4.create(), fieldOfView, aspect, zNear, zFar);
  const modelViewMatrix = mat4.create();

  const { drawTerrain, drawCoasts, drawRivers } = getRenderers(regl);


  // TODO: rust returns all the values we need.
  // Terrain: positions and normals as flat arrays with three components per point
  // Rivers: positions as a flat array of points. flux as a flat array.
  //         TODO: these can probably be indexed to avoid duplication
  // Coasts: positions as a flat array of points. These should also be indexed.

  // Terrain
  const triangleCount = triangles.flat().length;
  const zScale = z => (z - seaLevel) * 0.3 + seaLevel; // HACKY
  const toCoords = ([a, b, c]) => [
    [points[a*2], points[a*2+1], zScale(cellHeights[a])],
    [circumcenters[b*2], circumcenters[b*2+1], zScale(heights[b])],
    [circumcenters[c*2], circumcenters[c*2+1], zScale(heights[c])]
  ];
  const triangleCoordinates = triangles.map(toCoords);
  const [minHeight, maxHeight] = extent(triangleCoordinates.flat(), ([x, y, z]) => z);

  const positions = Float32Array.from(triangleCoordinates.flat().flat());
  const normals = Float32Array.from(triangleCoordinates.map(normal).flatMap(n => [...n, ...n, ...n]));

  // Rivers
  const riverCap = 1;
  const riverSegments = rivers.reduce((sum, next) => sum + next.length - 1, 0) - 1;
  const riverPoints = rivers.flatMap(river =>
    river.flatMap((part, i, arr) => {
      if (i === arr.length - 1) return [];
      const index1 = part[0];
      const x1 = circumcenters[index1 * 2 + 0];
      const y1 = circumcenters[index1 * 2 + 1];
      const z1 = zScale(heights[index1]);
      const index2 = arr[i + 1][0];
      const x2 = circumcenters[index2 * 2 + 0];
      const y2 = circumcenters[index2 * 2 + 1];
      const z2 = zScale(heights[index2]);
      return [x1, y1, z1, x2, y2, z2];
    })
  );

  const flux = rivers.flatMap(river =>
    river.flatMap((part, i, arr) => {
      if (i === arr.length - 1) return [];
      return [i === 0 ? arr[i + 1][1] : part[1], arr[i + 1][1]];
    })
  );

  const riverBuffer = regl.buffer(riverPoints)
  const fluxBuffer = regl.buffer(flux)

  // Coasts
  const coastBuffer = regl.buffer(coastLines.flat().flat());

  const defaultSettings = {
    riverCap: 80,
  };

  // Return a draw function, which can take arguments that can modify how the world is rendered.
  function draw ({ camera=null, settings=null }={}) {
    settings = Object.assign(defaultSettings, settings);

    regl.clear({
      color: [0, 0, 0, 1],
      depth: 1,
    });

    // Camera
    if (camera) {
      // First, clear the modelViewMatrix to an identity matrix
      // Then, move from origin [0, 0, 0] to [0.5, 0.5, 0.5], and add the camera X and Y
      // Next, rotate the camera along the Z-axis (think yaw)
      // Then along the Y-axis (think pitch)
      // Next, back the camera up by camera.distance
      // And invert it to get the matrix to move the world
      mat4.copy(modelViewMatrix, mat4.create());
      mat4.translate(modelViewMatrix, modelViewMatrix, [.5 + camera.x, .5 + camera.y, .5]);
      mat4.rotateZ(modelViewMatrix, modelViewMatrix, -camera.zRot);
      mat4.rotateX(modelViewMatrix, modelViewMatrix, -camera.yRot);
      mat4.translate(modelViewMatrix, modelViewMatrix, [0, 0, camera.dist**2]);
      mat4.invert(modelViewMatrix, modelViewMatrix);
    }

    drawTerrain({
      positions,
      normals,
      projectionMatrix,
      modelViewMatrix,
      hillColor:  color('#d3feb0'),
      landColor:  color('#40a74c'),
      waterColor: color('#0cc4d6'),
      depthColor: color('#005e8b'),
      cliffColor: color('#857f69'),
      extent: [minHeight, seaLevel, maxHeight],
      count: triangleCount,
    });

    drawRivers({
      projectionMatrix,
      modelViewMatrix,
      riverCap: settings.riverCap,
      points: riverBuffer,
      flux: fluxBuffer,
      color: color('#0d85c1'),
      segments: riverSegments,
    });

    drawCoasts({
      projectionMatrix,
      modelViewMatrix,
      points: coastBuffer,
      width: 2.5e-3,
      color: color('#133b66'),
      segments: coastLines.length,
    });
  }

  draw({
    camera: {
      zRot: 0,
      yRot: 0,
      dist: 1,
      x: 0,
      y: 0,
    }
  });

  return draw;
}
