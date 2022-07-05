<script setup>
import { ref } from 'vue'
import { PencilAltIcon } from '@heroicons/vue/outline'
import SidebarItem from './bootstrap/SidebarItem.vue'
import { useNote } from '../store/note'
import { storeToRefs } from 'pinia'

// Load the store
const noteStore = useNote()
// Create a ref
const position = ref(20)
const selectedIndex = ref(0)
// Link the store
const { notes } = storeToRefs(noteStore)
const { createNewNote, setWorkingNoteID } = noteStore

/**
 * Handle Dragging
 *    Disabled as there is a side effect which highlight other element
 * @param {Event} e
 */
//const handleDragging = (e) => {
//  const percentage = (e.pageX / window.innerWidth) * 100
//
//  if (percentage >= 15 && percentage <= 25) {
//    position.value = percentage.toFixed(2)
//  } else {
//    stopDrag()
//  }
//}

//const startDrag = () => {
//  document.addEventListener('mousemove', handleDragging)
//}

//const stopDrag = () => {
//  document.removeEventListener('mousemove', handleDragging)
//}

function itemCallback(id) {
  selectedIndex.value = this.index
  setWorkingNoteID(id)
}
</script>

<template>
  <div
    class="sidebar__container"
    :style="{ width: `${position}%` }"
    @mouseup="stopDrag"
  >
    <div class="sidebar__container-item">
      <div class="sidebar__container-icon">
        <PencilAltIcon class="pen-icon" @click="createNewNote" />
      </div>
      <div class="sidebar__container-list">
        <sidebar-item
          v-for="(n, idx) in notes"
          v-bind:key="idx"
          :id="n.id"
          :title="n.title"
          :formattedDate="n.formattedDate"
          :label="n.label"
          :click-cb="itemCallback"
          :active="selectedIndex === idx"
          :index="idx"
        />
      </div>
    </div>
    <div class="sidebar__container-handle" />
  </div>
</template>

<style scoped>
.sidebar__container {
  height: 100vh;
  background-color: #ffffff;
  color: #454545;
  display: flex;
  width: 25%;
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
  background-color: #ebeae8;
}

.pen-icon {
  display: block;
  color: #b2b0b0;
  width: 24px;
  height: 24px;
}

.sidebar__container-item {
  flex-grow: 1;
}

.sidebar__container-handle {
  width: 1px;
  height: 100vh;
}

.sidebar__container-handle {
  background-color: #e6e6e6;
}

.sidebar__container-list {
  height: 100%;
  overflow-y: auto;
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
    background-color: #393b3f;
  }
}
</style>
