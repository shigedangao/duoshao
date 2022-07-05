<script setup>
import { generateLightColorHex } from '../../utils'
import CountItem from './CountItem.vue'

defineProps({
  definition: {
    type: Object,
    required: true,
  },
})

const color = generateLightColorHex()
</script>

<template>
  <div class="definition-item">
    <div class="bullet" v-bind:style="`background-color: ${color}`"></div>
    <div class="content">
      <p class="definition-content">{{ definition.writing_method }}</p>
      <p class="definition-content">
        Pronounciation: {{ definition.pronunciation }}
      </p>
      <p class="definition-content">Translation:</p>
      <div class="translation__content">
        <p
          class="translation__content-item"
          v-for="(t, idx) in definition.translations"
          v-bind:key="idx"
        >
          - {{ t }}
        </p>
      </div>
    </div>
    <count-item :color="color" :count="definition.count" />
  </div>
</template>

<style lang="scss" scoped>
.definition-item {
  display: flex;
}

.content {
  padding-left: 10px;
  flex-grow: 1;
}

.definition-content {
  color: black;
  text-align: left;
}

.translation__content {
  padding-left: 20px;
}

.translation__content-item {
  text-align: left;
}

.bullet {
  width: 18px;
  height: 18px;
  border-radius: 5px;
  margin-top: 3px;
}

@media (prefers-color-scheme: dark) {
  .definition-content {
    color: white;
  }

  .translation__content {
    color: white;
  }
}
</style>
