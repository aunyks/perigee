/**
 * Gets a random integer within the range [0, max).
 *
 * @param {number} max
 *
 * @returns { integer }
 */
function randomIntFromZero(max = 0) {
  return parseInt(Math.random() * max)
}

export { randomIntFromZero }
