precision highp float;

attribute vec3 position;
attribute vec3 pointA, pointB;
attribute float widthA, widthB;

uniform mat4 projection;
uniform mat4 modelView;

void main() {
  vec2 xBasis = normalize(pointB.xy - pointA.xy);
  vec2 yBasis = vec2(-xBasis.y, xBasis.x);
  vec2 offsetA = pointA.xy + widthA * (position.x * xBasis + position.y * yBasis);
  vec2 offsetB = pointB.xy + widthB * (position.x * xBasis + position.y * yBasis);
  vec2 point = mix(offsetA, offsetB, position.z);
  float height = mix(pointA.z, pointB.z, position.z);
  gl_Position = projection * modelView * vec4(point, height, 1);
}
