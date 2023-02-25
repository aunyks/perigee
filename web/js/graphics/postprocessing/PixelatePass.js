import { ShaderMaterial, Vector4 } from '/js/graphics/three.module.js'
import { Pass, FullScreenQuad } from './Pass.js'

class PixelatePass extends Pass {
  constructor(resolution) {
    super()
    this.resolution = resolution
    this.fsQuad = new FullScreenQuad(this.material())
  }

  render(renderer, writeBuffer, readBuffer) {
    const uniforms = this.fsQuad.material.uniforms
    uniforms.tDiffuse.value = readBuffer.texture
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
                uniform vec4 resolution;
                varying vec2 vUv;
                void main() {
                    vec2 iuv = (floor(resolution.xy * vUv) + .5) * resolution.zw;
                    vec4 texel = texture2D( tDiffuse, iuv );
                    gl_FragColor = texel;
                }
                `,
    })
  }
}

export { PixelatePass }
