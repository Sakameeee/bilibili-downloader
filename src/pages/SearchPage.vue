<script setup lang="ts">
import {ref} from "vue";
import {createInvoke} from "../utils/api";
import {useRoute, useRouter} from "vue-router";
import {Search} from "@element-plus/icons-vue";

const input = ref("");
const selectedTab = ref("");
const route = useRoute();

const doSearch = async () => {
  const { status, data } = await createInvoke<string[]>("do_search", {
    key: selectedTab.value,
    text: input.value,
  });
}
</script>

<template>
<div class="search-page">
  <div>
    <el-image style="width: 100px; height: 100px; margin-top: 20px; margin-bottom: 20px" src="https://fuss10.elemecdn.com/e/5d/4a731a90594a4af544c0c25941171jpeg.jpeg" fit="cover" />
  </div>
  <el-input
      v-model="input"
      style="width: 400px"
      size="large"
      placeholder="请输入视频链接或者 BV 号"
  >
    <template #suffix @click="doSearch"><el-icon><Search/></el-icon></template>
  </el-input>
</div>
</template>

<style scoped>
.search-page {
  height: 50%;
  width: 100%;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}
</style>