attribute vec4 aVertexPosition;
attribute vec4 aVertexColor;
attribute vec4 aVertexHeight;

uniform mat4 uModelViewMatrix;
uniform mat4 uProjectionMatrix;

varying lowp vec4 vColor;

void main(void) {
  gl_Position = uProjectionMatrix * uModelViewMatrix * aVertexPosition;
  vColor = vec4(aVertexHeight.x, aVertexHeight.x, aVertexHeight.x, aVertexHeight.x);
}
