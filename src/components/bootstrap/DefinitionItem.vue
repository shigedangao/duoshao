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
  <div class="definition-wrapper">
    <div class="definition-item">
      <div class="bullet" v-bind:style="`background-color: ${color}`" />
      <div class="content">
        <p class="definition-content title">
          {{ definition.writing_method }}
        </p>
        <p class="definition-content title">Pronunciations</p>
        <div class="list__content">
          <p
            class="list__content-item"
            v-for="(t, idx) in definition.pronunciations"
            v-bind:key="idx"
          >
            - {{ t }}
          </p>
        </div>
        <p class="definition-content title">Translation:</p>
        <div class="list__content">
          <p
            class="list__content-item"
            v-for="(t, idx) in definition.translations"
            v-bind:key="idx"
          >
            - {{ t }}
          </p>
        </div>
      </div>
      <count-item :color="color" :count="definition.count" />
    </div>
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

  &.title {
    font-weight: 500;
  }
}

.list__content {
  padding-left: 20px;

  &-item {
    text-align: left;
  }
}

.bullet {
  width: 18px;
  height: 18px;
  background-color: blue;
  border-radius: 5px;
  border: 1px solid transparent;
}

.divider {
  width: 100%;
  height: 1px;
  margin: 10px;
}

@media (prefers-color-scheme: dark) {
  .definition-content {
    color: white;
  }

  .translation__content {
    color: white;
  }

  .list__content-item {
    color: white;
  }
}
</style>
