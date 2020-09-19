attribute vec4 position;
attribute vec4 color;

uniform mat4 modelView;
uniform mat4 projection;

varying lowp vec4 vColor;

void main(void) {
  gl_Position = projection * modelView * position;
  vColor = color;
}
