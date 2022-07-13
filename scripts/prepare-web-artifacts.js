const fs = require('fs')
const path = require('path')

const cwd = process.cwd()
const isReleaseBuild = !!process.env.RELEASE

// Copy player sliding sounds into the web_interface
fs.copyFileSync(
  path.join(cwd, 'assets', 'audio', 'slide.mp3'),
  path.join(cwd, 'web_interface', 'audio', 'player', 'slide.mp3'),
  fs.constants.COPYFILE_FICLONE
)

// Copy player jump sounds into the web_interface
fs.copyFileSync(
  path.join(cwd, 'assets', 'audio', 'jump.mp3'),
  path.join(cwd, 'web_interface', 'audio', 'player', 'jump.mp3'),
  fs.constants.COPYFILE_FICLONE
)

// Copy player footstep sounds into the web_interface
fs.copyFileSync(
  path.join(cwd, 'assets', 'audio', 'footstep.mp3'),
  path.join(cwd, 'web_interface', 'audio', 'player', 'footstep.mp3'),
  fs.constants.COPYFILE_FICLONE
)

// Copy player camera animations from assets folder to
// the web_interface glTF folder
fs.copyFileSync(
  path.join(cwd, 'assets', 'gltf', 'player-camera.glb'),
  path.join(cwd, 'web_interface', 'gltf', 'player', 'animated-camera.glb'),
  fs.constants.COPYFILE_FICLONE
)

const gltfLevelsPath = path.join(cwd, 'assets', 'gltf', 'levels')
fs.readdirSync(gltfLevelsPath).forEach((fileOrDir) => {
  if (fs.statSync(path.join(gltfLevelsPath, fileOrDir)).isDirectory()) {
    const levelName = fileOrDir

    // Copy the level's visual setting glTF from the assets folder to the
    // the web_interface glTF folder. Use the physics world if the graphics
    // one doesn't explicitly exist
    const graphicsGltfPath = fs.existsSync(
      path.join(
        cwd,
        'assets',
        'gltf',
        'levels',
        levelName,
        'graphics-world.glb'
      )
    )
      ? path.join(
          cwd,
          'assets',
          'gltf',
          'levels',
          levelName,
          'graphics-world.glb'
        )
      : path.join(
          cwd,
          'assets',
          'gltf',
          'levels',
          levelName,
          'physics-world.glb'
        )
    fs.copyFileSync(
      graphicsGltfPath,
      path.join(
        cwd,
        'web_interface',
        'gltf',
        'levels',
        levelName,
        'graphics-world.glb'
      ),
      fs.constants.COPYFILE_FICLONE
    )

    // Copy the built WASM binary for the level into
    // the web_interface WASM folder
    fs.copyFileSync(
      path.join(
        cwd,
        'target',
        'wasm32-unknown-unknown',
        isReleaseBuild ? 'release' : 'debug',
        `level_${levelName}.wasm`
      ),
      path.join(cwd, 'web_interface', 'wasm', 'levels', levelName, 'sim.wasm'),
      fs.constants.COPYFILE_FICLONE
    )

    const outputSimFilePath = path.join(
      cwd,
      'web_interface',
      'js',
      'levels',
      levelName,
      'Sim.module.js'
    )

    // Copy the level's JavaScript wrapper from the WASM crate / module
    // to the web_interface simulations folder
    fs.copyFileSync(
      path.join(
        cwd,
        'single_player_ffi',
        'wasm_js_wrappers',
        `level_${levelName}`,
        'sim.js'
      ),
      outputSimFilePath,
      fs.constants.COPYFILE_FICLONE
    )

    // Convert the web_interface simulation wrapper's export format from
    // CommonJS to ES6 to be used as a web JS module
    const originalSimFile = fs.readFileSync(outputSimFilePath).toString()
    const es6ExportsSimFile = originalSimFile.replaceAll(
      'module.exports = ',
      'export '
    )
    fs.writeFileSync(outputSimFilePath, es6ExportsSimFile)
  }
})
