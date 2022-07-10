<script setup>
import TextareaItem from './bootstrap/TextareaItem.vue'
import { useNote } from '../store/note'
import { onUpdated } from 'vue'
import { isEmpty } from 'ramda'
import { confirm } from '@tauri-apps/api/dialog'

// import the store
const noteStore = useNote()

const triggerTextAnalysis = (content) => {
  // Save the note with the new content
  noteStore.editNote({
    content,
    title: content.substring(0, 20),
  })
  // Generate the definitions by calling Rust
  noteStore.generateDefinitions().catch(() =>
    confirm('An error occurred while generating the definition', {
      type: 'error',
    })
  )
}

onUpdated(() => {
  if (!isEmpty(noteStore.getContent)) {
    noteStore.generateDefinitions().catch(() =>
      confirm('An error occurred while generating the definition', {
        type: 'error',
      })
    )
  } else {
    noteStore.setEmptyDefinitions()
  }
})
</script>

<template>
  <div class="scratchpad__container">
    <p class="label">Scratchpad</p>
    <textarea-item
      :callback="triggerTextAnalysis"
      :initial-content="noteStore.getContent"
    />
  </div>
</template>

<style lang="scss">
.scratchpad__container {
  width: 50%;
  padding: 0px 10px 5px 10px;
}

@media (prefers-color-scheme: dark) {
  .scratchpad__container {
    background-color: #1e1e1e;
  }
}
</style>
