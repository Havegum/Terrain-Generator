import { mat4 } from 'gl-matrix';

function initScene (gl) {
  gl.clearColor(0.0, 0.0, 0.0, 1.0);  // Clear to black, fully opaque
  gl.clearDepth(1.0);                 // Clear everything
  gl.enable(gl.DEPTH_TEST);           // Enable depth testing
  gl.depthFunc(gl.LEQUAL);            // Near things obscure far things
  gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT); // Clear the canvas before we start drawing on it.

  // Setup "camera"
  const fieldOfView = 20 * Math.PI / 180;   // in radians
  const aspect = gl.canvas.clientWidth / gl.canvas.clientHeight;
  const zNear = 0.1;
  const zFar = 100.0;
  const projectionMatrix = mat4.create();
  mat4.perspective(projectionMatrix, fieldOfView, aspect, zNear, zFar);
  // NOTE: glmatrix.js always has the first argument as the destination to receive the result.
  const modelViewMatrix = mat4.create();// Set the drawing position to the "identity" point, which is the center of the scene.
  // // Now move the drawing position a bit to where we want to start drawing the square.
  mat4.translate(modelViewMatrix,     // destination matrix
                 modelViewMatrix,     // matrix to translate
                 [-.5, -.5, -5]);  // amount to translate
 // mat4.translate(modelViewMatrix,     // destination matrix
 //                modelViewMatrix,     // matrix to translate
 //                [0, -0.1, 0]);  // amount to translate
 // mat4.rotateY(modelViewMatrix,
 //              modelViewMatrix,
 //              -Math.PI/2);

  return { projectionMatrix, modelViewMatrix };
}

export default initScene;
