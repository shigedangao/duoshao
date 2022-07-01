<script setup>
import Sidebar from './components/Sidebar.vue'
import Topbar from './components/Topbar.vue'
import Scratchpad from './components/Scratchpad.vue'
import Generated from './components/Generated.vue'
import { useNote } from './store/note';

// Load the store
const noteStore = useNote()

// Import function
const { loadNotes } = noteStore

loadNotes()
</script>

<template>
  <div id="app">
    <Sidebar />
    <div class="main">
      <Topbar />
      <div class="core__container">
        <Scratchpad />
        <Generated />
      </div>
    </div>
  </div>
</template>

<style>
body {
  padding: 0px;
  margin: 0px;
  background-color: transparent;
  overflow: hidden;
}

p {
  padding: 0px;
  margin: 0px;
}

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  display: flex;
  width: 100%;
}

.main {
  flex-grow: 1;
}

.core__container {
  display: flex;
}

@media (prefers-color-scheme: dark) {
  body {
    background-color: #232529;
    color: white;
  }
}
</style>
