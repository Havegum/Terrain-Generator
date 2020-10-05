function normal (arr) {
  const [p1, p2, p3] = arr;
  const normal = new Array(3);

  // Set Vector U to (Triangle.p2 minus Triangle.p1)
  const U = [p2[0] - p1[0], p2[1] - p1[1], p2[2] - p1[2]];
  // Set Vector V to (Triangle.p3 minus Triangle.p1)
  const V = [p3[0] - p1[0], p3[1] - p1[1], p3[2] - p1[2]];

  // Set Normal.x to (multiply U.y by V.z) minus (multiply U.z by V.y)
  normal[0] = U[1] * V[2] - U[2] * V[1];
  // Set Normal.y to (multiply U.z by V.x) minus (multiply U.x by V.z)
  normal[1] = U[2] * V[0] - U[0] * V[2];
  // Set Normal.z to (multiply U.x by V.y) minus (multiply U.y by V.x)
  normal[2] = (U[0] * V[1] - U[1] * V[0]) * -1;
  // Returning Normal
  return normal.map(n => isNaN(n) ? 0 : n);
}

function color (str) {
  let colors = new Float32Array(4);
  colors[3] = 1;
  for (let i = 0; i < 3; i++)
    colors[i] = parseInt(str.substring(1+2*i, 3+2*i), 16) / 255;
  return colors;
}


export { color, normal };
