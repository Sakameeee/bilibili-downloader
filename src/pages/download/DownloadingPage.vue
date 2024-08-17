<script setup lang="ts">
import {onMounted, reactive, ref} from "vue";
import {Download, DownloadProgress} from "../../types";
import {Close, VideoPause, VideoPlay} from "@element-plus/icons-vue";
import {createInvoke, notify} from "../../utils/api.ts";
import {throttle} from "lodash";
import {listen} from "@tauri-apps/api/event";

const downloadingItems = reactive({value: [] as Download[]})
const downloadingItemsMap = new Map();
const multipleSelection = ref<Download[]>([])
const PAUSE = "paused";

onMounted(async () => {
  await loadData();

  await listen<DownloadProgress>('progress', (message) => {
    updateProgress(message.payload);
  });
})

const updateProgress = throttle((message: DownloadProgress) => {
  if (message.chunk_length === -1) {
    notify(downloadingItems.value[downloadingItemsMap.get(message.id)].file_name, "文件下载失败!")
  }
  downloadingItems.value[downloadingItemsMap.get(message.id)].downloaded_size = message.chunk_length;
  if (downloadingItems.value[downloadingItemsMap.get(message.id)].downloaded_size === downloadingItems.value[downloadingItemsMap.get(message.id)].total_size) {
    loadData();
  }
}, 1000);

const handleSelectionChange = (val: Download[]) => {
  multipleSelection.value = val
}

const loadData = async () => {
  const {status, data} = await createInvoke<Download[]>("get_downloading_files");
  if (status === "ok") {
    downloadingItems.value = [...data];
    downloadingItems.value.forEach((item, index) => {
      downloadingItemsMap.set(item.id, index);
    })
    console.log(downloadingItemsMap);
  }
}

const startAllDownload = async () => {
  downloadingItems.value.forEach(item => {
    startDownload(item.id);
  })
}

const stopAllDownload = async () => {
  downloadingItems.value.forEach(item => {
    stopDownload(item.id);
  })
}

const startDownload = async (id: number) => {
  downloadingItems.value[downloadingItemsMap.get(id)].status = "downloading";
  const {status, err} = await createInvoke<string>("start_downloading_file", {
    id: id,
  });
  if (status !== "ok") {
    await notify("开始下载失败", err);
    console.log(err);
  }
}

const stopDownload = async (id: number) => {
  downloadingItems.value[downloadingItemsMap.get(id)].status = "paused";
  const {status, err} = await createInvoke<string>("stop_downloading_file", {
    id: id
  });
  if (status !== "ok") {
    await notify("暂停下载失败", err);
    console.log(err);
  }
}

const deleteDownloading = async (id: number) => {
  await createInvoke("delete_download", {
    id: id,
  });
  await loadData();
}

const deleteChosenDownloading = async () => {
  multipleSelection.value.forEach(item => {
    createInvoke("delete_download", {
      id: item.id,
    });
  });
  await loadData();
}
</script>

<template>
  <div class="downloading">
    <template v-if="false">
      <el-empty description="description"/>
    </template>
    <template v-else>
      <div class="control-bar">
        <el-progress :text-inside="true" :stroke-width="10" :percentage="70" style="height: 100%"/>
        <el-button type="primary" size="small" @click="startAllDownload">全部开始</el-button>
        <el-button type="warning" size="small" @click="stopAllDownload">全部暂停</el-button>
        <el-button type="danger" size="small" @click="deleteChosenDownloading">删除选中</el-button>
      </div>

      <div class="download-items">
        <el-scrollbar>
          <el-table
              :data="downloadingItems.value"
              style="width: 100%; font-size: small"
              height="400"
              ref="multipleTableRef"
              @selection-change="handleSelectionChange"
          >
            <el-table-column type="selection" width="55"/>
            <el-table-column property="file_name" label="文件名" width="120">
              <template #default="scope">
                <div style="text-overflow: ellipsis; white-space: nowrap;">{{ scope.row.file_name }}</div>
              </template>
            </el-table-column>
            <el-table-column width="200" align="center">
              <template #default="scope">
                <template v-if="(scope.row.status === PAUSE)">
                  <el-icon class="clickIcon" :size="16" color="#409efc" @click="startDownload(scope.row.id)">
                    <VideoPlay/>
                  </el-icon>
                </template>
                <template v-else>
                  <el-icon class="clickIcon" :size="16" color="#409efc" @click="stopDownload(scope.row.id)">
                    <VideoPause/>
                  </el-icon>
                </template>
                <el-divider direction="vertical" border-style="none"/>
                <el-icon class="clickIcon" :size="16" color="#409efc" @click="deleteDownloading(scope.row.id)">
                  <Close/>
                </el-icon>
              </template>
            </el-table-column>
            <el-table-column property="total_size" label="文件大小">
              <template #default="scope">
                {{
                  Math.round(scope.row.downloaded_size / (1024 * 1024))
                }}MB/{{ Math.round(scope.row.total_size / (1024 * 1024)) }}MB
              </template>
            </el-table-column>
            <el-table-column property="status" label="状态" align="center" style="padding-right: 200px">
              <template #default="scope">
                <el-progress
                    :percentage="Math.round(scope.row.downloaded_size / scope.row.total_size * 100)"
                    :stroke-width="15"
                    striped
                    striped-flow
                    :duration="10"
                    :text-inside="true"
                />
              </template>
            </el-table-column>
          </el-table>
        </el-scrollbar>
      </div>
    </template>
  </div>
</template>

<style scoped>
.downloading {
  height: 100%;
  width: 100%;
  position: relative;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.control-bar {
  width: 100%;
  display: flex;
  justify-content: flex-end;
}

.download-items {
  width: 100%;
}

.clickIcon {
  cursor: pointer;
  width: 30px;
  height: 20px;
  color: #ff99b3;
}

.clickIcon:hover {
  color: #ffd6dc;
}
</style>