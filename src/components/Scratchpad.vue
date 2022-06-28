<script setup>
import Textarea from "./bootstrap/Textarea.vue"

const triggerTextAnalysis = content => {
  console.log(content)
}
</script>

<template>
  <div class="scratchpad__container">
    <Textarea :callback="triggerTextAnalysis" />
  </div>
</template>

<style>
.scratchpad__container {
  flex-grow: 1;
  padding: 0px 10px 5px 10px;
}
</style>
