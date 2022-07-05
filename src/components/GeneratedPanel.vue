<script setup>
import { storeToRefs } from 'pinia'
import { useNote } from '../store/note'
import DefinitionItem from './bootstrap/DefinitionItem.vue'

// import the store
const store = useNote()

// get the selected content
const { generatedDefinitions } = storeToRefs(store)
</script>

<template>
  <div class="generated__container">
    <p class="label">Generated</p>
    <div class="container">
      <div
        class="generated__container-item"
        v-for="(def, idx) in generatedDefinitions"
        v-bind:key="idx"
      >
        <definition-item :definition="def" />
      </div>
    </div>
  </div>
</template>

<style lang="scss">
.generated__container {
  width: 50%;
  background-color: #f2f2f2;
  height: 100vh;
}

.container {
  padding: 0px 20px;
  overflow-y: auto;
  height: 90vh;
}

.label {
  font-size: 0.8em;
  padding-top: 10px;
}

@media (prefers-color-scheme: dark) {
  .generated__container {
    background-color: #222528;
  }

  .label {
    color: #808181;
  }
}
</style>
