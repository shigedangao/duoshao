<script setup>
const props = defineProps({
  count: {
    type: Number,
    required: true
  },
  color: {
    type: String,
    required: true
  }
})
</script>

<template>
  <div class="counter__container" v-bind:style="`border-color: ${color}`">
    <p v-bind:style="`color: ${color}`">{{count}}</p>
  </div>
</template>

<style>
.counter__container {
  border: 2px solid transparent;
  border-radius: 100%;
  width: 25px;
  height: 25px;
  align-content: flex-end;
}
</style>
