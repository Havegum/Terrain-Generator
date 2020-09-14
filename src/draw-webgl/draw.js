import { mat4 } from 'gl-matrix';
import { initShaderProgram, loadShader, vsSource, fsSource } from './shader.js';
import initScene from './init-scene.js';

import { interpolateYlGn as interpolateLand, interpolatePuBu as interpolateSea } from 'd3-scale-chromatic';

function initBuffers(gl, positions, colors) {
  const positionBuffer = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
  gl.bufferData(gl.ARRAY_BUFFER, positions, gl.STATIC_DRAW);

  const colorBuffer = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, colorBuffer);
  gl.bufferData(gl.ARRAY_BUFFER, colors, gl.STATIC_DRAW);

  // const heightBuffer = gl.createBuffer();
  // gl.bindBuffer(gl.ARRAY_BUFFER, heightBuffer);
  // gl.bufferData(gl.ARRAY_BUFFER, heights, gl.STATIC_DRAW);

  return {
    position: positionBuffer,
    color: colorBuffer,
    // height: heightBuffer,
  };
}


function drawScene(gl, programInfo, buffers, vertexCount) {
  const { projectionMatrix, modelViewMatrix } = initScene(gl);

  {
    // Tell WebGL how to pull out the positions from the position buffer into the vertexPosition attribute.
    const numComponents = 2;  // pull out 2 values per iteration
    const type = gl.FLOAT;    // the data in the buffer is 32bit floats
    const normalize = false;  // don't normalize
    const stride = 0;         // how many bytes to get from one set of values to the next
    const offset = 0;         // how many bytes inside the buffer to start from
    gl.bindBuffer(gl.ARRAY_BUFFER, buffers.position);
    gl.vertexAttribPointer(
        programInfo.attribLocations.vertexPosition,
        numComponents,
        type,
        normalize,
        stride,
        offset
    );
    gl.enableVertexAttribArray(programInfo.attribLocations.vertexPosition);
  }

  {
    const numComponents = 4;
    const type = gl.FLOAT;
    const normalize = false;
    const stride = 0;
    const offset = 0;
    gl.bindBuffer(gl.ARRAY_BUFFER, buffers.color);
    gl.vertexAttribPointer(
        programInfo.attribLocations.vertexColor,
        numComponents,
        type,
        normalize,
        stride,
        offset
    );
    gl.enableVertexAttribArray(programInfo.attribLocations.vertexColor);
  }

  // {
  //   // Height component
  //   const numComponents = 1;
  //   const type = gl.FLOAT;
  //   const normalize = false;
  //   const stride = 0;
  //   const offset = 0;
  //   gl.bindBuffer(gl.ARRAY_BUFFER, buffers.height);
  //   gl.vertexAttribPointer(
  //       programInfo.attribLocations.vertexHeight,
  //       numComponents,
  //       type,
  //       normalize,
  //       stride,
  //       offset
  //   );
  //   gl.enableVertexAttribArray(programInfo.attribLocations.vertexHeight);
  // }

  gl.useProgram(programInfo.program); // Tell WebGL to use our program when drawing

  // Set the shader uniforms
  gl.uniformMatrix4fv(
      programInfo.uniformLocations.projectionMatrix,
      false,
      projectionMatrix);
  gl.uniformMatrix4fv(
      programInfo.uniformLocations.modelViewMatrix,
      false,
      modelViewMatrix);

  {
    const first = 0;
    gl.drawArrays(gl.TRIANGLES, first, vertexCount);
  }
}



export default function draw (canvas, triangles, points, circumcenters, heights, seaLevel) {
  const gl = canvas.getContext('webgl');
  if (gl === null) {
    alert('Unable to initialize WebGL. Your browser or machine may not support it.');
    return;
  }

  const shaderProgram = initShaderProgram(gl, vsSource, fsSource);

  const programInfo = {
    program: shaderProgram,
    attribLocations: {
      vertexPosition: gl.getAttribLocation(shaderProgram, 'aVertexPosition'),
      vertexColor: gl.getAttribLocation(shaderProgram, 'aVertexColor'),
      vertexHeight: gl.getAttribLocation(shaderProgram, 'aVertexHeight'),
    },
    uniformLocations: {
      projectionMatrix: gl.getUniformLocation(shaderProgram, 'uProjectionMatrix'),
      modelViewMatrix: gl.getUniformLocation(shaderProgram, 'uModelViewMatrix'),
    },
  };

  const positions = Float32Array.from(triangles.flat().flatMap((i, n) => n % 3 === 0
    ? [points[i*2], points[i*2+1]]
    : [circumcenters[i*2], circumcenters[i*2+1]]
  )).map(n => n * 2 - 1);

  function interpolateHeight (i) {
    let height = heights[i];
    if (height === undefined) return 'none';

    let color = height >= seaLevel
      ? interpolateLand(1 - height)
      : interpolateSea(1 - height);
    color = color.slice(4, -1).split(', ').map(n => n / 255);
    color[3] = 1;
    return color;
  }

  const colors = Float32Array.from(
    triangles.flatMap((_, i) => {
      let c = interpolateHeight(i);
      return [c, c, c].flat();
    })
  );





  console.log(interpolateHeight(0));


  const buffers = initBuffers(gl, positions, colors);
  drawScene(gl, programInfo, buffers, triangles.flat().length);
}
