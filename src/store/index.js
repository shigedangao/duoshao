import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api'
import { useNote } from './note'

export const useLanguage = defineStore('language', {
  state: () => {
    return {
      selectedLanguage: 'traditional_chinese',
    }
  },
  actions: {
    async setLanguage(lang) {
      const note = useNote()
      this.selectedLanguage = lang

      await invoke('set_language', { lang }).then(() =>
        note.editNote({ label: lang })
      )

      return Promise.resolve()
    },
  },
})
