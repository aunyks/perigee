import Controller from './controller.module.js'

class KeyboardInput extends Controller {
  constructor(elementToWatch) {
    super()
    this.determineReadyState()

    window.addEventListener(
      'resize',
      () => {
        this.determineReadyState()
      },
      false
    )

    this._pressedButtonsList = []
    this._elementToWatch = elementToWatch || document.body
    this._elementToWatch.addEventListener(
      'keydown',
      (e) => this.onKeyDown(e),
      false
    )
    this._elementToWatch.addEventListener(
      'keyup',
      (e) => this.onKeyUp(e),
      false
    )
  }

  determineReadyState() {
    this._ready = !window.matchMedia('(max-width: 1024px)').matches
  }

  onKeyDown(event) {
    const key = event.key
    if (!this._pressedButtonsList.includes(key)) {
      this._pressedButtonsList.push(key)
    }
  }

  onKeyUp(event) {
    const key = event.key
    if (this._pressedButtonsList.includes(key)) {
      for (let i = 0; i < this._pressedButtonsList.length; i++) {
        if (this._pressedButtonsList[i] === key) {
          this._pressedButtonsList.splice(i, 1)
        }
      }
    }
  }

  ready() {
    return this._ready
  }

  buttonPressed(key) {
    return this._pressedButtonsList.includes(key)
  }

  wPressed() {
    return this.buttonPressed('w') || this.buttonPressed('W')
  }

  aPressed() {
    return this.buttonPressed('a') || this.buttonPressed('A')
  }

  sPressed() {
    return this.buttonPressed('s') || this.buttonPressed('S')
  }

  dPressed() {
    return this.buttonPressed('d') || this.buttonPressed('D')
  }

  qPressed() {
    return this.buttonPressed('q') || this.buttonPressed('Q')
  }

  iPressed() {
    return this.buttonPressed('i') || this.buttonPressed('I')
  }

  ePressed() {
    return this.buttonPressed('e') || this.buttonPressed('E')
  }

  upPressed() {
    return this.buttonPressed('ArrowUp') || this.buttonPressed('Up')
  }

  leftPressed() {
    return this.buttonPressed('ArrowLeft') || this.buttonPressed('Left')
  }

  downPressed() {
    return this.buttonPressed('ArrowDown') || this.buttonPressed('Down')
  }

  rightPressed() {
    return this.buttonPressed('ArrowRight') || this.buttonPressed('Right')
  }

  spacebarPressed() {
    return this.buttonPressed(' ')
  }

  escapePressed() {
    return this.buttonPressed('Escape')
  }

  cPressed() {
    return this.buttonPressed('c') || this.buttonPressed('C')
  }
}

export default KeyboardInput
