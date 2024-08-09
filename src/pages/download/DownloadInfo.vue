<script setup lang="ts">
import {ElNotification} from "element-plus";
import {onMounted, ref} from "vue";
import router from "../../router";

const open2 = () => {
  ElNotification({
    title: 'Prompt',
    message: 'This is a message that does not automatically close',
    duration: 1000,
  })
}

const editableTabsValue = ref('downloading')
const editableTabs = ref([
  {
    title: '下载',
    name: 'downloading',
  },
  {
    title: '已完成',
    name: 'downloaded',
  },
])

onMounted(() => {
  router.replace("/download/downloading");
  const element = document.querySelector('.el-tabs__content');
  if (element) {
    element.style.height = '88%';
  }
})

const doChange = () => {
  router.push(`/download/${editableTabsValue.value}`);
}
</script>

<template>
  <div class="download-info">
    <el-tabs
        v-model="editableTabsValue"
        type="card"
        class="demo-tabs"
        @tab-change="doChange"
        style="height: 100%"
    >
      <el-tab-pane
          v-for="item in editableTabs"
          :key="item.name"
          :label="item.title"
          :name="item.name"
          style="height: 100%"
      >
        <router-view></router-view>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<style scoped>
.download-info {
  height: 100%;
  width: 100%;
}
</style>