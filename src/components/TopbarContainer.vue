<script setup>
import { useLanguage } from '../store'
import { TrashIcon, UploadIcon } from '@heroicons/vue/outline'
import SelectItem from './bootstrap/SelectItem.vue'
import { useNote } from '../store/note'
import { save } from '@tauri-apps/api/dialog'

// import the store
const store = useLanguage()
const noteStore = useNote()

const { setLanguage } = store

/**
 * Selected
 *  Set the selected language for the app
 *
 * @param {String} value
 */
const selected = (value) => {
  setLanguage(value).catch(() =>
    confirm('Unable to set the targeted language', { type: 'error' })
  )
}

const saveFile = () => {
  save({
    filters: [
      {
        title: 'Save definitions',
        name: 'export',
        extensions: ['csv'],
      },
    ],
  })
    .then((path) => noteStore.exportToCSV(path))
    .catch(() =>
      confirm('Error while exporting the definitions to CSV', { type: 'error' })
    )
}
</script>

<template>
  <div class="topbar__container">
    <select-item :selected="selected" />
    <div class="topbar__container-icon">
      <TrashIcon class="topbar-icon" @click="noteStore.deleteNote" />
      <UploadIcon class="topbar-icon" @click="saveFile" />
    </div>
  </div>
</template>

<style lang="scss">
.topbar__container {
  width: 100%;
  height: 50px;
  display: flex;
  align-items: center;
  justify-content: end;

  &-icon {
    display: flex;
  }
}

.topbar-icon {
  color: #b2b0b1;
  margin-right: 10px;
  transition: background-color 100ms ease-in-out;
  border-radius: 8px;
  width: 24px;
  height: 24px;

  &:hover {
    background-color: #ebeae8;
    cursor: pointer;
  }
}

@media (prefers-color-scheme: dark) {
  .topbar__container {
    background-color: #2c2e32;
    color: white;
  }

  .topbar-icon:hover {
    background-color: #393b3f;
  }
}
</style>
