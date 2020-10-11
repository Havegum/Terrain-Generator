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
  const modelViewMatrix = mat4.create();// Set the drawing position to the "identity" point, which is the center of the scene.

  const { drawTerrain, drawCoasts, drawRivers } = getRenderers(regl);

  // Terrain
  const triangleCount = triangles.flat().length;
  const zScale = z => (z - seaLevel) * 0.3 + seaLevel;
  const toCoords = ([a, b, c]) => [
    [points[a*2], points[a*2+1], zScale(cellHeights[a])],
    [circumcenters[b*2], circumcenters[b*2+1], zScale(heights[b])],
    [circumcenters[c*2], circumcenters[c*2+1], zScale(heights[c])]
  ];
  const triangleCoordinates = triangles.map(toCoords);
  const [minHeight, maxHeight] = extent(triangleCoordinates.flat(), ([x, y, z]) => z);

  const positions3d = Float32Array.from(triangleCoordinates.flat().flat());
  const normals3d = Float32Array.from(triangleCoordinates.map(normal).flatMap(n => [...n, ...n, ...n]));

  // Rivers
  const riverCap = 80;
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
  const riverWidths = rivers.flatMap(river =>
    river.flatMap((part, i, arr) => {
      if (i === arr.length - 1) return [];
      return [i === 0 ? arr[i + 1][1] : part[1], arr[i + 1][1]];
    }).map(n => n <= riverCap ? 0 : Math.log((n - riverCap) * 5) * 4e-4)
  );

  // Coasts
  const coastBuffer = regl.buffer(coastLines.flat().flat());

  function draw (camera) {
    regl.clear({
      color: [0, 0, 0, 1],
      depth: 1,
    })

    // Camera
    // First, clear the modelViewMatrix to an identity matrix
    mat4.copy(modelViewMatrix, mat4.create());
    // Then, move from origin [0, 0, 0] to [0.5, 0.5, 0.5], and add the camera X and Y
    mat4.translate(modelViewMatrix, modelViewMatrix, [.5 + camera.x, .5 + camera.y, .5]);
    // Next, rotate the camera along the Z-axis (think yaw)
    mat4.rotateZ(modelViewMatrix, modelViewMatrix, -camera.zRot);
    // Then along the Y-axis (think pitch)
    mat4.rotateX(modelViewMatrix, modelViewMatrix, -camera.yRot);
    // Next, back the camera up by camera.distance
    mat4.translate(modelViewMatrix, modelViewMatrix, [0, 0, camera.dist**2]);
    // And invert it to get the matrix to move the world
    mat4.invert(modelViewMatrix, modelViewMatrix);

    drawTerrain({
      positions: positions3d,
      normals: normals3d,
      projectionMatrix: projectionMatrix,
      modelViewMatrix: modelViewMatrix,
      hillColor:  color('#d3feb0'),
      landColor:  color('#40a74c'),
      waterColor: color('#0cc4d6'),
      depthColor: color('#005e8b'),
      extent: [minHeight, seaLevel, maxHeight],
      count: triangleCount,
    });

    drawRivers({
      projectionMatrix: projectionMatrix,
      modelViewMatrix: modelViewMatrix,
      points: regl.buffer(riverPoints),
      widths: regl.buffer(riverWidths),
      color: color('#0d85c1'),
      segments: riverSegments,
    });

    drawCoasts({
      projectionMatrix: projectionMatrix,
      modelViewMatrix: modelViewMatrix,
      points: coastBuffer,
      width: 2.5e-3,
      color: color('#133b66'),
      // color: [1, 0, 1, 1], // FOR DEBUGGING
      segments: coastLines.length,
    });
  }

  draw({
    zRot: 0,
    yRot: 0,
    dist: 1,
    x: 0,
    y: 0,
  });

  return draw;
}
