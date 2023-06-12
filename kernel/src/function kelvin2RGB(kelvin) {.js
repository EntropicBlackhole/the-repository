function clamp(x, max, min) {
  return Math.min(Math.max(x, min), max);
}

function kelvin2RGB(kelvin) {
  const temp = kelvin / 100;
  const color = { r: 0, g: 0, b: 0 };
  if (temp <= 66) {
    color.r = 255;

    color.g = temp;
    color.g = 99.4708025861 * Math.log(color.g) - 161.1195681661;
    color.g = clamp(color.g, 255, 0);
  } else {
    color.r = temp - 60;
    color.r = 329.698727446 * Math.pow(color.r, -0.1332047592);
    color.r = clamp(color.r, 255, 0);

    color.g = temp - 60;
    color.g = 288.1221695283 * Math.pow(color.g, -0.0755148492);
    color.g = clamp(color.g, 255, 0);
  }

  if (temp >= 66) {
    color.b = 255;
  } else {
    if (temp <= 19) {
      color.b = 0;
    } else {
      color.b = temp - 10;
      color.b = 138.5177312231 * Math.log(color.b) - 305.0447927307;
      color.b = clamp(color.b, 255, 0);
    }
  }
  return color;
}
