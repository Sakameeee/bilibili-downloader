<script setup lang="ts">
import {ref} from "vue";
import {Bell, Film, Moon, Setting, Sunny, VideoCamera} from "@element-plus/icons-vue";
import {useDark, useToggle} from "@vueuse/core";
import {useRouter} from "vue-router";

const isCollapse = ref(false);
const isDark = useDark();
const router = useRouter();

const handleSelect = (key: string) => {
  if (key === "download") {
    router.replace("/download/downloading");
  } else {
    router.replace(`/${key}`);
  }
}
</script>

<template>
  <div id="side">
    <div class="logo" data-tauri-drag-region>
      <el-image style="width: 70px; height: 70px; margin-top: 20px" src="/bilibili-downloader.png" fit="cover" />
    </div>
    <el-divider/>
    <el-menu
        default-active="video"
        class="el-menu-vertical-demo"
        :collapse="isCollapse"
        @select="handleSelect"
        active-text-color="#ff77a4"
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
      <el-switch v-model="isDark" @click="useToggle" style="--el-switch-on-color: #ff77a4;">
        <template #active-action>
          <Moon/>
        </template>
        <template #inactive-action>
          <Sunny/>
        </template>
      </el-switch>
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