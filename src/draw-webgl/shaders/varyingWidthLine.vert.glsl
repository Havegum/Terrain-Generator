precision highp float;

attribute vec3 position;
attribute vec2 pointA, pointB;
attribute float widthA, widthB;

uniform mat4 projection;
uniform mat4 modelView;

void main() {
  vec2 xBasis = normalize(pointB - pointA);
  vec2 yBasis = vec2(-xBasis.y, xBasis.x);
  vec2 offsetA = pointA + widthA * (position.x * xBasis + position.y * yBasis);
  vec2 offsetB = pointB + widthB * (position.x * xBasis + position.y * yBasis);
  vec2 point = mix(offsetA, offsetB, position.z);
  gl_Position = projection * modelView * vec4(point, 0, 1);
}
