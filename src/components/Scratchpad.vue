<script setup>
import { useLanguage } from "../store";
import Textarea from "./bootstrap/Textarea.vue"

// import the store
const store = useLanguage()

// get some store actions
const { setContent, generateDefinitions } = store

const triggerTextAnalysis = content => {
  setContent(content)
  generateDefinitions()
    .catch(err => console.log(err))
}
</script>

<template>
  <div class="scratchpad__container">
    <p class="label">Scratchpad</p>
    <Textarea :callback="triggerTextAnalysis" />
  </div>
</template>

<style>
.scratchpad__container {
  width: 50%;
  padding: 0px 10px 5px 10px;
}

@media (prefers-color-scheme: dark) {
  .scratchpad__container {
    background-color: #1E1E1E;
  }
}
</style>
