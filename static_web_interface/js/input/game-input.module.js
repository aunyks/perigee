import GamepadInput from './gamepad.module.js'
import KeyboardInput from './keyboard.module.js'
import PointerLockInput from './pointerlock.module.js'
import TouchInput from './touch.module.js'
import VirtualJoystickInput from './virtual-joystick.module.js'
import VirtualJumpBtnInput from './virtual-jumpbtn.module.js'
import VirtualCrouchBtnInput from './virtual-crouchbtn.module.js'

// Mirrors the Input struct in the core crate
class GameInput {
  constructor() {
    this._moveForward = 0
    this._moveRight = 0
    this._rotateUp = 0
    this._rotateRight = 0
    this._jump = false
    this._crouch = false
    this._aim = false
  }

  moveForward() {
    return this._moveForward
  }

  moveRight() {
    return this._moveRight
  }

  rotateUp() {
    return this._rotateUp
  }

  rotateRight() {
    return this._rotateRight
  }

  jump() {
    return this._jump
  }

  crouch() {
    return this._crouch
  }

  aim() {
    return this._aim
  }

  setMoveForward(newMagnitude) {
    this._moveForward = newMagnitude
  }

  setMoveRight(newMagnitude) {
    this._moveRight = newMagnitude
  }

  setRotateUp(newMagnitude) {
    this._rotateUp = newMagnitude
  }

  setRotateRight(newMagnitude) {
    this._rotateRight = newMagnitude
  }

  setJump(jumpState) {
    this._jump = jumpState
  }

  setCrouch(crouchState) {
    this._crouch = crouchState
  }

  setAim(aimState) {
    this._aim = aimState
  }
}

function processInputs(inputs, gameInput) {
  for (const input of inputs) {
    if (input.ready()) {
      if (input instanceof KeyboardInput) {
        if (input.wPressed() || input.upPressed()) {
          gameInput.setMoveForward(-1)
        } else if (input.sPressed() || input.downPressed()) {
          gameInput.setMoveForward(1)
        } else {
          gameInput.setMoveForward(0)
        }
        if (input.aPressed() || input.leftPressed()) {
          gameInput.setMoveRight(-1)
        } else if (input.dPressed() || input.rightPressed()) {
          gameInput.setMoveRight(1)
        } else {
          gameInput.setMoveRight(0)
        }
        gameInput.setJump(input.spacebarPressed())
        gameInput.setCrouch(input.cPressed())
      } else if (input instanceof PointerLockInput) {
        if (input.isLocked()) {
          gameInput.setRotateRight(input.getDx() * 0.25)
          gameInput.setRotateUp(-input.getDy() * 0.25)
          gameInput.setAim(input.rightDown())
        }
      } else if (input instanceof VirtualJoystickInput) {
        const stickPos = input.getStickPos()
        gameInput.setMoveRight(stickPos.x)
        gameInput.setMoveForward(stickPos.y)
      } else if (input instanceof TouchInput) {
        gameInput.setRotateRight(input.getDx() * 0.3)
        gameInput.setRotateUp(-input.getDy() * 0.3)
      } else if (input instanceof VirtualJumpBtnInput) {
        gameInput.setJump(input.btnPressed())
      } else if (input instanceof VirtualCrouchBtnInput) {
        gameInput.setCrouch(input.btnPressed())
      } else if (input instanceof GamepadInput) {
        const leftStickPos = input.getLeftStickPos()
        const rightStickPos = input.getRightStickPos()

        gameInput.setJump(input.bPadSouthPressed())
        gameInput.setCrouch(input.bPadEastPressed())
        gameInput.setAim(input.leftTriggerPressed())

        gameInput.setMoveRight(leftStickPos.x)
        gameInput.setMoveForward(leftStickPos.y)

        gameInput.setRotateRight(rightStickPos.x * 0.5)
        gameInput.setRotateUp(-rightStickPos.y * 0.3)
      } else {
        console.warn(`Unrecognized input ${input} provided for processing`)
      }
    }
  }
}

function collectInputsIntoSimulation(gameInput, gameSimulation) {
  gameSimulation.inputSetRotateUp(gameInput.rotateUp())
  gameSimulation.inputSetRotateRight(gameInput.rotateRight())
  gameSimulation.inputSetMoveForward(gameInput.moveForward())
  gameSimulation.inputSetMoveRight(gameInput.moveRight())
  gameSimulation.inputSetJump(gameInput.jump())
  gameSimulation.inputSetCrouch(gameInput.crouch())
  gameSimulation.inputSetAim(gameInput.aim())
}

export { GameInput, processInputs, collectInputsIntoSimulation }
