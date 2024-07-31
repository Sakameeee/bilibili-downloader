<script setup lang="ts">
import {ref} from "vue";
import {ArrowLeft} from "@element-plus/icons-vue";
import Divider from "../components/Divider.vue";

const loading = ref(false);
const value = ref('4k');
const radio = ref(3);

const options = ['4k', '1080p', '720p', '360p'];

const goBack = () => {
  console.log('go back')
}
</script>

<template>
  <div class="download-video">
    <el-container style="height: 100%">
      <el-header>
        <el-page-header :icon="ArrowLeft" @back="goBack" title="返回">
          <template #content>
            <el-button>下载</el-button>
          </template>
        </el-page-header>
      </el-header>
      <el-main>
        <div class="content" v-loading="loading">
          <div class="left">
            <el-text tag="b" size="large" style="max-width: 300px" truncated>title</el-text>
            <el-image
                fit="cover"
                style="width: 300px; height: 150px"
                src="https://cube.elemecdn.com/6/94/4d3ea53c084bad6931a56d5158a48jpeg.jpeg">
              <template #error>
                <div class="image-slot">
                  <el-icon>
                    <icon-picture/>
                  </el-icon>
                </div>
              </template>
            </el-image>
            <el-text style="max-width: 300px" truncated>descriptiondescriptiondescriptiondescriptiondescriptiondescription</el-text>
          </div>
          <div class="right">
            <el-radio-group v-model="radio">
              <el-radio :value="3" border style="width: 120px">下载音视频</el-radio>
              <divider/>
              <el-radio :value="6" border style="width: 120px">下载视频</el-radio>
              <divider/>
              <el-radio :value="9" border style="width: 120px">下载音频</el-radio>
            </el-radio-group>
            <el-segmented v-model="value" :options="options" size="large"/>
          </div>
        </div>
      </el-main>
    </el-container>
  </div>
</template>

<style scoped>
.left {
  width: 50%;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: flex-start;
}
.right {
  width: 40%;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: flex-start;
}
.content {
  display: flex;
  width: 80%;
  height: 70%;
  justify-content: space-between;
  margin: 0 auto;
}
.download-video {
  height: 100%;
  width: 100%;
}
el-main {
  //display: flex;
  //justify-content: center;
  //align-items: center;
  align-content: center;
}
</style>