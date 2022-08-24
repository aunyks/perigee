import {
  CircleGeometry,
  MeshLambertMaterial,
  Object3D,
  Color,
  Mesh,
  Group,
} from '/js/graphics/three.module.js'

class Sun extends Group {
  constructor(_color, _scaleFactor) {
    super()
    const scaleFactor = _scaleFactor || 1
    const color = (_color || new Color(0xffffff)).convertLinearToSRGB()
    this._sunMesh = new Mesh(
      new CircleGeometry(5, 26),
      new MeshLambertMaterial({
        color: color,
        emissive: color,
        emissiveIntensity: 1000,
        flatShading: true,
      })
    )
    this.pivot = new Object3D()

    this.pivot.add(this._sunMesh)
    this.add(this.pivot)

    this._sunMesh.position.set(0, 0, -40)
    this._sunMesh.lookAt(0, 0, 0)
    this._sunMesh.scale.set(scaleFactor, scaleFactor, scaleFactor)

    // - 1 so that it's in front of the skydome
    this._sunMesh.renderOrder = -(Number.MAX_SAFE_INTEGER - 1)
    this._sunMesh.material.depthTest = false
  }
}

export default Sun
