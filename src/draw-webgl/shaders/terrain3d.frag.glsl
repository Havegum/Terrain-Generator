varying lowp vec4 vColor;
varying highp vec2 vXY;

void main(void) {
  if (vXY.x < 0.0 || vXY.y < 0.0 || vXY.x > 1.0 || vXY.y > 1.0) {
    gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
    return;
  }
  gl_FragColor = vec4(vColor.rgb, 1);
}
