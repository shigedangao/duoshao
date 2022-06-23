import { createApp } from 'vue'
import { invoke } from '@tauri-apps/api'
import { appWindow } from '@tauri-apps/api/window'
import App from './App.vue'

invoke('hello_tauri', {})
    .then(response => console.log(response))

createApp(App).mount('#app')
