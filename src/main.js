import { createApp } from "vue";
import "./styles.css";
import router from './router'
import App from "./App.vue";
import { appWindow } from '@tauri-apps/api/window'


createApp(App).use(router).mount("#app");