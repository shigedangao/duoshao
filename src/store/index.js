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
      console.log(defs)
      this.generatedDefinitions = defs
    },
    async generateDefinitions() {
      const res = await invoke('generate_definitions', { content: this.content })
      // get the english translations splitted
      const requests = res.map(def => invoke('get_definition_vec', { def }))
      
      return Promise.all(requests)
        .then(translations => {
          translations.forEach((t, i) => {
            res[i].translations = t
          })

          this.setDefinition(res)

          return Promise.resolve()
        })
    }
  }
})
