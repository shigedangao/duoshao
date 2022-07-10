<script setup>
import TagItem from './TagItem.vue'

const props = defineProps({
  title: {
    type: String,
    required: true,
  },
  formattedDate: {
    type: String,
    required: true,
  },
  id: {
    type: String,
    required: true,
  },
  lang: {
    type: String,
    required: true,
  },
  clickCb: {
    type: Function,
    required: true,
  },
  active: {
    type: Boolean,
    required: false,
    default: false,
  },
  index: {
    type: Number,
    required: true,
    default: 0,
  },
})
</script>

<template>
  <div
    class="sidebar__item__container"
    v-bind:class="{ active: active }"
    @click="clickCb(props.id, props.lang)"
  >
    <p class="sidebar__item__container-title">{{ props.title }}</p>
    <p class="sidebar__item__container-date">{{ props.formattedDate }}</p>
    <tag-item :label="props.lang" />
  </div>
</template>

<style lang="scss">
.sidebar__item__container {
  margin: 10px;
  width: auto;
  background-color: #dcdcdc;
  border-radius: 8px;
  text-align: left;
  padding: 10px;
  transition: background-color 100ms ease-in-out;

  &:hover {
    cursor: pointer;
  }

  &-title {
    font-weight: bold;
  }

  &-date {
    font-size: 0.8em;
  }

  &.active {
    background-color: #79aadf;
  }
}

@media (prefers-color-scheme: dark) {
  .sidebar__item__container {
    background-color: #464646;
    color: white;

    &.active {
      background-color: #2b4668 !important;
    }
  }
}
</style>
