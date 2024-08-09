<script setup lang="ts">
import {ref} from "vue";
import {Bell, Close, Film, Location, Setting, VideoCamera} from "@element-plus/icons-vue";
import {useDark, useToggle} from "@vueuse/core";
import {useRouter} from "vue-router";

const isCollapse = ref(false);
const isDark = useDark();
const router = useRouter();

const handleSelect = (key: string, keyPath: string[]) => {
  router.replace(`/${key}`);
}
</script>

<template>
  <div id="side">
    <div class="logo" data-tauri-drag-region>
      <el-image style="width: 70px; height: 70px; margin-top: 20px" src="https://fuss10.elemecdn.com/e/5d/4a731a90594a4af544c0c25941171jpeg.jpeg" fit="cover" />
    </div>
    <el-divider/>
    <el-menu
        default-active="video"
        class="el-menu-vertical-demo"
        :collapse="isCollapse"
        @select="handleSelect"
    >
      <el-menu-item index="video">
        <template #title>
          <el-icon><VideoCamera /></el-icon>
          <span>下载视频/音频</span>
        </template>
      </el-menu-item>
      <el-menu-item index="anime">
        <el-icon><Film /></el-icon>
        <template #title>下载番剧</template>
      </el-menu-item>
      <el-menu-item index="download">
        <el-icon><Bell /></el-icon>
        <template #title>下载详情</template>
      </el-menu-item>
      <el-menu-item index="config">
        <el-icon><setting /></el-icon>
        <template #title>设置</template>
      </el-menu-item>
    </el-menu>

    <div style="position: absolute; bottom: 20px; width: 100%; text-align: center; align-content: center">
      <el-switch v-model="isDark" @click="useToggle"/>
    </div>
  </div>
</template>

<style scoped>
#side {
  height: 100%;
  width: 100%;
  position: relative;
  background-color: var(--el-color-primary-light-9);
}

.logo {
  text-align: center;
  align-content: center;
}
</style>