uniform highp vec4 landColor;
uniform highp vec4 hillColor;
uniform highp vec4 cliffColor;
uniform highp vec4 waterColor;
uniform highp vec4 depthColor;
uniform highp vec3 extent;

varying lowp float vLight;
varying highp vec3 vPos;
varying highp vec3 vNormal;


void main(void) {
  if (vPos.x < 0.0 || vPos.y < 0.0 || vPos.x > 1.0 || vPos.y > 1.0) {
    gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
    return;
  }

  highp float min = extent.x;
  highp float seaLevel = extent.y;
  highp float max = extent.z;
  lowp vec4 color = vec4(0.0);

  if (vPos.z > seaLevel) {
    color = mix(landColor, hillColor, (vPos.z - seaLevel) / (max - seaLevel));
    color = mix(cliffColor, color, clamp(pow(vNormal.z, 6.0) + 0.2, 0.0, 1.0));
  } else {
    color = mix(depthColor, waterColor, (vPos.z - min) / (seaLevel - min));
  }

  gl_FragColor = vec4(color.rgb * vLight, 1);
}
