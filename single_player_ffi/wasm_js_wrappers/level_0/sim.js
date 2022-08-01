function getUint8LE(memoryArrayBuffer, memoryAddress, startIndex) {
  if (memoryArrayBuffer.length - startIndex < 1)
    throw new Error(`Cannot get Uint8LE: index ${startIndex} is out of bounds`)
  const dataView = new DataView(memoryArrayBuffer, memoryAddress)
  return dataView.getUint8(startIndex)
}

function getUint32LE(memoryArrayBuffer, memoryAddress, startIndex) {
  if (memoryArrayBuffer.length - startIndex < 4)
    throw new Error(`Cannot get Uint32LE: index ${startIndex} is out of bounds`)
  const dataView = new DataView(memoryArrayBuffer, memoryAddress)
  return dataView.getUint32(startIndex, true)
}

function getFloat32LE(memoryArrayBuffer, memoryAddress, startIndex) {
  if (memoryArrayBuffer.length - startIndex < 4) {
    throw new Error(
      `Cannot get Float32LE: index ${startIndex} is out of bounds`
    )
  }
  const dataView = new DataView(memoryArrayBuffer, memoryAddress)
  return dataView.getFloat32(startIndex, true)
}

const U32_MAX = 2 ** 32 - 1

const PLAYER_EVENT_OFFSET = parseInt(U32_MAX / 3) * 0
const AUDIOVISUAL_EVENT_OFFSET = parseInt(U32_MAX / 3) * 1

const SOUND_ASSET_OFFSET = parseInt(U32_MAX / 2) * 0
const ANIMATION_ASSET_OFFSET = parseInt(U32_MAX / 2) * 1

const Asset = Object.freeze({
  Animation: {
    CameraIdle: ANIMATION_ASSET_OFFSET + 0,
    CameraRunning: ANIMATION_ASSET_OFFSET + 1,
  },
  Sound: {
    PlayerSighRelaxed: SOUND_ASSET_OFFSET + 0,
  },
})

const GameEvent = Object.freeze({
  // Player events
  Player: {
    Jump: PLAYER_EVENT_OFFSET + 0,
    Landed: PLAYER_EVENT_OFFSET + 1,
    Moving: PLAYER_EVENT_OFFSET + 2,
    Stopped: PLAYER_EVENT_OFFSET + 3,
    Crouched: PLAYER_EVENT_OFFSET + 4,
    StoodUpright: PLAYER_EVENT_OFFSET + 5,
    Stepped: PLAYER_EVENT_OFFSET + 6,
    StartedWallRunning: PLAYER_EVENT_OFFSET + 7,
    StoppedWallRunning: PLAYER_EVENT_OFFSET + 8,
    StartedSliding: PLAYER_EVENT_OFFSET + 9,
    StoppedSliding: PLAYER_EVENT_OFFSET + 10,
  },
  // Audiovisual events
  AudioVisual: {
    PlayAsset: AUDIOVISUAL_EVENT_OFFSET + 0,
    PauseAsset: AUDIOVISUAL_EVENT_OFFSET + 1,
    StopAsset: AUDIOVISUAL_EVENT_OFFSET + 2,
    LoopAsset: AUDIOVISUAL_EVENT_OFFSET + 3,
    FadeInAsset: AUDIOVISUAL_EVENT_OFFSET + 4,
    FadeOutAsset: AUDIOVISUAL_EVENT_OFFSET + 5,
    ClearAsset: AUDIOVISUAL_EVENT_OFFSET + 6,
  },
})

class GameEvents {
  constructor() {}

  on(type, cb) {
    this['_on' + type] = this['_on' + type] || []
    this['_on' + type].push(cb)
  }

  off(type, cb) {
    if (cb in this['_on' + type]) {
      for (let i = 0; i < this['_on' + type].length; i++) {
        if (this['_on' + type][i] === cb) {
          this['_on' + type].splice(i, 1)
        }
      }
    }
  }

  emit(type, args) {
    this['_on' + type] &&
      this['_on' + type].forEach((cb) => {
        cb(...args)
      })
  }
}

class Sim {
  static async fromWasmBinary(wasmPath) {
    const gameEventEmitter = new GameEvents()
    let wasmModule = null
    let wasmMemory = new WebAssembly.Memory({
      initial: 1,
      maximum: 2 ** 16,
    })
    const wasmFunctionImports = {
      now: () => performance.now(),
      on_event: (type, arg1, arg2) => {
        gameEventEmitter.emit(type, [arg1, arg2])
      },
    }
    const wasmImports = {
      env: wasmFunctionImports,
      js: {
        mem: wasmMemory,
      },
    }
    if (typeof window === 'undefined') {
      const wasmBuffer = require('fs').readFileSync(wasmPath)
      wasmModule = await WebAssembly.instantiate(wasmBuffer, wasmImports)
    } else {
      wasmModule = await WebAssembly.instantiateStreaming(
        fetch(wasmPath),
        wasmImports
      )
    }
    const wasmExports = wasmModule.instance.exports
    wasmMemory = wasmModule.instance.exports.memory

    const vecPtr = wasmExports.allocate_vector_space()
    const quatPtr = wasmExports.allocate_quaternion_space()
    const isoPtr = wasmExports.allocate_isometry_space()

    const gamePointer = wasmExports.create_sim()
    wasmExports.initialize_sim(gamePointer)

    const game = new Sim(
      wasmExports,
      wasmMemory,
      gamePointer,
      vecPtr,
      quatPtr,
      isoPtr,
      gameEventEmitter
    )
    return game
  }

  constructor(
    exports,
    memory,
    gamePtr,
    vecPtr,
    quatPtr,
    isoPtr,
    gameEventEmitter
  ) {
    this._wasmExports = exports
    this._wasmMemory = memory
    this._gamePointer = gamePtr
    this._vectorPointer = vecPtr
    this._quaternionPointer = quatPtr
    this._isometryPointer = isoPtr
    this.events = gameEventEmitter
  }

  destroySim() {
    this._wasmExports.destroy_sim(this._gamePointer)
  }

  inputSetMoveForward(newMagnitude) {
    this._wasmExports.input_set_move_forward(this._gamePointer, newMagnitude)
  }

  inputSetMoveRight(newMagnitude) {
    this._wasmExports.input_set_move_right(this._gamePointer, newMagnitude)
  }

  inputSetRotateUp(newMagnitude) {
    this._wasmExports.input_set_rotate_up(this._gamePointer, newMagnitude)
  }

  inputSetRotateRight(newMagnitude) {
    this._wasmExports.input_set_rotate_right(this._gamePointer, newMagnitude)
  }

  inputSetJump(jumpVal) {
    this._wasmExports.input_set_jump(this._gamePointer, jumpVal ? 1 : 0)
  }

  inputSetCrouch(crouchVal) {
    this._wasmExports.input_set_crouch(this._gamePointer, crouchVal ? 1 : 0)
  }

  step(deltaSeconds) {
    this._wasmExports.step_sim(this._gamePointer, deltaSeconds)
    this._wasmExports.get_game_events(this._gamePointer)
  }

  leftRightLookSensitivity() {
    return this._wasmExports.settings_left_right_look_sensitivity(
      this._gamePointer
    )
  }

  upDownLookSensitivity() {
    return this._wasmExports.settings_up_down_look_sensitivity(
      this._gamePointer
    )
  }

  setLeftRightLookSensitivity(newSensitivity) {
    this._wasmExports.settings_set_left_right_look_sensitivity(
      this._gamePointer,
      newSensitivity
    )
  }

  setUpDownLookSensitivity(newSensitivity) {
    this._wasmExports.settings_set_up_down_look_sensitivity(
      this._gamePointer,
      newSensitivity
    )
  }

  headGlobalTranslation() {
    this._wasmExports.head_global_translation(
      this._gamePointer,
      this._vectorPointer
    )
    return [
      getFloat32LE(this._wasmMemory.buffer, this._vectorPointer, 0),
      getFloat32LE(this._wasmMemory.buffer, this._vectorPointer, 4),
      getFloat32LE(this._wasmMemory.buffer, this._vectorPointer, 8),
    ]
  }

  headGlobalRotation() {
    this._wasmExports.head_global_rotation(
      this._gamePointer,
      this._quaternionPointer
    )
    return [
      getFloat32LE(this._wasmMemory.buffer, this._quaternionPointer, 4),
      getFloat32LE(this._wasmMemory.buffer, this._quaternionPointer, 8),
      getFloat32LE(this._wasmMemory.buffer, this._quaternionPointer, 12),
      getFloat32LE(this._wasmMemory.buffer, this._quaternionPointer, 0),
    ]
  }
}

module.exports = { Sim, GameEvent, Asset }
