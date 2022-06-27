import { createApp } from 'vue'
import { invoke } from '@tauri-apps/api'
import { appWindow } from '@tauri-apps/api/window'
import { library } from '@fortawesome/fontawesome-svg-core'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import {
  faPenToSquare,
  faTrashCan,
  faShareFromSquare,
  faHeart
} from '@fortawesome/free-regular-svg-icons'
import App from './App.vue'

// add icon to the library
library.add([
  faPenToSquare,
  faTrashCan,
  faShareFromSquare,
  faHeart
])

invoke('hello_tauri', {})
    .then(response => console.log(response))

createApp(App)
  .component('font-awesome-icon', FontAwesomeIcon)
  .mount('#app')
