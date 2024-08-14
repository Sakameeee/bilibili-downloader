<script setup lang="ts">
import {onMounted, ref} from "vue";
import {ArrowLeft} from "@element-plus/icons-vue";
import Divider from "../components/Divider.vue";
import {useRoute} from "vue-router";
import {Video} from "../types";

const loading = ref(false);
const value = ref(0);
const radio = ref(3);
const checkboxGroup1 = ref([0])
const route = useRoute();
const videoInfo = ref<Video>(<Video>{
  duration: 0,
  bvid: "BV16f4y1z7kn",
  cid: "301428697",
  title: "MIT 6.824 2021 分布式系统 [中英文字幕]_哔哩哔哩_bilibili",
  description: "是MIT 6.824 2021 分布式系统 [中英文字幕]的第NaN集视频，该合集共计22集，视频收藏或关注UP主，及时了解更多相关视频内容。是MIT 6.824 2021 分布式系统 [中英文字幕]的第NaN集视频，该合集共计22集，视频收藏或关注UP主，及时了解更多相关视频内容。是MIT 6.824 2021 分布式系统 [中英文字幕]的第NaN集视频，该合集共计22集，视频收藏或关注UP主，及时了解更多相关视频内容。是MIT 6.824 2021 分布式系统 [中英文字幕]的第NaN集视频，该合集共计22集，视频收藏或关注UP主，及时了解更多相关视频内容。是MIT 6.824 2021 分布式系统 [中英文字幕]的第NaN集视频，该合集共计22集，视频收藏或关注UP主，及时了解更多相关视频内容。",
  cover: "http://i1.hdslb.com/bfs/archive/f9a6c834d4dff5d349d451900ec9724302b36135.jpg",
  author: "mayf09",
  author_avatar: "https://i0.hdslb.com/bfs/face/341af0339a1eee3e4b518f03de2ae5481d7dc474.jpg",
  count: 22,
  danmaku: "269",
  play: "52834",
  date: "2021-02-23",
  formats: ["高清 1080P", "高清 720P", "清晰 480P", "流畅 360P"],
  episodes: [{
    cid: "301428697",
    title: "Lecture 1 - Introduction",
    video_urls: ["https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30032.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=d37e6b34f01330c2e3e0126f28868c9b&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=49880&logo=A0020000", "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30011.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=43409cf50f79c1a58fa6f504b8396ea4&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=10573&logo=A0020000"],
    audio_url: "https://xy111x225x218x142xy.mcdn.bilivideo.cn:4483/upgcxcode/97/86/301428697/301428697-1-30280.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722621726&gen=playurlv2&os=mcdn&oi=1035876564&trid=0000db23ebfe93334781b190a097b9646038u&mid=0&platform=pc&og=hw&upsig=0fe2d0a79cadffcf846f4ef0c8eface2&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50004915&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=u_0_0&agrr=0&bw=16193&logo=A0020000",
    duration: 4749,
  }]
});

const options = ref<string[]>([]);
const downloadOptions = ref(["下载视频", "下载音频", "仅视频", "下载封面"]);
const downloadOption = ref(0);

onMounted(() => {
  loading.value = true;
  setTimeout(() => {
    loading.value = false;
  }, 2000);
  console.log(route.params.bvid);
  options.value = videoInfo.value.formats;
  console.log(radio.value);
})

const goBack = () => {
  history.back();
}

const formatSeconds = (seconds) => {
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
              <el-button class="base-style" style="width: 70px">下载</el-button>
              <el-divider direction="vertical" border-style="none" />
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
      <el-main>
        <div class="content">
          <div class="left">
            <div>
              <el-text tag="b" size="large">{{ videoInfo.title }}</el-text>
            </div>
            <div class="up-info">
              <el-avatar :size="40" :src="videoInfo.author_avatar"/>
              <div style="margin-left: 20px">
                <el-text>{{ videoInfo.author }}</el-text>
              </div>
            </div>
            <div>
              <el-image
                  fit="cover"
                  style="border-radius: 10px"
                  :src="videoInfo.cover"
              />
            </div>
            <div class="description">
              <el-text>{{ videoInfo.description }}</el-text>
            </div>
            <div class="data">
              <div class="info">
                <el-text tag="b">播放量:</el-text>
                <el-text>
                  {{ videoInfo.play }}
                </el-text>
              </div>
              <el-divider direction="vertical" border-style="none"/>
              <el-divider direction="vertical" border-style="none"/>
              <div class="info">
                <el-text tag="b">弹幕量:</el-text>
                <el-text>
                  {{ videoInfo.danmaku }}
                </el-text>
              </div>
              <el-divider direction="vertical" border-style="none"/>
              <el-divider direction="vertical" border-style="none"/>
              <div class="info">
                <el-text tag="b">日期:</el-text>
                <el-text>
                  {{videoInfo.date}}
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
                <div v-for="(episode, index) in videoInfo.episodes">
                  <div style="display: flex">
                    <el-checkbox-button :key="index" :value="index" style="width: 60%; flex: 1">
                      <template #default>
                        <div style="height: 43px; width: 100%; display: flex; flex-direction: column; align-items: flex-start">
                          <div style="text-overflow: ellipsis; white-space: nowrap; height: 60%; align-content: center">{{ index + 1 }}.{{ episode.title }}&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</div>
                          <div style="font-size: small;">{{formatSeconds(episode.duration)}}</div>
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
              <div v-for="(format, index) in videoInfo.formats" style="margin: 5px">
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
}

.data {
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

.up-info {
  display: flex;
  align-items: center;
  justify-content: space-between;
}
</style>