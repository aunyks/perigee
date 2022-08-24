import * as fs from 'https://deno.land/std@0.152.0/node/fs.ts'
import * as path from 'https://deno.land/std/path/mod.ts'

const cwd = Deno.cwd()
const isReleaseBuild = !!Deno.env.get('RELEASE')

if (isReleaseBuild) {
  console.log('****COMPILING WITH RELEASE BUILD!****')
} else {
  console.log('Compiling with debug build')
}

const gltfLevelsPath = path.joinGlobs([cwd, 'assets', 'gltf', 'levels'])
fs.readdirSync(gltfLevelsPath).forEach((fileOrDir) => {
  if (fs.statSync(path.joinGlobs([gltfLevelsPath, fileOrDir])).isDirectory()) {
    const levelName = fileOrDir

    // Build the level to a WASM binary
    const wasmBuildFlags = [
      'build',
      '-p',
      'single_player',
      '--features',
      `ffi level_${levelName}`,
      '--target',
      'wasm32-unknown-unknown',
    ]
    if (isReleaseBuild) {
      wasmBuildFlags.push('--release')
    }

    const wasmBuildCmd = Deno.spawnSync(
      'cargo',
      { args: wasmBuildFlags },
      { cwd: cwd }
    )

    console.log(new TextDecoder().decode(wasmBuildCmd.stderr))
    console.log(new TextDecoder().decode(wasmBuildCmd.stdout))

    fs.renameSync(
      path.joinGlobs([
        cwd,
        'target',
        'wasm32-unknown-unknown',
        isReleaseBuild ? 'release' : 'debug',
        'single_player.wasm',
      ]),
      path.joinGlobs([
        cwd,
        'target',
        'wasm32-unknown-unknown',
        isReleaseBuild ? 'release' : 'debug',
        `level_${levelName}.wasm`,
      ])
    )

    if (wasmBuildCmd.code !== 0) {
      Deno.exit(1)
    }
  }
})
