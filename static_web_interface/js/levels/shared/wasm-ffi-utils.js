// Provide utility functions as a super class
// to make it easier to "import" them.
//
// Having a simulation extend this class instead of
// importing each function individually takes less mind share imo.
class WasmUtils {
  constructor() {
    // Maintain a map between strings and their pointers in WASM memory
    // so that we're not creating the same string multiple times, saving
    // a few bytes of WASM memory here and there.
    this._stringMap = Object.create(null)
  }

  _getUint8LE(memoryTypedArray, memoryAddress, startIndex) {
    if (memoryTypedArray.length - startIndex < 1) {
      throw new Error(
        `Cannot get Uint8LE: index ${startIndex} is out of bounds`
      )
    }
    const dataView = new DataView(memoryTypedArray, memoryAddress)
    return dataView.getUint8(startIndex)
  }

  _getUint32LE(memoryTypedArray, memoryAddress, startIndex) {
    if (memoryTypedArray.length - startIndex < 4) {
      throw new Error(
        `Cannot get Uint32LE: index ${startIndex} is out of bounds`
      )
    }
    const dataView = new DataView(memoryTypedArray, memoryAddress)
    return dataView.getUint32(startIndex, true)
  }

  getFloat32LE(memoryTypedArray, memoryAddress, startIndex) {
    if (memoryTypedArray.length - startIndex < 4) {
      throw new Error(
        `Cannot get Float32LE: index ${startIndex} is out of bounds`
      )
    }
    const dataView = new DataView(memoryTypedArray, memoryAddress)
    return dataView.getFloat32(startIndex, true)
  }

  // This is static because it's used in the WASM imports
  // which are created before a Sim is constructed
  static getCString(memoryTypedArray, memoryAddress) {
    const view = new Uint8Array(memoryTypedArray.buffer)

    let terminalByteAddress = memoryAddress
    while (view[terminalByteAddress]) {
      terminalByteAddress++
    }

    const strBytes = new Uint8Array(
      view.subarray(memoryAddress, terminalByteAddress)
    )
    return new TextDecoder().decode(strBytes)
  }

  // static logHandlers(wasmMemory) {
  //   return {
  //     on_error: (stringPtr) => {
  //       const str = this.getCString(wasmMemory, stringPtr)
  //       console.error(str)
  //     },
  //     on_warn: (stringPtr) => {
  //       const str = this.getCString(wasmMemory, stringPtr)
  //       console.warn(str)
  //     },
  //     on_debug: (stringPtr) => {
  //       const str = this.getCString(wasmMemory, stringPtr)
  //       console.debug(str)
  //     },
  //     on_info: (stringPtr) => {
  //       const str = this.getCString(wasmMemory, stringPtr)
  //       console.log(str)
  //     },
  //     on_trace: (stringPtr) => {
  //       const str = this.getCString(wasmMemory, stringPtr)
  //       console.log(str)
  //     },
  //   }
  // }

  ptrToString(str, allocFn, wasmMemory) {
    const strPtr = this._stringMap[str]
    if (strPtr !== undefined) {
      return strPtr
    } else {
      const strBytes = new TextEncoder().encode(str)
      // Copy the string into memory allocated in the WebAssembly
      const newStrPtr = allocFn(strBytes.byteLength)
      const byteBuffer = new Uint8Array(
        wasmMemory.buffer,
        newStrPtr,
        strBytes.byteLength
      )
      // Write the string into newly allocated space
      byteBuffer.set(strBytes)
      // Cache the string's pointer in JS memory
      this._stringMap[str] = newStrPtr
      return newStrPtr
    }
  }

  getVector3F32(wasmMemory, vectorPtr) {
    return [
      this.getFloat32LE(wasmMemory.buffer, vectorPtr, 0),
      this.getFloat32LE(wasmMemory.buffer, vectorPtr, 4),
      this.getFloat32LE(wasmMemory.buffer, vectorPtr, 8),
    ]
  }

  getQuaternionF32(wasmMemory, quatPtr) {
    return [
      this.getFloat32LE(wasmMemory.buffer, quatPtr, 0),
      this.getFloat32LE(wasmMemory.buffer, quatPtr, 4),
      this.getFloat32LE(wasmMemory.buffer, quatPtr, 8),
      this.getFloat32LE(wasmMemory.buffer, quatPtr, 12),
    ]
  }

  getIsometryF32(wasmMemory, isoPtr) {
    return [
      this.getQuaternionF32(wasmMemory, isoPtr),
      this.getVector3F32(wasmMemory, isoPtr + 16),
    ]
  }

  getTransformF32(wasmMemory, transPtr) {
    const isometry = this.getIsometryF32(wasmMemory, transPtr)
    return [
      isometry[0],
      isometry[1],
      this.getVector3F32(wasmMemory, transPtr + 28),
    ]
  }
}

export { WasmUtils }
