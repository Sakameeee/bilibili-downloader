<script setup lang="ts">
import {onMounted, ref} from "vue";
import router from "../../router";

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
  const element = document.querySelector('.el-tabs__content');
  if (element) {
    // @ts-ignore
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