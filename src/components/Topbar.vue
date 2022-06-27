<script setup>
import Select from './bootstrap/Select.vue'

const selected = (value) => {
  console.log(value)
}
</script>

<template>
  <div class="topbar__container">
    <Select :selected="selected" />
    <div class="topbar__container-icon">
      <font-awesome-icon class="topbar-icon" icon="fa-regular fa-trash-can" />
      <font-awesome-icon class="topbar-icon" icon="fa-regular fa-share-from-square" />
    </div>
  </div>
</template>

<style>
.topbar__container {
  width: 100%;
  height: 50px;
  display: flex;
  align-items: center;
  justify-content: end;
}

.topbar-icon {
  color: #B2B0B1;
  font-size: 1.5em;
  padding: 0px 5px;
  margin-right: 10px;
  transition: background-color 100ms ease-in-out;
  padding: 5px;
  border-radius: 8px;
}

.topbar-icon:hover {
  background-color: #EBEAE8;
}

@media (prefers-color-scheme: dark) {
  .topbar__container {
    background-color: #2C2E32;
    color: white;
  }

  .topbar-icon:hover {
    background-color: #393B3F;
  }
}
</style>
