import terrainVertShader from './shaders/terrain.vert.glsl';
import terrainFragShader from './shaders/terrain.frag.glsl';

import lineVertShader from './shaders/line.vert.glsl';
import lineFragShader from './shaders/line.frag.glsl';
import varyingWidthLineVertShader from './shaders/varyingWidthLine.vert.glsl';

import { interpolateYlGn as interpolateLand, interpolatePuBu as interpolateSea } from 'd3-scale-chromatic';
import { context } from 'gl-util';
import initScene from './init-scene.js';

import roundCapJoinGeometry from './roundCapJoinGeometry.js';
import REGL from 'regl';

export default function draw (canvas, triangles, points, circumcenters, heights, seaLevel, coasts, rivers) {
  const gl = context(canvas);
  const { projectionMatrix, modelViewMatrix } = initScene(gl);

  const regl = REGL({ gl, extensions: ['ANGLE_instanced_arrays'] });

  const positions = Float32Array.from(triangles.flat().flatMap((i, n) => n % 3 === 0 ? [points[i*2], points[i*2+1]] : [circumcenters[i*2], circumcenters[i*2+1]]));
  function interpolateHeight (i) {
    let height = heights[i]; if (isNaN(height)) return 'none';
    let color = height >= seaLevel ? interpolateLand(1 - height) : interpolateSea(1 - height);
    color = color.slice(4, -1).split(', ').map(n => n / 255); color[3] = 1; return color;
  }
  const colors = Float32Array.from(triangles.flatMap((_, i) => { let c = interpolateHeight(i); return [c, c, c].flat() }));


  const drawTerrain = regl({
    vert: terrainVertShader,
    frag: terrainFragShader,
    attributes: {
      position: {
        buffer: regl.buffer(positions),
        size: 2
      },
      color: colors
    },
    uniforms: {
      projection: projectionMatrix,
      modelView: modelViewMatrix,
    },
    count: triangles.flat().length
  });

  const segmentInstanceGeometry = [
    [0, -0.5],
    [1, -0.5],
    [1,  0.5],
    [0, -0.5],
    [1,  0.5],
    [0,  0.5]
  ];
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
      modelView: modelViewMatrix,
    },
    count: segmentInstanceGeometry.length,
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
        offset: Float32Array.BYTES_PER_ELEMENT * 0,
        stride: Float32Array.BYTES_PER_ELEMENT * 4,
      },
      pointB: {
        buffer: regl.prop('points'),
        divisor: 1,
        offset: Float32Array.BYTES_PER_ELEMENT * 2,
        stride: Float32Array.BYTES_PER_ELEMENT * 4,
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
      modelView: modelViewMatrix,
    },
    count: segmentInstanceGeometry.length,
    instances: regl.prop('segments'),
  });

  const coastBuffer = regl.buffer(coasts.flat().flat());
  drawCoasts({
    points: coastBuffer,
    width: 2.5e-3,
    color: [19/255, 59/255, 102/255, 1],
    // color: [1, 0, 1, 1], // FOR DEBUGGING
    segments: coasts.length
  });

  const riverSegments = rivers.reduce((sum, next) => sum + next.length - 1, 0) - 1;

  const riverPoints = rivers.flatMap(river =>
    river.flatMap((part, i, arr) => {
      if (i === arr.length - 1) return [];
      const index1 = part[0];
      const x1 = circumcenters[index1 * 2 + 0];
      const y1 = circumcenters[index1 * 2 + 1];
      const index2 = arr[i + 1][0];
      const x2 = circumcenters[index2 * 2 + 0];
      const y2 = circumcenters[index2 * 2 + 1];
      return [x1, y1, x2, y2];
    })
  );

  const riverCap = 80;
  const riverWidths = rivers.flatMap(river =>
    river.flatMap((part, i, arr) => {
      if (i === arr.length - 1) return [];
      return [i === 0 ? arr[i + 1][1] : part[1], arr[i + 1][1]];
    }).map(n => n <= riverCap ? 0 : Math.log((n - riverCap) * 5) * 4e-4)
  );

  drawRivers({
    points: regl.buffer(riverPoints),
    widths: regl.buffer(riverWidths),
    color: [13/255, 133/255, 193/255, 1],
    segments: riverSegments
  })

  drawTerrain();
}
