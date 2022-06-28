<script setup>
import { ref } from 'vue';
import { generateLightColorHex } from '../../utils';
import Count from './Count.vue';

const props = defineProps({
  definition: {
    type: Object,
    required: true
  }
})

const color = generateLightColorHex()
</script>

<template>
  <div class="definition-item">
    <div class="bullet" v-bind:style="`background-color: ${color}`"></div>
    <p class="definition-content">{{definition.writing_method}} - {{definition.pronounciation}} - {{definition.english}}</p>
    <Count :color="color" :count="definition.count" />
  </div>
</template>

<style scoped>
.definition-item {
  display: flex
}

.definition-content {
  color: black;
}

.bullet {
  width: 18px;
  height: 18px;
  border-radius: 5px;
}

@media (prefers-color-scheme: dark) {
  .definition-content {
    color: white;
  }
}
</style>
