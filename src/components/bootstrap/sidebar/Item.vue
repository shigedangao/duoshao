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
  },
  active: {
    type: Boolean,
    required: false,
    default: false
  },
  index: {
    type: Number,
    required: true,
    default: 0
  }
})
</script>

<template>
  <div
    class="sidebar__item__container"
    v-bind:class="{ active: active }"
    @click="clickCb(props.id)"
  >
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
  transition: background-color 100ms ease-in-out;
}

.sidebar__item__container:hover {
  cursor: pointer;
}

.sidebar__item__container.active {
  background-color: #79AADF;
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

  .sidebar__item__container.active {
    background-color: #2B4668 !important;
  }
}
</style>
