<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
/**
 * Imports
*/
  import { RouterLink, RouterView } from 'vue-router'
  import Login from './views/Login.vue';
</script>

<template>
  <div>
    <div data-tauri-drag-region class="titlebar">
      <div class="titlebar-button" id="titlebar-minimize">
        <img
          src="https://api.iconify.design/mdi:window-minimize.svg"
          alt="minimize"
        />
      </div>

      <div class="titlebar-button" id="titlebar-maximize">
        <img
          src="https://api.iconify.design/mdi:window-maximize.svg"
          alt="maximize"
        />
      </div>

      <div class="titlebar-button" id="titlebar-close">
        <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
      </div>
    </div>
  </div>

  <RouterView />
</template>

<style scoped>
  html, body {
    margin: 0px;
    padding: 0px;
  }

.titlebar {
  height: 30px;
  background: #329ea3;
  user-select: none;
  display: flex;
  justify-content: flex-end;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
}
.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;
}
.titlebar-button:hover {
  background: #30bac1;
}
</style>

