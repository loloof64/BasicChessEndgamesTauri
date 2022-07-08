<script setup>
import { onMounted } from "vue";
import { invoke } from "@tauri-apps/api";
import { useStore } from "@/stores/options";

const optionsStore = useStore();

async function loadSettingsFromFile() {
  try {
    const settingsStr = await invoke("get_settings");
    const settings = JSON.parse(settingsStr);

    optionsStore.setSelectedEnginePath(settings.engine_path);
  } catch (err) {
    console.warn(err);
  }
}

onMounted(loadSettingsFromFile);

defineExpose({});
</script>

<template>
  <router-view></router-view>
</template>

<style>
html, body {
  height: calc(100% - 2px);
}

body {
  margin: 0;
}

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  background-color: black;
  display: flex;
  flex-direction: column;
  justify-content: space-evenly;
  height: 100%;
}
</style>
