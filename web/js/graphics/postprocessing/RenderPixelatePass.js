import {
  NearestFilter,
  RGBAFormat,
  RGBFormat,
  DepthTexture,
  MeshNormalMaterial,
  ShaderMaterial,
  Vector4,
  WebGLRenderTarget,
} from '/js/graphics/three.module.js'
import { Pass, FullScreenQuad } from './Pass.js'

export default class RenderPixelatedPass extends Pass {
  constructor(resolution, scene, camera) {
    super()
    this.resolution = resolution
    this.fsQuad = new FullScreenQuad(this.material())
    this.scene = scene
    this.camera = camera

    this.rgbRenderTarget = pixelRenderTarget(resolution, RGBAFormat, true)
    this.normalRenderTarget = pixelRenderTarget(resolution, RGBFormat, false)

    this.normalMaterial = new MeshNormalMaterial()
  }

  render(renderer, writeBuffer) {
    renderer.setRenderTarget(this.rgbRenderTarget)
    renderer.render(this.scene, this.camera)

    const overrideMaterial_old = this.scene.overrideMaterial
    renderer.setRenderTarget(this.normalRenderTarget)
    this.scene.overrideMaterial = this.normalMaterial
    renderer.render(this.scene, this.camera)
    this.scene.overrideMaterial = overrideMaterial_old

    // @ts-ignore
    const uniforms = this.fsQuad.material.uniforms
    uniforms.tDiffuse.value = this.rgbRenderTarget.texture
    uniforms.tDepth.value = this.rgbRenderTarget.depthTexture
    uniforms.tNormal.value = this.normalRenderTarget.texture

    if (this.renderToScreen) {
      renderer.setRenderTarget(null)
    } else {
      renderer.setRenderTarget(writeBuffer)
      if (this.clear) renderer.clear()
    }
    this.fsQuad.render(renderer)
  }

  material() {
    return new ShaderMaterial({
      uniforms: {
        tDiffuse: { value: null },
        tDepth: { value: null },
        tNormal: { value: null },
        resolution: {
          value: new Vector4(
            this.resolution.x,
            this.resolution.y,
            1 / this.resolution.x,
            1 / this.resolution.y
          ),
        },
      },
      vertexShader: `
                varying vec2 vUv;
                void main() {
                    vUv = uv;
                    gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );
                }
                `,
      fragmentShader: `
                uniform sampler2D tDiffuse;
                uniform sampler2D tDepth;
                uniform sampler2D tNormal;
                uniform vec4 resolution;
                varying vec2 vUv;
                float getDepth(int x, int y) {
                    return texture2D( tDepth, vUv + vec2(x, y) * resolution.zw ).r;
                }
                vec3 getNormal(int x, int y) {
                    return texture2D( tNormal, vUv + vec2(x, y) * resolution.zw ).rgb * 2.0 - 1.0;
                }
                float neighborNormalEdgeIndicator(int x, int y, float depth, vec3 normal) {
                    float depthDiff = getDepth(x, y) - depth;
                    
                    // Edge pixels should yield to faces closer to the 
                    vec3 normalEdgeBias = vec3(1., 1., 1.); // This should probably be a parameter.
                    float normalDiff = dot(normal - getNormal(x, y), normalEdgeBias);
                    float normalIndicator = clamp(smoothstep(-.01, .01, normalDiff), 0.0, 1.0);
                    
                    // Only the shallower pixel should detect the normal edge.
                    float depthIndicator = clamp(sign(depthDiff * .25 + .0025), 0.0, 1.0);
                    return distance(normal, getNormal(x, y)) * depthIndicator * normalIndicator;
                }
                float depthEdgeIndicator() {
                    float depth = getDepth(0, 0);
                    vec3 normal = getNormal(0, 0);
                    float diff = 0.0;
                    diff += clamp(getDepth(1, 0) - depth, 0.0, 1.0);
                    diff += clamp(getDepth(-1, 0) - depth, 0.0, 1.0);
                    diff += clamp(getDepth(0, 1) - depth, 0.0, 1.0);
                    diff += clamp(getDepth(0, -1) - depth, 0.0, 1.0);
                    return floor(smoothstep(0.01, 0.02, diff) * 2.) / 2.;
                }
                float normalEdgeIndicator() {
                    float depth = getDepth(0, 0);
                    vec3 normal = getNormal(0, 0);
                    
                    float indicator = 0.0;
                    indicator += neighborNormalEdgeIndicator(0, -1, depth, normal);
                    indicator += neighborNormalEdgeIndicator(0, 1, depth, normal);
                    indicator += neighborNormalEdgeIndicator(-1, 0, depth, normal);
                    indicator += neighborNormalEdgeIndicator(1, 0, depth, normal);
                    return step(0.1, indicator);
                }
                float lum(vec4 color) {
                    vec4 weights = vec4(.2126, .7152, .0722, .0);
                    return dot(color, weights);
                }
                float smoothSign(float x, float radius) {
                    return smoothstep(-radius, radius, x) * 2.0 - 1.0;
                }
                void main() {
                    vec4 texel = texture2D( tDiffuse, vUv );
                    float tLum = lum(texel);
                    // float normalEdgeCoefficient = (smoothSign(tLum - .3, .1) + .7) * .25;
                    // float depthEdgeCoefficient = (smoothSign(tLum - .3, .1) + .7) * .3;
                    float normalEdgeCoefficient = .3;
                    float depthEdgeCoefficient = .4;
                    float dei = depthEdgeIndicator();
                    float nei = normalEdgeIndicator();
                    float coefficient = dei > 0.0 ? (1.0 - depthEdgeCoefficient * dei) : (1.0 + normalEdgeCoefficient * nei);
                    gl_FragColor = texel * coefficient;
                }
                `,
    })
  }
}

function pixelRenderTarget(resolution, pixelFormat, depthTexture) {
  const renderTarget = new WebGLRenderTarget(
    resolution.x,
    resolution.y,
    !depthTexture
      ? undefined
      : {
          depthTexture: new DepthTexture(resolution.x, resolution.y),
          depthBuffer: true,
        }
  )
  renderTarget.texture.format = pixelFormat
  renderTarget.texture.minFilter = NearestFilter
  renderTarget.texture.magFilter = NearestFilter
  renderTarget.texture.generateMipmaps = false
  renderTarget.stencilBuffer = false
  return renderTarget
}
