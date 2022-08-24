import { beforeEach, it } from 'https://deno.land/std@0.152.0/testing/bdd.ts'
import { Sim } from './sim.js'

const FRAMES_PER_SECOND = 60
const DELTA_SECONDS = 1 / FRAMES_PER_SECOND

const isReleaseBuild = !!Deno.env.get('RELEASE')

let sim = null
beforeEach(async () => {
  sim = await Sim.fromWasmBinary(
    isReleaseBuild
      ? // These paths are with the working directory of the justfile
        'target/wasm32-unknown-unknown/release/level_0.wasm'
      : 'target/wasm32-unknown-unknown/debug/level_0.wasm'
  )
  sim.initialize()
})

it('playground', () => {
  for (let i = 0; i < FRAMES_PER_SECOND * 0.75; i++) {
    sim.inputSetMoveForward(-1)
    sim.step(DELTA_SECONDS)
  }
})
