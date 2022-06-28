<script setup>
import { useDebouncedRef } from "../../utils"
import { watch } from "vue"

const props = defineProps({
  callback: {
    type: Function,
    required: true
  }
})

const content = useDebouncedRef('')

watch(content, newContent => {
  props.callback(newContent)
})
</script>

<template>
  <div class="input__container">
    <textarea class="typing" v-model="content" />
  </div>
</template>

<style>
.typing {
  width: 100%;
  height: calc(100vh - 60px);
  overflow-y: scroll;
  border: 1px solid transparent;
  font-size: 1em;
  resize: none;
  background-color: transparent;
}

textarea:focus {
  outline: none;
}

@media (prefers-color-scheme: dark) {
  .typing {
    color: white;
  }
}
</style>
