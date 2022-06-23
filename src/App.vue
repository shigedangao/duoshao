<script setup>
import Sidebar from './components/Sidebar.vue'
</script>

<template>
  <div id="app">
    <Sidebar />
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
}

@media (prefers-color-scheme: dark) {
  body {
    background-color: #232529;
    color: white;
  }
}
</style>
