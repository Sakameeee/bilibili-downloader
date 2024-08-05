<script setup lang="ts">
import {ref} from "vue";
import {useRoute, useRouter} from "vue-router";
import {Search} from "@element-plus/icons-vue";
import {event} from "@tauri-apps/api";

const input = ref("");
const router = useRouter();

const doSearch = () => {
  const choice = router.currentRoute.value.path;
  console.log(choice);
  let content = input.value;
  if (input.value.startsWith("https")) {
    if (choice === "/anime") {
      content = content.split("/")[5].split("?")[0];
    } else {
      content = content.split("/")[4];
    }
  }
  console.log(content);
  router.push(`${choice}/${content}`);
}
</script>

<template>
  <div class="search-page">
    <div>
      <el-image style="width: 100px; height: 100px; margin-top: 20px; margin-bottom: 20px"
                src="https://fuss10.elemecdn.com/e/5d/4a731a90594a4af544c0c25941171jpeg.jpeg" fit="cover"/>
    </div>
    <el-input
        v-model="input"
        style="width: 400px"
        size="large"
        placeholder="请输入视频链接或者 BV 号"
        @keydown.enter="doSearch(event)"
    >
      <template #suffix @click="doSearch">
        <el-icon>
          <Search/>
        </el-icon>
      </template>
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