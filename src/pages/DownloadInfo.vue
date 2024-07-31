<script setup lang="ts">
import {ElNotification} from "element-plus";

const open2 = () => {
  ElNotification({
    title: 'Prompt',
    message: 'This is a message that does not automatically close',
    duration: 1000,
  })
}
</script>

<template>
  <el-affix position="bottom" :offset="20">
    <el-button type="primary">Offset bottom 20px</el-button>
  </el-affix>

  <el-backtop :bottom="100">
    <div
        style="
        height: 100%;
        width: 100%;
        background-color: var(--el-bg-color-overlay);
        box-shadow: var(--el-box-shadow-lighter);
        text-align: center;
        line-height: 40px;
        color: #1989fa;
      "
    >
      UP
    </div>
  </el-backtop>

  <el-steps style="max-width: 600px" :active="active" finish-status="success">
    <el-step title="Step 1" />
    <el-step title="Step 2" />
    <el-step title="Step 3" />
  </el-steps>

  <el-empty description="description" />

  <el-pagination background layout="prev, pager, next" :total="1000" />

  <el-progress type="circle" :percentage="100" status="success" />

  <el-progress
      :percentage="100"
      :stroke-width="15"
      status="success"
      striped
      striped-flow
      :duration="10"
  />
</template>

<style scoped>
#video {
  height: 100%;
  width: 100%;
  background-color: var(--el-color-primary-light-9);
}
</style>