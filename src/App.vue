<script setup>
import SidebarContainer from './components/SidebarContainer.vue'
import TopbarContainer from './components/TopbarContainer.vue'
import ScratchpadPanel from './components/ScratchpadPanel.vue'
import GeneratedPanel from './components/GeneratedPanel.vue'
import { useNote } from './store/note'

// Load the store
const noteStore = useNote()

// Import function
const { loadNotes } = noteStore

loadNotes()
</script>

<template>
  <div id="app">
    <sidebar-container />
    <div class="main">
      <topbar-container />
      <div class="core__container">
        <scratchpad-panel />
        <generated-panel />
      </div>
    </div>
  </div>
</template>

<style lang="scss">
body {
  padding: 0px;
  margin: 0px;
  background-color: transparent;
  overflow: hidden;
  user-select: none;
  -webkit-user-select: none;
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

  .core__container {
    display: flex;
  }
}

@media (prefers-color-scheme: dark) {
  body {
    background-color: #232529;
    color: white;
  }
}
</style>
