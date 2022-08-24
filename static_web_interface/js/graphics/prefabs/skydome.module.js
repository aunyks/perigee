import {
  SphereGeometry,
  ShaderMaterial,
  Uniform,
  Color,
  BackSide,
  Mesh,
} from '/js/graphics/three.module.js'

class SkyDome extends Mesh {
  constructor(_horizonColor, _skyColor, _blendHeight, _blendFactor) {
    const horizonColor = (
      _horizonColor || new Color(0x80d9ff)
    ).convertLinearToSRGB()
    const skyColor = (_skyColor || new Color(0x00b2ff)).convertLinearToSRGB()
    const blendHeight = _blendHeight || 0.55
    const blendFactor = _blendFactor || 0.002

    super(
      new SphereGeometry(10, 15, 16),
      new ShaderMaterial({
        side: BackSide,
        vertexShader: VERTEX_SHADER,
        fragmentShader: FRAGMENT_SHADER,
        uniforms: {
          horizonColor: new Uniform(horizonColor),
          skyColor: new Uniform(skyColor),
          blendHeight: new Uniform(blendHeight),
          blendFactor: new Uniform(blendFactor),
        },
      })
    )

    this.renderOrder = -Number.MAX_SAFE_INTEGER
    this.material.depthTest = false
  }
}

const VERTEX_SHADER = `
varying vec2 vUv;
uniform vec3 horizonColor;
uniform vec3 skyColor;
uniform float blendHeight;
uniform float blendFactor;

void main() {
  vUv = uv;
  gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
}
`

const FRAGMENT_SHADER = `
varying vec2 vUv;
uniform vec3 horizonColor;
uniform vec3 skyColor;
uniform float blendHeight;
uniform float blendFactor;

float algebraicSigmoid(float x, float a, float b, float c) {
  return (a * x - b) / sqrt(c + pow(a * x - b, 2.0));
}

float horizonCurve(float x, float blendFactor, float blendHeight) {
  return 0.5 * algebraicSigmoid(x, 1.0, blendHeight, blendFactor) + 0.5;
}

void main() {
  gl_FragColor = vec4(
    mix(horizonColor, skyColor, horizonCurve(vUv.y, blendFactor, blendHeight)),
    1.0
  );
}
`

export default SkyDome
