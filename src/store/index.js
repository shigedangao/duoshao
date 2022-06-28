import { defineStore } from "pinia"
import { invoke } from '@tauri-apps/api';

export const useLanguage = defineStore('language', {
  state: () => {
    return {
      selectedLanguage: 'chinese',
      content: '',
      generatedDefinitions: []
    }
  },
  actions: {
    async setLanguage(lang) {
      this.selectedLanguage = lang

      return invoke('set_language', { lang })
    },
    setContent(content) {
      this.content = content
    },
    setDefinition(defs) {
      this.generatedDefinitions = defs
    },
    async generateDefinitions() {
      const res = await invoke('generate_definitions', { content: this.content })
      this.setDefinition(res)

      return Promise.resolve()
    }
  }
})
