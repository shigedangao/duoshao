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
      <div class="sidebar__container-icon">
        <font-awesome-icon icon="fa-regular fa-pen-to-square" id="pen-icon" />
      </div>
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

.sidebar__container-icon {
  display: block;
  margin-left: 10px;
  width: 24px;
  padding: 5px;
  border-radius: 8px;
  transition: background-color 100ms ease-in-out;
}

.sidebar__container-icon:hover {
  background-color: #EBEAE8;
}

#pen-icon {
  display: block;
  font-size: 1.5em;
  color: #B2B0B0;
}

.sidebar__container-item {
  flex-grow: 1;
}

.sidebar__container-handle {
    position: absolute;
    width: 1px;
    height: 100vh;
  }

.sidebar__container-handle:hover {
  cursor: ew-resize;
}

.sidebar__container-handle {
  background-color: #E6E6E6;
}

@media (prefers-color-scheme: dark) {
  .sidebar__container {
    background-color: #232528;
    color: white;
  }

  .sidebar__container-handle {
    background-color: #000000;
  }

  .sidebar__container-icon:hover {
    background-color: #393B3F;
  }

}
</style>
