<script setup>
import Tag from '../Tag.vue'

const props = defineProps({
  title: {
    type: String,
    required: true
  },
  formattedDate: {
    type: String,
    required: true,
  },
  id: {
    type: String,
    required: true
  },
  label: {
    type: String,
    required: true
  },
  clickCb: {
    type: Function,
    required: true
  }
})
</script>

<template>
  <div class="sidebar__item__container" @click="clickCb(props.id)">
    <p class="sidebar__item__container-title">{{ props.title }}</p>
    <p class="sidebar__item__container-date">{{ props.formattedDate }}</p>
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
