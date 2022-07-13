const { Sim, GameEvent, Asset } = require('./sim')
const assert = require('assert')

const FRAMES_PER_SECOND = 60

const isReleaseBuild = !!process.env.RELEASE

let sim = null
beforeEach(async () => {
  sim = await Sim.fromWasmBinary(
    isReleaseBuild
      ? '../../target/wasm32-unknown-unknown/release/level_0.wasm'
      : '../../target/wasm32-unknown-unknown/debug/level_0.wasm'
  )
})

test('playground', () => {
  for (let i = 0; i < FRAMES_PER_SECOND * 5; i++) {
    if (i > FRAMES_PER_SECOND * 3) {
      sim.inputSetCrouch(true)
    }
    sim.step(1 / 60)
    console.log(sim.headGlobalTranslation())
  }
})
