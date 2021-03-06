import { defineStore } from 'pinia'
import storage from '../storage'
import { isEmpty, isNil } from 'ramda'
import { DateTime } from 'luxon'
import { uuidv4 } from '../utils'
import { useLanguage } from '.'
import { invoke } from '@tauri-apps/api'
import { message } from '@tauri-apps/api/dialog'

// constant
const NOTE_STORE_KEY = 'note'
const DEFAULT_NOTE_LABEL = 'traditional_chinese'

export const useNote = defineStore('note', {
  getters: {
    /**
     * Get Content
     *
     * @param {Object} state
     * @returns
     */
    getContent: (state) => {
      const items = state.notes.filter((n) => n.id == state.currentNoteID)
      if (isEmpty(items) || isNil(items)) {
        return ''
      }

      const item = items.pop()
      return item.content
    },
  },
  /**
   * Store a list of notes. A note is define by the following object
   * {
   *  "title": "foo",
   *  "date": "2022-06-01",
   *  "label": "chinese",
   *  "text": "",
   * }
   *
   * @returns
   */
  state: () => {
    return {
      notes: [],
      currentNoteID: '',
      generatedDefinitions: [],
    }
  },
  actions: {
    /**
     * Create a new note
     *
     * @returns
     */
    async createNewNote() {
      const note = {
        id: uuidv4(),
        title: 'New scratchpad',
        date: DateTime.now(),
        formattedDate: DateTime.now().toLocaleString(),
        label: DEFAULT_NOTE_LABEL,
        content: '',
        generated: [],
      }

      const copy = this.notes.slice()
      copy.unshift(note)

      this.notes = copy.slice()
      this.setWorkingNoteID(note.id)
      this.generatedDefinitions = []

      storage.set(NOTE_STORE_KEY, this.notes)
    },
    /**
     * Set Working ID
     *
     * @param {String} id
     */
    setWorkingNoteID(id) {
      this.currentNoteID = id
    },
    /**
     * Edit Note
     *
     * @param {Object} replace
     */
    async editNote(replace) {
      const notes = this.notes.map((n) => {
        if (n.id === this.currentNoteID) {
          n = {
            ...n,
            ...replace,
          }
        }

        return n
      })

      this.notes = notes
      storage.set(NOTE_STORE_KEY, this.notes)

      return Promise.resolve()
    },
    /**
     * Delete Note
     *
     * @param {String} id
     */
    async deleteNote() {
      if (this.notes.length === 1) {
        await message(`You can't delete the last item of the note`, {
          title: 'Warning',
          type: 'warning',
        })
        return
      }

      const itemIdx = this.notes.findIndex((n) => n.id !== this.currentNoteID)
      const filtered = this.notes.filter((n) => n.id !== this.currentNoteID)

      this.notes = filtered
      storage.set(NOTE_STORE_KEY, this.notes)

      if (!isNil(filtered[itemIdx])) {
        this.currentNoteID = filtered[itemIdx].id
      } else {
        this.currentNoteID = filtered[filtered.length - 1].id
      }

      return Promise.resolve()
    },
    setEmptyDefinitions() {
      this.generatedDefinitions = []
    },
    /**
     * Generate Definitions
     *
     * @returns
     */
    async generateDefinitions() {
      const res = await invoke('generate_definitions', {
        content: this.getContent,
      }).catch((err) => {
        if (
          err === 'No definition has been founded for the targeted language'
        ) {
          return Promise.resolve([])
        }

        return Promise.reject(err)
      })

      // somehow tauri in js can't find property which end by an 's'
      const items = res.map((item) => {
        item.translations = item.translation.split(',')
        item.pronunciations = item.pronounciation.split(',')

        return item
      })

      this.generatedDefinitions = items
    },
    /**
     * Load Notes
     *  Load the notes in at the startup
     */
    loadNotes() {
      const langStore = useLanguage()
      const notes = storage.get(NOTE_STORE_KEY)
      if (isEmpty(notes) || isNil(notes)) {
        // create a new note
        this.createNewNote()
        return Promise.resolve()
      }

      this.notes = notes
      this.setWorkingNoteID(notes[0].id)
      langStore.setLanguage(notes[0].label)

      return Promise.resolve()
    },
    /**
     * Export to CSV the definitions
     *
     * @param {String} path
     * @returns
     */
    async exportToCSV(path) {
      await invoke('export_definition_to_csv', {
        defs: this.generatedDefinitions,
        path,
      })

      return Promise.resolve()
    },
  },
})
