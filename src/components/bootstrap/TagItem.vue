<script setup>
const props = defineProps({
  label: {
    type: String,
    required: true,
  },
})
</script>

<template>
  <div class="tag__container">
    <p class="tag__container-content">{{ props.label }}</p>
  </div>
</template>

<style lang="scss">
.tag__container {
  border: 1px solid transparent;
  border-radius: 5px;
  background-color: #f3ab52;
  font-size: 0.8em;
  padding: 0px 15px 0px 15px;
  display: inline-block;
  width: auto;

  &-content {
    color: white;
  }
}

@media (prefers-color-scheme: dark) {
  .tag__container {
    background-color: #f36752;
    color: white;
  }
}
</style>
