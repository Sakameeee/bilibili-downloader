<script setup lang="ts">
import {ElMessage} from "element-plus";
import {onMounted, ref} from "vue";
import {useConfigStore} from "../store/config.ts";
import {createInvoke} from "../utils/api.ts";
import { open } from '@tauri-apps/plugin-dialog';
import {Folder} from "@element-plus/icons-vue";
import {BiliConfig} from "../types";

const store = useConfigStore();
const config = ref<BiliConfig>({
  cookie: "",
  agent: "",
  save_path: ""
});

onMounted( async () => {
  await store.loadConfig();
  config.value = store.config as BiliConfig;
})

const reset = () => {
  console.log(config.value);
}

const submit = async () => {
  const param = config.value;
  const { status } = await createInvoke("update_config", { config: param });
  if (status === "ok") {
    ElMessage({
      message: '配置修改成功',
      type: 'success',
    });
  } else {
    ElMessage({
      message: '配置修改失败',
      type: 'warning',
    });
  }
}

const openFileDialog = async () => {
  // @ts-ignore
  config.value.save_path = await open({
    multiple: false,
    directory: true,
  });
}
</script>

<template>
  <div class="config">
    <el-form
        label-position="left"
        label-width="auto"
        :model="config"
        style="width: 85%; margin: 25px 25px"
    >
      <el-form-item label="cookie">
        <el-input v-model="config.cookie" />
      </el-form-item>
      <el-form-item label="agent">
        <el-input v-model="config.agent" />
      </el-form-item>

      <el-form-item label="save_path">
        <el-input v-model="config.save_path" disabled >
          <template #suffix>
            <el-button @click="openFileDialog">
              <template #default>
                <el-icon :size="16" color="#409efc">
                  <Folder/>
                </el-icon>
              </template>
            </el-button>
          </template>
        </el-input>
      </el-form-item>
      <el-divider border-style="none"/>
      <el-form-item>
        <el-button class="base-style" @click="submit">确认</el-button>
        <el-button @click="reset">重置</el-button>
      </el-form-item>
    </el-form>
  </div>
</template>

<style scoped>
.config {
  height: 100%;
  width: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
}
</style>