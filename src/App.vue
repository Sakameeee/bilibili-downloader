<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import {onMounted} from 'vue';
import {useConfigStore} from "./store/config.ts";
import GlobalHeader from "./components/GlobalHeader.vue";
import GlobalSider from "./components/GlobalSider.vue";

const configStore = useConfigStore();

onMounted( async () => {
  await configStore.loadConfig();
})
</script>

<template>
  <div class="container">
    <el-container style="height: 100%">
      <el-aside width="150px" data-tauri-drag-region>
        <global-sider data-tauri-drag-region style="background-color: var(--el-color-primary-light-8)"/>
      </el-aside>
      <el-container>
        <el-header data-tauri-drag-region style="padding-top: 0; background-color: var(--el-color-primary-light-9)">
          <global-header data-tauri-drag-region/>
        </el-header>
        <el-main>
          <router-view style="height: 100%"/>
        </el-main>
      </el-container>
    </el-container>
  </div>
</template>

<style scoped>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100vh;
  overflow: hidden;
}

el-header {
  position: relative;
  background-color: var(--el-color-primary-light-7);
  color: var(--el-text-color-primary);
  border-bottom-color: white;
  border-bottom: 2px;
}

el-aside {
  color: var(--el-text-color-primary);
  background-color: var(--el-color-primary-light-8);
  border-right-color: white !important;
  border-right: 2px !important;
}

el-main {
  color: var(--el-text-color-primary);
  background-color: var(--el-color-primary-light-5);
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }
}
</style>
