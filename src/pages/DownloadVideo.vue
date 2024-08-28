<script setup lang="ts">
import {onMounted, ref} from "vue";
import {ArrowLeft} from "@element-plus/icons-vue";
import {useRoute} from "vue-router";
import {Video} from "../types";
import {createInvoke, notify} from "../utils/api.ts";

const loading = ref(false);
const value = ref(0);
const checkboxGroup1 = ref([0])
const route = useRoute();
const videoInfo = ref<Video>();

const options = ref<string[]>([]);
const downloadOptions = ref(["下载视频", "下载音频", "仅视频", "下载封面"]);
const downloadOption = ref(0);

onMounted(async () => {
  loading.value = true;
  const { status, data, err } = await createInvoke<Video>("get_videos", {bvId: route.query.id})
  if (status === "ok") {
    videoInfo.value = data;
  } else {
    await notify("获取视频信息失败", err);
  }
  options.value = videoInfo.value?.formats!;
  loading.value = false;
})

const addDownload = async () => {
  loading.value = true;
  let count = 0;
  for (let j = 0; j < checkboxGroup1.value.length; j++) {
    const i = checkboxGroup1.value[j];
    let video_url = "";
    let audio_url = "";
    switch (downloadOption.value) {
      case 0:
        video_url = videoInfo.value?.episodes[i].video_urls[value.value] ?? "";
        audio_url = videoInfo.value?.episodes[i].audio_url ?? "";
        break;
      case 1:
        audio_url = videoInfo.value?.episodes[i].audio_url ?? "";
        break;
      case 2:
        video_url = videoInfo.value?.episodes[i].video_urls[value.value] ?? "";
        break;
      case 3:
        await downloadCover();
        return;
    }
    const {status} = await createInvoke("add_download", {
      download: {
        id: 0,
        video_url: video_url,
        audio_url: audio_url,
        file_name: videoInfo.value?.episodes[i].title,
        file_path: "",
        referer: `https://www.bilibili.com/video/${videoInfo.value?.bvid}`,
        video_size: videoInfo.value?.episodes[i].sizes[value.value]??0,
        audio_size: 0,
        total_size: 0,
        downloaded_size: 0,
        status: "downloading",
        added_date: new Date().toLocaleDateString(),
        last_updated_date: new Date().toLocaleDateString()
      }
    });
    if (status !== "ok") {
      count++;
    }
  }
  loading.value = false;
  if (count === 0) {
    await notify("下载视频", "添加下载成功");
  } else {
    await notify("添加下载失败", `${count} 个视频添加失败`);
  }
}

const downloadCover = async () => {
  loading.value = true;
  const {status} = await createInvoke("download_cover", {
    url: videoInfo.value?.cover!,
  })
  if (status === "ok") {
    await notify("下载成功", "图片已保存在下载目录");
  } else {
    await notify("图片下载失败", "");
  }
  loading.value = false;
}

const goBack = () => {
  history.back();
}

const formatSeconds = (seconds: number) => {
  // 计算小时、分钟和秒
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = seconds % 60;

  // 格式化为两位数的字符串
  const formattedHours = String(hours).padStart(2, '0');
  const formattedMinutes = String(minutes).padStart(2, '0');
  const formattedSeconds = String(secs).padStart(2, '0');

  // 返回格式化后的时间字符串
  return `${formattedHours}:${formattedMinutes}:${formattedSeconds}`;
}
</script>

<template>
  <div class="download-video" v-loading="loading">
    <el-container style="height: 100%">
      <el-header>
        <el-page-header :icon="ArrowLeft" @back="goBack" title="返回" style="border-bottom: 1px solid var(--el-text-color-placeholder);
  padding-bottom: 5px">
          <template #content>
            <div style="display: flex; align-items: center">
              <el-button style="width: 70px" @click="addDownload">下载</el-button>
              <el-divider direction="vertical" border-style="none"/>
              <el-radio-group
                  v-model="downloadOption"
                  style="width: 100%;"
                  text-color="white"
                  fill="#ff99b3"
              >
                <div v-for="(format, index) in downloadOptions" style="margin-right: 7px; height: 32px">
                  <el-radio-button :value="index" :label="format"/>
                </div>
              </el-radio-group>
            </div>
          </template>
        </el-page-header>
      </el-header>
      <el-main style="padding-top: 0">
        <div class="content">
          <div class="left">
            <div style="margin-bottom: 10px">
              <el-text tag="b" size="large">{{ videoInfo?.title }}</el-text>
            </div>
            <div class="up-info">
              <el-avatar :size="40" :src="videoInfo?.author_avatar"/>
              <div style="margin-left: 20px">
                <el-text>{{ videoInfo?.author }}</el-text>
              </div>
            </div>
            <div>
              <el-image
                  fit="cover"
                  style="max-height: 95%; border-radius: 10px"
                  :src="videoInfo?.cover"
              />
            </div>
            <div class="description">
              <el-text>{{ videoInfo?.description }}</el-text>
            </div>
            <div class="data">
              <div class="info">
                <el-text tag="b">播放量:</el-text>
                <el-text>
                  {{ videoInfo?.play }}
                </el-text>
              </div>
              <el-divider direction="vertical" border-style="none"/>
              <el-divider direction="vertical" border-style="none"/>
              <div class="info">
                <el-text tag="b">弹幕量:</el-text>
                <el-text>
                  {{ videoInfo?.danmaku }}
                </el-text>
              </div>
              <el-divider direction="vertical" border-style="none"/>
              <el-divider direction="vertical" border-style="none"/>
              <div class="info">
                <el-text tag="b">日期:</el-text>
                <el-text>
                  {{ videoInfo?.date }}
                </el-text>
              </div>
            </div>
          </div>
          <div class="right">
            <div class="episode">
              <el-checkbox-group
                  v-model="checkboxGroup1"
                  style="overflow-y: scroll;"
                  text-color="white"
                  fill="#ff99b3"
              >
                <div v-for="(episode, index) in videoInfo?.episodes">
                  <div style="display: flex">
                    <el-checkbox-button :key="index" :value="index" style="width: 60%; flex: 1">
                      <template #default>
                        <div
                            style="height: 43px; width: 100%; display: flex; flex-direction: column; align-items: flex-start">
                          <div style="text-overflow: ellipsis; white-space: nowrap; height: 60%; align-content: center">
                            {{ index + 1 }}.{{ episode.title }}&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
                          </div>
                          <div style="font-size: small;">{{ formatSeconds(episode.duration) }}</div>
                        </div>
                      </template>
                    </el-checkbox-button>
                  </div>
                  <div style="height: 10px"></div>
                </div>
              </el-checkbox-group>
            </div>
            <el-radio-group
                v-model="value"
                style="width: 100%"
                text-color="white"
                fill="#ff99b3"
            >
              <div v-for="(format, index) in videoInfo?.formats" style="margin: 5px">
                <el-radio-button :value="index" :label="format"/>
              </div>
            </el-radio-group>
          </div>
        </div>
      </el-main>
    </el-container>
  </div>
</template>

<style scoped>
.left {
  width: 60%;
  height: 100%;
  display: flex;
  flex-direction: column;
  margin-right: 30px;
  align-items: flex-start;
  justify-content: space-between;
}

.right {
  width: 40%;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.content {
  display: flex;
  width: 100%;
  height: 100%;
  justify-content: space-between;
  margin: 0 auto;
}

.download-video {
  height: 100%;
  width: 100%;
}

.description {
  display: -webkit-box;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 3;
  overflow: hidden;
  text-overflow: ellipsis;
  margin-bottom: 10px;
}

.data {
  display: flex;
  width: 100%;
}

.info {
  background-color: var(--el-color-primary-light-9);
  padding: 16px;
  border-radius: 16px;
  box-sizing: border-box;
}

.episode {
  display: flex;
  margin-bottom: 20px;
  background-color: var(--el-color-primary-light-9);
  border-radius: 10px;
  padding: 20px;
  max-height: 70%;
}

.up-info {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 10px;
}
</style>