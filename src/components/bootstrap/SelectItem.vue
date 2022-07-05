<script setup>
import { ref } from 'vue'
import { TranslateIcon } from '@heroicons/vue/outline'

// state
const selected = ref('chinese')

// props
const props = defineProps({
  selected: {
    type: Function,
    required: true,
  },
})

const onSelected = () => {
  props.selected(selected.value)
}
</script>

<template>
  <div class="searchbar__container">
    <TranslateIcon class="translate-icon" />
    <select name="language" v-model="selected" @change="onSelected">
      <option value="chinese">Chinese</option>
      <option value="laotian">Laotian</option>
    </select>
  </div>
</template>

<style scoped>
.searchbar__container {
  border: 1px solid #b3b0b0;
  border-radius: 8px;
  padding: 5px 10px;
  margin-right: 15px;
  display: flex;
  width: 180px;
  align-items: center;
}

.translate-icon {
  width: 15px;
  height: 15px;
  color: #b2b0b1;
}

select {
  margin-left: 5px;
  background-color: transparent;
  appearance: none;
  border: 0px;
  padding: 0px 10px;
  font-size: 0.8em;
  width: 100%;
  color: #b2b0b1;
}

.focus {
  border: 1px solid #a91e85 !important;
}

input.middle:focus {
  outline-width: 0;
}

input:focus,
select:focus,
textarea:focus,
button:focus {
  outline: none;
}

@media (prefers-color-scheme: dark) {
  .searchbar__container {
    border: 1px solid #5f6164;
  }

  select {
    color: #b2b0b1;
  }
}
</style>
