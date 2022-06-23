<script setup>
import Tag from '../Tag.vue'

const props = defineProps({
  title: {
    type: String,
    required: true
  },
  date: {
    type: String,
    required: true,
  },
  label: {
    type: String,
    required: true
  }
})
</script>

<template>
  <div class="sidebar__item__container">
    <p class="sidebar__item__container-title">{{ props.title }}</p>
    <p class="sidebar__item__container-date">{{ props.date }}</p>
    <Tag :label="props.label" />
  </div>
</template>

<style>
.sidebar__item__container {
  margin: 10px;
  width: auto;
  background-color: #DCDCDC;
  border-radius: 8px;
  text-align: left;
  padding: 10px;
  border-bottom: 1px solid #464646;
}

.sidebar__item__container-title {
  font-weight: bold;
}

.sidebar__item__container-date {
  font-size: 0.8em;
}

@media (prefers-color-scheme: dark) {
  .sidebar__item__container {
    background-color: #464646;
    color: white;
  }
}
</style>
