import Controller from './controller.module.js'

class TouchInput extends Controller {
  constructor(domElement) {
    super()
    this.determineReadyState()
    this._domElement = domElement || document.body
    this._x = 0
    this._y = 0
    this._dx = 0
    this._dy = 0

    window.addEventListener(
      'resize',
      () => {
        this.determineReadyState()
        this.connect()
      },
      false
    )

    this.connect()
  }

  determineReadyState() {
    this._ready = window.matchMedia('(max-width: 1024px)').matches
  }

  ready() {
    return this._ready
  }

  getDx() {
    const dx = this._dx
    this._dx = 0
    return dx
  }

  getDy() {
    const dy = this._dy
    this._dy = 0
    return dy
  }

  connect() {
    if (this._ready) {
      this._domElement.addEventListener(
        'touchmove',
        (e) => this.onTouchMove(e),
        false
      )
      this._domElement.addEventListener(
        'touchstart',
        (e) => this.onTouchStart(e),
        false
      )
      this._domElement.addEventListener(
        'touchend',
        (e) => this.onTouchEnd(e),
        false
      )
      this._domElement.addEventListener(
        'touchcancel',
        (e) => this.onTouchEnd(e),
        false
      )
    }
  }

  onTouchMove(event) {
    this._dx = event.changedTouches[0].screenX - this._x
    this._dy = event.changedTouches[0].screenY - this._y
    this._x = event.changedTouches[0].screenX
    this._y = event.changedTouches[0].screenY
  }

  onTouchStart(event) {
    this._x = event.changedTouches[0].screenX
    this._y = event.changedTouches[0].screenY
  }

  onTouchEnd(event) {
    this._x = 0
    this._y = 0
    this._dx = 0
    this._dy = 0
  }
}

export default TouchInput
