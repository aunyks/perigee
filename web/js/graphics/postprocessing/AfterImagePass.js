// Used in:
// https://github.com/mrdoob/three.js/blob/master/examples/webgl_postprocessing_afterimage.html
import {
  LinearFilter,
  MeshBasicMaterial,
  NearestFilter,
  RGBAFormat,
  ShaderMaterial,
  UniformsUtils,
  WebGLRenderTarget,
} from '/js/graphics/three.module.js'
import { Pass } from './Pass.js'
import { AfterImageShader } from '../shaders/AfterImageShader.js'

var AfterImagePass = function (damp) {
  Pass.call(this)

  if (AfterImageShader === undefined)
    console.error('THREE.AfterImagePass relies on AfterImageShader')

  this.shader = AfterImageShader

  this.uniforms = UniformsUtils.clone(this.shader.uniforms)

  this.uniforms['damp'].value = damp !== undefined ? damp : 0.96

  this.textureComp = new WebGLRenderTarget(
    window.innerWidth,
    window.innerHeight,
    {
      minFilter: LinearFilter,
      magFilter: NearestFilter,
      format: RGBAFormat,
    }
  )

  this.textureOld = new WebGLRenderTarget(
    window.innerWidth,
    window.innerHeight,
    {
      minFilter: LinearFilter,
      magFilter: NearestFilter,
      format: RGBAFormat,
    }
  )

  this.shaderMaterial = new ShaderMaterial({
    uniforms: this.uniforms,
    vertexShader: this.shader.vertexShader,
    fragmentShader: this.shader.fragmentShader,
  })

  this.compFsQuad = new Pass.FullScreenQuad(this.shaderMaterial)

  var material = new MeshBasicMaterial()
  this.copyFsQuad = new Pass.FullScreenQuad(material)
}

AfterImagePass.prototype = Object.assign(Object.create(Pass.prototype), {
  constructor: AfterImagePass,

  render: function (renderer, writeBuffer, readBuffer) {
    this.uniforms['tOld'].value = this.textureOld.texture
    this.uniforms['tNew'].value = readBuffer.texture

    renderer.setRenderTarget(this.textureComp)
    this.compFsQuad.render(renderer)

    this.copyFsQuad.material.map = this.textureComp.texture

    if (this.renderToScreen) {
      renderer.setRenderTarget(null)
      this.copyFsQuad.render(renderer)
    } else {
      renderer.setRenderTarget(writeBuffer)

      if (this.clear) renderer.clear()

      this.copyFsQuad.render(renderer)
    }

    // Swap buffers.
    var temp = this.textureOld
    this.textureOld = this.textureComp
    this.textureComp = temp
    // Now textureOld contains the latest image, ready for the next frame.
  },

  setSize: function (width, height) {
    this.textureComp.setSize(width, height)
    this.textureOld.setSize(width, height)
  },
})

export { AfterImagePass }
