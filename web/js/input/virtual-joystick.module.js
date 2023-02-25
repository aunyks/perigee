import Controller from './controller.module.js'

const JOYSTICK_CENTER_X = 184
const JOYSTICK_CENTER_Y = 184
const MAX_OFFSET_RADIUS = 80
const SVG_VIEWBOX_SIZE = 384
const clamp = (num, min, max) => Math.min(Math.max(num, min), max)

class VirtualJoystickInput extends Controller {
  constructor(stickElement) {
    super()
    this.determineReadyState()

    this._stickElement = stickElement
    this._transformMatrix = null
    this._stickX = 0
    this._stickY = 0

    window.addEventListener(
      'resize',
      () => {
        this.determineReadyState()
      },
      false
    )

    this._stickElement.setAttribute('cx', '184')
    this._stickElement.setAttribute('cy', '184')

    this._stickElement.addEventListener('touchstart', (e) =>
      this.onTouchStart(e)
    )
    this._stickElement.addEventListener('touchmove', (e) => this.onTouchMove(e))
    this._stickElement.addEventListener('touchend', (e) => this.onTouchEnd(e))
  }

  determineReadyState() {
    this._ready = window.matchMedia('(max-width: 1024px)').matches
  }

  ready() {
    return this._ready
  }

  onTouchStart(event) {
    this._stickElement.classList.add('active')
  }

  onTouchEnd() {
    this._stickElement.classList.remove('active')
    this._stickElement.setAttribute('cx', '184')
    this._stickElement.setAttribute('cy', '184')
    this._stickX = 0
    this._stickY = 0
  }

  onTouchMove(event) {
    this._stickElement.classList.remove('active')
    this._transformMatrix = this._stickElement.getScreenCTM()
    let newXPos =
      (event.touches[0].clientX - this._transformMatrix.e) /
      this._transformMatrix.a
    let newYPos =
      (event.touches[0].clientY - this._transformMatrix.f) /
      this._transformMatrix.d
    let centeredX = clamp(
      (newXPos - JOYSTICK_CENTER_X) / MAX_OFFSET_RADIUS,
      -1,
      1
    )
    let centeredY = clamp(
      (newYPos - JOYSTICK_CENTER_Y) / MAX_OFFSET_RADIUS,
      -1,
      1
    )
    this._stickElement.setAttribute(
      'cx',
      `${clamp(
        newXPos,
        JOYSTICK_CENTER_X - MAX_OFFSET_RADIUS,
        JOYSTICK_CENTER_X + MAX_OFFSET_RADIUS
      )}`
    )
    this._stickElement.setAttribute(
      'cy',
      `${clamp(
        newYPos,
        JOYSTICK_CENTER_Y - MAX_OFFSET_RADIUS,
        JOYSTICK_CENTER_Y + MAX_OFFSET_RADIUS
      )}`
    )
    this._stickX = centeredX
    this._stickY = centeredY
  }

  getStickPos() {
    // x and y are both in the range [-1, 1]
    // x: negative is left. y: negative is up
    return {
      x: this._stickX,
      y: this._stickY,
    }
  }
}

export default VirtualJoystickInput
