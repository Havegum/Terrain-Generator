uniform mat4 modelView;
uniform mat4 projection;

attribute highp vec4 position;
attribute vec4 normal;

varying lowp float vLight;
varying highp vec3 vPos;
varying highp vec3 vNormal;

void main(void) {
  highp vec3 lightDirection = normalize(vec3(0.2, .2, 1));
  float light = dot(normalize(normal.xyz), lightDirection);
  gl_Position = projection * modelView * position;

  // vColor.rgb = normalize(normal.xyz);
  vLight = light;
  vNormal = normalize(normal.xyz);
  vPos = position.xyz;
}
