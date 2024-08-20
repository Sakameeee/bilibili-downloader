<script setup lang="ts">
import {onMounted, ref} from "vue";
import {useRouter} from "vue-router";
import {Search} from "@element-plus/icons-vue";

const input = ref("");
const placeholder = ref("");
const title = ref("");
const router = useRouter();

const doSearch = () => {
  const choice = router.currentRoute.value.path;
  let content = input.value;
  if (input.value.startsWith("https")) {
    if (choice === "/anime") {
      content = content.split("/")[5].split("?")[0];
    } else {
      content = content.split("/")[4];
    }
  }
  router.push(`${choice}/${content}`);
}

onMounted(() => {
  const choice = router.currentRoute.value.path;
  if (choice === "/anime") {
    placeholder.value = "请输入番剧链接或者 ep 号搜索番剧";
    title.value = "番剧搜索";
  } else {
    placeholder.value = "请输入视频链接或者 BV 号搜索视频";
    title.value = "视频搜索";
  }
})

router.afterEach((to) => {
  if (to.name === "番剧搜索" || to.name === "音视频搜索") {
    const choice = router.currentRoute.value.path;
    if (choice === "/anime") {
      placeholder.value = "请输入 ep 号搜索番剧，例如：ep285911";
      title.value = "番剧搜索";
    } else {
      placeholder.value = "请输入 BV 号搜索视频，例如：BV16f4y1z7kn";
      title.value = "视频搜索";
    }
  }
})
</script>

<template>
  <div class="search-page">
    <div style="display: flex; flex-direction: column; align-items: center; justify-content: flex-start">
      <el-image style="width: 200px; height: 150px; margin-top: 20px; margin-bottom: 20px"
                src="/banner.png" fit="cover"/>
      <el-text tag="b" style="margin-bottom: 20px">{{title}}</el-text>
    </div>
    <el-input
        v-model="input"
        style="width: 400px"
        size="large"
        :placeholder="placeholder"
        @keydown.enter="doSearch()"
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
  height: 100%;
  width: 100%;
  display: flex;
  flex-direction: column;
  justify-content: flex-start;
  padding-top: 50px;
  align-items: center;
}
</style>