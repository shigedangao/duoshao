export default {
  /**
   * Set
   *
   * @param {String} key
   * @param {Object} object
   * @returns
   */
  set(key, object) {
    const data = JSON.stringify(object)
    localStorage.setItem(`duoshao::${key}`, data)
  },
  /**
   * Get
   *
   * @param {String} key
   * @returns
   */
  get(key) {
    const item = localStorage.getItem(`duoshao::${key}`)
    const deserialized = JSON.parse(item)

    return deserialized
  },
}
