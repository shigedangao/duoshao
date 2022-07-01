<script setup>
import { useDebouncedRef } from "../../utils"
import { ref, watch } from "vue"

const props = defineProps({
  callback: {
    type: Function,
    required: true
  },
  initialContent: {
    type: String,
    required: false,
    default: ''
  }
})

let debounce = null
const callback = event => {
  clearTimeout(debounce)
  debounce = setTimeout(() => {
    props.callback(event.target.value)
  }, 300)
}

</script>

<template>
  <div class="input__container">
    <textarea
      class="typing"
      :value="initialContent"
      placeholder="Enter your text"
      @input="callback"
    />
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
