import {
  InstancedMesh,
  Object3D,
  BufferGeometry,
  Material,
} from '/js/graphics/three.module.js'

/**
 * Creates a specialized InstancedMesh that places instances
 * on each vertex of the provided `terrainGeometry`. This is useful
 * for distributing grass and foliage across terrain meshes.
 *
 * I (@aunyks) am not sure whether the vertex positions are in local
 * or world space. It's safest to assume they're in local space.
 */
class TerrainInstancedMesh extends InstancedMesh {
  /**
   *
   * @param {BufferGeometry} terrainGeometry - The geometry of the terrain across which instances will be distributed.
   * @param {BufferGeometry} instancedGeometry - The geometry of the instanced mesh.
   * @param {Material} instancedMaterial - The material of the instanced mesh.
   */
  constructor(terrainGeometry, instancedGeometry, instancedMaterial) {
    // Create one mesh instance per terrainGeometry vertex
    const terrainVertexPositions = terrainGeometry.attributes.position
    super(instancedGeometry, instancedMaterial, terrainVertexPositions.count)

    // Set the position of each mesh to the (local? world? not sure) position of the
    // its respective vertex on terrainGeometry.
    const dummy = new Object3D()
    for (let i = 0; i < terrainVertexPositions.count; i++) {
      dummy.position.fromBufferAttribute(terrainVertexPositions, i)
      dummy.updateMatrix()
      this.setMatrixAt(i, dummy.matrix)
    }
  }
}

export default TerrainInstancedMesh
