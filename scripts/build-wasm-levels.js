const fs = require('fs')
const path = require('path')
const { spawnSync } = require('child_process')

const cwd = process.cwd()
const isReleaseBuild = !!process.env.RELEASE

if (isReleaseBuild) {
  console.log('****COMPILING WITH RELEASE BUILD!****')
} else {
  console.log('Compiling with debug build')
}

const gltfLevelsPath = path.join(cwd, 'assets', 'gltf', 'levels')
fs.readdirSync(gltfLevelsPath).forEach((fileOrDir) => {
  if (fs.statSync(path.join(gltfLevelsPath, fileOrDir)).isDirectory()) {
    const levelName = fileOrDir

    // Build the level to a WASM binary
    const wasmBuildFlags = [
      'build',
      '-p',
      'single_player_ffi',
      '--features',
      `level_${levelName}`,
      '--target',
      'wasm32-unknown-unknown',
    ]
    if (isReleaseBuild) {
      wasmBuildFlags.push('--release')
    }

    const wasmBuildCmd = spawnSync('cargo', wasmBuildFlags, { cwd: cwd })

    console.log(wasmBuildCmd.stderr.toString())
    console.log(wasmBuildCmd.stdout.toString())

    fs.renameSync(
      path.join(
        cwd,
        'target',
        'wasm32-unknown-unknown',
        isReleaseBuild ? 'release' : 'debug',
        'single_player_ffi.wasm'
      ),
      path.join(
        cwd,
        'target',
        'wasm32-unknown-unknown',
        isReleaseBuild ? 'release' : 'debug',
        `level_${levelName}.wasm`
      )
    )
  }
})
