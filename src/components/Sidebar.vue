<script setup>
import { ref } from 'vue';
import Item from './bootstrap/sidebar/Item.vue'

// data
const position = ref(20);

function handleDragging(e) {
  const percentage = (e.pageX / window.innerWidth) * 100;

  if (percentage >= 10 && percentage <= 25) {
    position.value = percentage.toFixed(2)
  } else {
    stopDrag()
  }
}

function startDrag() {
  document.addEventListener('mousemove', handleDragging)
}

function stopDrag() {
  document.removeEventListener('mousemove', handleDragging)
}

</script>

<template>
  <div class="sidebar__container" :style="{width: `${position}%`}" @mouseup="stopDrag">
    <div class="sidebar__container-item">
      <Item title="foo" date="2022-06-01" label="chinese" />
    </div>
    <div
      class="sidebar__container-handle"
      @mousedown="startDrag"
      :style="{left: `${position}%`}"
    ></div>
  </div>
</template>

<style scoped>
.sidebar__container {
  height: 100vh;
  background-color: #FFFFFF;
  color: #454545;
  display: flex;
}

.sidebar__container-item {
  flex-grow: 1;
}

.sidebar__container-handle {
    position: absolute;
    width: 2px;
    height: 100vh;
  }

.sidebar__container-handle:hover {
  cursor: ew-resize;
}

@media (prefers-color-scheme: dark) {
  .sidebar__container {
    background-color: #232528;
    color: white;
  }

  .sidebar__container-handle {
    border-right: 1px solid #000000;
  }
}
</style>
