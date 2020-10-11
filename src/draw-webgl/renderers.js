import terrain3dVertShader from './shaders/terrain3d.vert.glsl';
import terrain3dFragShader from './shaders/terrain3d.frag.glsl';

import lineVertShader from './shaders/line.vert.glsl';
import varyingWidthLineVertShader from './shaders/varyingWidthLine.vert.glsl';
import lineFragShader from './shaders/line.frag.glsl';
import roundCapJoinGeometry from './roundCapJoinGeometry.js';


export default function getRenderers (regl) {
  const roundCapJoin = roundCapJoinGeometry(regl, 16);

  const drawTerrain = regl({
    vert: terrain3dVertShader,
    frag: terrain3dFragShader,
    attributes: {
      position: {
        buffer: regl.prop('positions'),
        size: 3
      },
      normal: {
        buffer: regl.prop('normals'),
        size: 3,
      },
    },
    uniforms: {
      projection: regl.prop('projectionMatrix'),
      modelView: regl.prop('modelViewMatrix'),
      landColor: regl.prop('landColor'),
      hillColor: regl.prop('hillColor'),
      waterColor: regl.prop('waterColor'),
      depthColor: regl.prop('depthColor'),
      zScale: regl.prop('zScale'),
      extent: regl.prop('extent'),
    },
    count: regl.prop('count'),
  });


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
      projection: regl.prop('projectionMatrix'),
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
      projection: regl.prop('projectionMatrix'),
      modelView: regl.prop('modelViewMatrix'),
    },
    depth: {
      enable: false,
      mask: false,
    },
    count: roundCapJoin.count,
    instances: regl.prop('segments'),
  });


  return {
    drawTerrain,
    drawCoasts,
    drawRivers
  }
}
