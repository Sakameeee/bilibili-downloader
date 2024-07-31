<script setup lang="ts">

import {Close, FullScreen, Minus} from "@element-plus/icons-vue";
import {onMounted} from "vue";
import { Window } from '@tauri-apps/api/window';

const appWindow = Window.getByLabel('main');
onMounted(() => {
  document.getElementById('titlebar-close')!.addEventListener('click', () => {
    appWindow.close();
    console.log("aaaaaaaa");
  })
  document.getElementById('titlebar-minimize')!.addEventListener('click', () => appWindow.minimize())
  document.getElementById('titlebar-maximize')!.addEventListener('click', () => appWindow.toggleMaximize())
})
</script>

<template>
  <div id="head-bar">
    <el-avatar :size="40" src="https://empty">
      <img
          src="https://cube.elemecdn.com/e/fd/0fc7d20532fdaf769a25683617711png.png"
      />
    </el-avatar>
    <el-divider direction="vertical" border-style="none" />
    <span>xxxxx</span>
    <el-divider direction="vertical" />
    <el-icon id="titlebar-minimize">
      <Minus/>
    </el-icon>
    <el-divider direction="vertical" border-style="none" />
    <el-icon id="titlebar-maximize">
      <FullScreen/>
    </el-icon>
    <el-divider direction="vertical" border-style="none" />
    <el-icon id="titlebar-close">
      <Close/>
    </el-icon>
  </div>
</template>

<style scoped>
#head-bar {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  float: right;
}
</style>