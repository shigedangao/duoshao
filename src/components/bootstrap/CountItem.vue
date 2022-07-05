<script setup>
defineProps({
  count: {
    type: Number,
    required: true,
  },
  color: {
    type: String,
    required: true,
  },
})
</script>

<template>
  <div class="counter__wrapper">
    <p class="counter__wrapper-label">Founded</p>
    <div class="counter__container" v-bind:style="`border-color: ${color}`">
      <p v-bind:style="`color: ${color}`">{{ count }}</p>
    </div>
  </div>
</template>

<style>
.counter__wrapper {
  display: flex;
  height: 30px;
  align-items: center;
}

.counter__wrapper-label {
  padding: 0px 10px;
  font-size: 0.8em;
}

.counter__container {
  border: 2px solid transparent;
  border-radius: 100%;
  width: 25px;
  height: 25px;
  align-content: flex-end;
  display: flex;
  justify-content: center;
  align-items: center;
}

@media (prefers-color-scheme: dark) {
  .counter__wrapper-label {
    color: white;
  }
}
</style>
