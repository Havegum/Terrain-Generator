precision highp float;

attribute vec3 position;
attribute vec3 pointA, pointB;
attribute float fluxA, fluxB;

uniform mat4 projection;
uniform mat4 modelView;

uniform float riverCap;

float scaleRiver(in float r) {
  if (r <= riverCap) {
    return 0.0;
  } else {
    return log((r - riverCap) * 5.0) * 0.0004;
  }
}

void main() {
  vec2 xBasis = normalize(pointB.xy - pointA.xy);
  vec2 yBasis = vec2(-xBasis.y, xBasis.x);
  vec2 offsetA = pointA.xy + scaleRiver(fluxA) * (position.x * xBasis + position.y * yBasis);
  vec2 offsetB = pointB.xy + scaleRiver(fluxB) * (position.x * xBasis + position.y * yBasis);
  vec2 point = mix(offsetA, offsetB, position.z);
  float height = mix(pointA.z, pointB.z, position.z);
  gl_Position = projection * modelView * vec4(point, height, 1);
}
