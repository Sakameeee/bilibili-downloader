<script setup lang="ts">
import {ElMessage, FormInstance} from "element-plus";
import {createInvoke} from "../utils/api.ts";
import {reactive, ref} from "vue";

const configItem = ref({
  cookie: '',
  agent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36 Edg/126.0.0.0',
  savePath: 'D:\\download',
})

const reset = () => {
  if (!formEl) return
  formEl.resetFields()
}

const submit = async () => {
  // if (!formEl) return;
  console.log(configItem.value.savePath);
  // const { code, data } = await createInvoke("update-config");
  ElMessage({
    message: '配置修改成功',
    type: 'success',
  });
}
</script>

<template>
  <el-form
      label-position="left"
      label-width="auto"
      :model="configItem"
      style="max-width: 600px; margin: 20px 25px"
  >
    <el-form-item label="cookie">
      <el-input v-model="configItem.cookie" />
    </el-form-item>
    <el-form-item label="agent">
      <el-input v-model="configItem.agent" />
    </el-form-item>
    <el-form-item label="savePath">
      <el-input v-model="configItem.savePath" />
    </el-form-item>
    <el-form-item>
      <el-button type="primary" @click="submit">确认</el-button>
      <el-button @click="reset">重置</el-button>
    </el-form-item>
  </el-form>
</template>

<style scoped>

</style>