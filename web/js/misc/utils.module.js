/**
 * Returns a random integer within the range [0, max).
 *
 * @param {number} max
 *
 * @returns { integer }
 */
function randomIntFromZero(max = 0) {
  return parseInt(Math.random() * max)
}

/**
 * Receives an element with an aria-live attribute and returns a function
 * for creating announcements from that element.
 *
 * @param {announcerElem} HTMLElement
 *
 * @returns { Function }
 */
function bindAssistiveDeviceAnnouncer(announcerElem) {
  return (announcement) => {
    announcerElem.innerText = announcement
  }
}

export { randomIntFromZero, bindAssistiveDeviceAnnouncer }
