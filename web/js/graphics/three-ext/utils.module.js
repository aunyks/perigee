import { AudioLoader, TextureLoader } from '/js/graphics/three.module.js'
import GltfLoader from '/js/graphics/loaders/gltf-loader.module.js'

const gltfLoader = new GltfLoader()
function promiseLoadGltf(path) {
  return new Promise((resolve, reject) => {
    gltfLoader.load(
      path,
      (gltf) => {
        resolve(gltf)
      },
      () => {},
      (error) => reject(error)
    )
  })
}

function promiseParseGltf(arrayBuffer) {
  return new Promise((resolve, reject) => {
    gltfLoader.parse(
      arrayBuffer,
      '',
      (gltf) => {
        resolve(gltf)
      },
      (error) => reject(error)
    )
  })
}

const textureLoader = new TextureLoader()
function promiseLoadTexture(path) {
  return new Promise((resolve, reject) => {
    textureLoader.load(
      path,
      (texture) => {
        resolve(texture)
      },
      () => {},
      (error) => reject(error)
    )
  })
}

const audioLoader = new AudioLoader()
function promiseLoadAudioBuffer(path) {
  return new Promise((resolve, reject) => {
    audioLoader.load(
      path,
      (audioBuffer) => {
        resolve(audioBuffer)
      },
      () => {},
      (error) => reject(error)
    )
  })
}

export {
  promiseLoadAudioBuffer,
  promiseLoadTexture,
  promiseLoadGltf,
  promiseParseGltf,
}
