<script setup>
import { ref } from 'vue'

const props = defineProps({
  label: {
    type: String,
    required: true
  }
})
</script>

<template>
  <div class="tag__container">
    <p class="tag__container-content">{{ props.label }}</p>
  </div>
</template>

<style>
.tag__container {
  border: 1px solid transparent;
  border-radius: 5px;
  background-color: #F3AB52;
  font-size: 0.8em;
  padding: 0px 15px 0px 15px;
  display: inline-block;
  width: auto;
}

.tag__container-content {
  color: white;
}

@media (prefers-color-scheme: dark) {
  .tag__container {
    background-color: #F36752;
    color: white;
  }
}
</style>
