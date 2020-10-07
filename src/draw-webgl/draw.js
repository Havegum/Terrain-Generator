import terrain3dVertShader from './shaders/terrain3d.vert.glsl';
import terrain3dFragShader from './shaders/terrain3d.frag.glsl';

import lineVertShader from './shaders/line.vert.glsl';
import lineFragShader from './shaders/line.frag.glsl';
import varyingWidthLineVertShader from './shaders/varyingWidthLine.vert.glsl';

import { context } from 'gl-util';
import initScene from './init-scene.js';
import { mat4, vec4 } from 'gl-matrix';
import { color, normal } from './utils.js';
import { extent } from 'd3-array';

import roundCapJoinGeometry from './roundCapJoinGeometry.js';
import REGL from 'regl';

export default function draw (canvas, triangles, points, circumcenters, triangleHeights, seaLevel, coastLines, rivers, cellHeights, heights) {
  const regl = REGL({ canvas, extensions: ['ANGLE_instanced_arrays'] });
  const { projectionMatrix, modelViewMatrix } = initScene(regl._gl, canvas);


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

  const draw3DTerrain = regl({
    vert: terrain3dVertShader,
    frag: terrain3dFragShader,
    attributes: {
      position: {
        buffer: regl.buffer(positions3d),
        size: 3
      },
      normal: {
        buffer: regl.buffer(normals3d),
        size: 3,
      },
    },
    uniforms: {
      projection: projectionMatrix,
      modelView: regl.prop('modelViewMatrix'),
      landColor: regl.prop('landColor'),
      hillColor: regl.prop('hillColor'),
      waterColor: regl.prop('waterColor'),
      depthColor: regl.prop('depthColor'),
      zScale: regl.prop('zScale'),
      extent: regl.prop('extent'),
    },
    count: triangleCount
  });

  const roundCapJoin = roundCapJoinGeometry(regl, 16);

  const drawCoasts = regl({
    vert: lineVertShader,
    frag: lineFragShader,
    attributes: {
      position: {
        buffer: roundCapJoin.buffer,
        divisor: 0,
      },
      pointA: {
        buffer: regl.prop('points'),
        divisor: 1,
        offset: Float32Array.BYTES_PER_ELEMENT * 0,
        stride: Float32Array.BYTES_PER_ELEMENT * 4,
      },
      pointB: {
        buffer: regl.prop('points'),
        divisor: 1,
        offset: Float32Array.BYTES_PER_ELEMENT * 2,
        stride: Float32Array.BYTES_PER_ELEMENT * 4,
      },
    },
    uniforms: {
      width: regl.prop('width'),
      color: regl.prop('color'),
      projection: projectionMatrix,
      modelView: regl.prop('modelViewMatrix'),
    },
    depth: {
      enable: false,
      mask: false,
    },
    count: roundCapJoin.count,
    instances: regl.prop('segments'),
  });

  const drawRivers = regl({
    vert: varyingWidthLineVertShader,
    frag: lineFragShader,
    attributes: {
      position: {
        buffer: roundCapJoin.buffer,
        divisor: 0,
      },
      pointA: {
        buffer: regl.prop('points'),
        divisor: 1,
        size: 3,
        offset: Float32Array.BYTES_PER_ELEMENT * 0,
        stride: Float32Array.BYTES_PER_ELEMENT * 6,
      },
      pointB: {
        buffer: regl.prop('points'),
        divisor: 1,
        size: 3,
        offset: Float32Array.BYTES_PER_ELEMENT * 3,
        stride: Float32Array.BYTES_PER_ELEMENT * 6,
      },
      widthA: {
        buffer: regl.prop('widths'),
        divisor: 1,
        offset: Float32Array.BYTES_PER_ELEMENT * 0,
        stride: Float32Array.BYTES_PER_ELEMENT * 2,
        size: 1,
      },
      widthB: {
        buffer: regl.prop('widths'),
        divisor: 1,
        offset: Float32Array.BYTES_PER_ELEMENT * 1,
        stride: Float32Array.BYTES_PER_ELEMENT * 2,
        size: 1,
      }
    },
    uniforms: {
      color: regl.prop('color'),
      projection: projectionMatrix,
      modelView: regl.prop('modelViewMatrix'),
    },
    depth: {
      enable: false,
      mask: false,
    },
    count: roundCapJoin.count,
    instances: regl.prop('segments'),
  });

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
  const riverCap = 80;
  const riverWidths = rivers.flatMap(river =>
    river.flatMap((part, i, arr) => {
      if (i === arr.length - 1) return [];
      return [i === 0 ? arr[i + 1][1] : part[1], arr[i + 1][1]];
    }).map(n => n <= riverCap ? 0 : Math.log((n - riverCap) * 5) * 4e-4)
  );

  const coastBuffer = regl.buffer(coastLines.flat().flat());

  const dist = 1;

  function draw () {
    regl.clear({
      color: [0, 0, 0, 1],
      depth: 1,
      stencil: 0
    })

    // CAMERA
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

    draw3DTerrain({
      modelViewMatrix: modelViewMatrix,
      hillColor:  color('#d3feb0'),
      landColor:  color('#40a74c'),
      waterColor: color('#0cc4d6'),
      depthColor: color('#005e8b'),
      extent: [minHeight, seaLevel, maxHeight],
    });

    drawRivers({
      modelViewMatrix: modelViewMatrix,
      points: regl.buffer(riverPoints),
      widths: regl.buffer(riverWidths),
      color: color('#0d85c1'),
      segments: riverSegments,
    });

    drawCoasts({
      modelViewMatrix: modelViewMatrix,
      points: coastBuffer,
      width: 2.5e-3,
      color: color('#133b66'),
      // color: [1, 0, 1, 1], // FOR DEBUGGING
      segments: coastLines.length,
    });
  }


  let camera = {
    zRot: 0,
    yRot: 0,
    dist: 1,
    x: 0,
    y: 0,
  };

  draw();


  return {
    setCamera: function (_camera) {
      camera = _camera;
      draw();
    }
  };
}
