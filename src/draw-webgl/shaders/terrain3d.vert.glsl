attribute highp vec4 position;
attribute vec4 normal;

uniform vec4 landColor;
uniform vec4 hillColor;
uniform vec4 waterColor;
uniform vec4 depthColor;
uniform mat4 modelView;
uniform mat4 projection;
uniform vec3 extent;

varying lowp vec4 vColor;
varying highp vec2 vXY;

void main(void) {
  float min = extent.x;
  float seaLevel = extent.y;
  float max = extent.z;

  highp vec3 lightDirection = normalize(vec3(0.2, .2, 1));

  float light = dot(normalize(normal.xyz), lightDirection);
  gl_Position = projection * modelView * position;

  if (position.z > seaLevel) {
    vColor = mix(landColor, hillColor, (position.z - seaLevel) / (max - seaLevel));
  } else {
    vColor = mix(depthColor, waterColor, (position.z - min) / (seaLevel - min));
  }
  // vColor.rgb = normalize(normal.xyz);
  vColor.rgb *= light;
  vXY = position.xy;
}
