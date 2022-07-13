import Controller from './controller.module.js'

class PointerLockInput extends Controller {
  constructor(domElement) {
    super()
    this._ready = !window.matchMedia('(max-width: 1024px)').matches
    this._domElement = domElement || document.body
    this._isLocked = false
    this._x = 0
    this._y = 0
    this._pressedButtonsList = []

    if (this._ready) {
      this.connect()
    }
  }

  ready() {
    return this._ready
  }

  getDx() {
    const x = this._x
    this._x = 0
    return x
  }

  getDy() {
    const y = this._y
    this._y = 0
    return y
  }

  isLocked() {
    return this._isLocked
  }

  lock() {
    this._domElement.requestPointerLock()
  }

  unlock() {
    this._domElement.ownerDocument.exitPointerLock()
  }

  connect() {
    this._domElement.addEventListener('mousemove', (e) => this.onMouseMove(e))
    this._domElement.ownerDocument.addEventListener('pointerlockchange', (e) =>
      this.onLockChange(e)
    )
    this._domElement.ownerDocument.addEventListener('pointerlockerror', (e) =>
      console.error('Pointer Lock Error', e)
    )
    this._domElement.addEventListener('mousedown', (e) => this.onMouseDown(e))
    this._domElement.addEventListener('mouseup', (e) => this.onMouseUp(e))
    this._domElement.addEventListener('contextmenu', (e) => {
      e.preventDefault()
    })
    this._domElement.addEventListener('click', (event) => {
      if (!this._isLocked) {
        this.lock()
      }
    })
  }

  onLockChange() {
    this._isLocked = !this._isLocked
    if (!this._isLocked) {
      this.unlock()
    }
  }

  onMouseMove(event) {
    if (this._isLocked) {
      this._x +=
        event.movementX || event.mozMovementX || event.webkitMovementX || 0
      this._y +=
        event.movementY || event.mozMovementY || event.webkitMovementY || 0
    }
  }

  onMouseDown(event) {
    const button = event.button
    if (!this._pressedButtonsList.includes(button)) {
      this._pressedButtonsList.push(button)
    }
  }

  onMouseUp(event) {
    const button = event.button
    if (this._pressedButtonsList.includes(button)) {
      for (let i = 0; i < this._pressedButtonsList.length; i++) {
        if (this._pressedButtonsList[i] === button) {
          this._pressedButtonsList.splice(i, 1)
        }
      }
    }
  }

  buttonPressed(button) {
    return this._pressedButtonsList.includes(button)
  }

  leftDown() {
    return this.buttonPressed(0)
  }

  middleDown() {
    return this.buttonPressed(1)
  }

  rightDown() {
    return this.buttonPressed(2)
  }
}

export default PointerLockInput
