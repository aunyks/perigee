import Controller from './controller.module.js'

class VirtualBtnInput extends Controller {
  constructor(btnElement) {
    super()
    this.determineReadyState()
    window.addEventListener(
      'resize',
      () => {
        this.determineReadyState()
      },
      false
    )

    this._btnElement = btnElement
    this._btnPressed = false

    this._btnElement.addEventListener('touchstart', () => this.onPressed())
    this._btnElement.addEventListener('touchend', () => this.onReleased())
  }

  determineReadyState() {
    this._ready = window.matchMedia('(max-width: 1024px)').matches
  }

  ready() {
    return this._ready
  }

  onPressed() {
    this._btnPressed = true
  }

  onReleased() {
    this._btnPressed = false
  }

  btnPressed() {
    return this._btnPressed
  }
}

export default VirtualBtnInput
