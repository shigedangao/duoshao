import { customRef } from 'vue'

export function useDebouncedRef(value, delay = 300) {
  let timeout
  return customRef((track, trigger) => {
    return {
      get() {
        track()
        return value
      },
      set(newValue) {
        clearTimeout(timeout)
        timeout = setTimeout(() => {
          value = newValue
          trigger()
        }, delay)
      },
    }
  })
}

/**
 * Generate Light Color Hex
 *  generate light colors code from see link below
 *  @link https://helderesteves.com/generating-random-colors-js/
 *
 * @returns
 */
export function generateLightColorHex() {
  const hue = Math.floor(Math.random() * 360)

  return `hsl(${hue},70%,70%)`
}

/**
 * Generate an UUID
 *  @link https://stackoverflow.com/a/2117523/7489243
 *
 * @returns
 */
// prettier-ignore
export function uuidv4() {
  return ([1e7]+-1e3+-4e3+-8e3+-1e11).replace(/[018]/g, (c) =>
    (c ^ crypto.getRandomValues(new Uint8Array(1))[0] & 15 >> c / 4).toString(16)
  )
}
