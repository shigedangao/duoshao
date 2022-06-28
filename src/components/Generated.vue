<script setup>
import { storeToRefs } from 'pinia';
import { useLanguage } from '../store';
import Definition from './bootstrap/Definition.vue';

// import the store
const store = useLanguage()

// get the selected content
const { generatedDefinitions } = storeToRefs(store)
console.log(generatedDefinitions)
</script>

<template>
  <div class="generated__container">
    <p class="label">Generated</p>
    <div class="generated__container">
      <div
        class="generated__container-item"
        v-for="def in generatedDefinitions"
      >
        <Definition :definition="def"/>
      </div>
    </div>
  </div>
</template>

<style>
.generated__container {
  flex-grow: 2;
  background-color: #F2F2F2;
  height: 100vh;
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
