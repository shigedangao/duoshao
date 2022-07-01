import { defineStore } from "pinia"
import { invoke } from '@tauri-apps/api'
import { useNote } from "./note"

export const useLanguage = defineStore('language', {
  state: () => {
    return {
      selectedLanguage: 'chinese',
    }
  },
  actions: {
    async setLanguage(lang) {
      const note = useNote()
      this.selectedLanguage = lang

      return invoke('set_language', { lang })
        .then(() => note.editNote({ label: lang }))
    },
  }
})
