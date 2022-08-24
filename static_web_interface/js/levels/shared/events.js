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

export { GameEvents }
