<script setup lang="ts">

import {ArrowLeft} from "@element-plus/icons-vue";
import {Anime} from "../types";
import {onMounted, ref} from "vue";
import {createInvoke, notify} from "../utils/api.ts";
import {useRoute} from "vue-router";

const loading = ref(false);
const value = ref(0);
const route = useRoute();
const checkboxGroup1 = ref([0])
const animeInfo = ref<Anime>();

const options = ref<string[]>([]);

onMounted(async () => {
  loading.value = true;
  const { status, data, err } = await createInvoke<Anime>("get_animates", {epId: route.params.epid})
  if (status === "ok" && data.count !== 0) {
    animeInfo.value = data;
  } else {
    await notify("获取番剧信息失败", err);
  }
  options.value = animeInfo.value?.formats!;
  loading.value = false;
})

const addDownload = async () => {
  loading.value = true;
  let count = 0;
  for (let j = 0; j < checkboxGroup1.value.length; j++) {
    let i = checkboxGroup1.value[j];
    if (animeInfo.value?.episodes[i].audio_url === "") {
      await notify("无剧集信息，请重新配置 cookie", `${animeInfo.value?.episodes[i].title}下载出错`);
      loading.value = false;
      return;
    }
    const {status} = await createInvoke("add_download", {
      download: {
        id: 0,
        video_url: animeInfo.value?.episodes[i].video_urls[value.value],
        audio_url: animeInfo.value?.episodes[i].audio_url,
        file_name: `${i}.${animeInfo.value?.episodes[i].title}`,
        file_path: "",
        referer: `https://www.bilibili.com/bangumi/play/ep${animeInfo.value?.episodes[i].epId}`,
        video_size: Number(animeInfo.value?.episodes[i].sizes[value.value]) ?? 0,
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
  if (count === 0) {
    await notify("下载番剧", "添加下载成功");
  } else {
    await notify("添加下载失败", `${count} 个视频添加失败`);
  }
  loading.value = false;
}

const goBack = () => {
  history.back();
}
</script>

<template>
  <div class="download-video" v-loading="loading">
    <el-container style="height: 100%">
      <el-header>
        <el-page-header :icon="ArrowLeft" @back="goBack" title="返回" style="border-bottom: 1px solid var(--el-text-color-placeholder);
  padding-bottom: 5px">
          <template #content>
            <el-button @click="addDownload">下载</el-button>
          </template>
        </el-page-header>
      </el-header>
      <el-main>
        <div class="content">
          <div class="left">
            <div style="display: flex; width: 100%">
              <div
                  style="display: flex; flex-direction: column; align-items: flex-start; justify-content: space-between; margin-right: 20px">
                <div>
                  <el-text tag="b" size="large">{{ animeInfo?.title }}</el-text>
                </div>
                <div>
                  <el-text tag="b">类型:&nbsp;</el-text>
                  <el-text>
                    {{ animeInfo?.types }}
                  </el-text>
                </div>
                <div>
                  <el-text tag="b">年份:&nbsp;</el-text>
                  <el-text>{{ animeInfo?.date }}</el-text>
                </div>
                <div>
                  <el-text tag="b">评分:&nbsp;</el-text>
                  <el-text>{{ animeInfo?.score }}</el-text>
                </div>
                <div>
                  <el-text tag="b">集数:&nbsp;</el-text>
                  <el-text>{{ animeInfo?.count }}</el-text>
                </div>
              </div>
              <div>
                <el-image
                    fit="cover"
                    style="width: 160px; height: 200px; border-radius: 10px"
                    :src="animeInfo?.cover"
                />
              </div>
            </div>
            <div class="description">
              <el-text tag="b">描述:</el-text>
              <div style="height: 16px"></div>
              <el-text>{{ animeInfo?.description }}</el-text>
            </div>
            <div class="data">
              <div class="info">
                <el-text tag="b">播放量:</el-text>
                <el-text>
                  {{ animeInfo?.play?.split("·")[0] }}
                </el-text>
              </div>
              <el-divider direction="vertical" border-style="none"/>
              <el-divider direction="vertical" border-style="none"/>
              <div class="info">
                <el-text tag="b">弹幕量:</el-text>
                <el-text>
                  {{ animeInfo?.play?.split("·")[1] }}
                </el-text>
              </div>
              <el-divider direction="vertical" border-style="none"/>
              <el-divider direction="vertical" border-style="none"/>
              <div class="info">
                <el-text tag="b">系列追番:</el-text>
                <el-text>
                  {{
                    animeInfo?.play?.split("·")[2]
                  }}
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
                <div v-for="(episode, index) in animeInfo?.episodes">
                  <div style="display: flex">
                    <el-image
                        fit="cover"
                        style="width: 100px; height: 60px"
                        :src="episode.cover"
                    />
                    <el-checkbox-button :key="index" :value="index" style="width: 60%; flex: 1">
                      <template #default>
                        <div style="height: 43px; width: 100%">
                          <div
                              style="text-overflow: ellipsis; white-space: nowrap; height: 100%; align-content: center">
                            {{ index + 1 }}.{{ episode.title }}&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
                          </div>
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
              <div v-for="(format, index) in animeInfo?.formats" style="margin: 5px">
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
span {
  color: var(--el-color-info-dark-2);
}

.left {
  width: 60%;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  margin-right: 30px;
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
  margin-top: 30px;
  display: -webkit-box;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 6;
  overflow: hidden;
  text-overflow: ellipsis;
}

.data {
  margin-top: 30px;
  display: flex;
  width: 100%;
}

.info {
  background-color: var(--el-color-primary-light-9);
  padding: 16px;
  border-radius: 16px;
}

.episode {
  display: flex;
  margin-bottom: 20px;
  background-color: var(--el-color-primary-light-9);
  border-radius: 10px;
  padding: 20px;
  max-height: 70%;
}
</style>