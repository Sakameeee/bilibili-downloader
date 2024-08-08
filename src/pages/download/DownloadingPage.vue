<script setup lang="ts">
import {onMounted, reactive, ref} from "vue";
import {Download, DownloadProgress} from "../../types";
import {Close, VideoPause, VideoPlay} from "@element-plus/icons-vue";
import {createInvoke} from "../../utils/api.ts";
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

  await createInvoke("download_progress");
})

const updateProgress = throttle((message: DownloadProgress) => {
  console.log(message);
  downloadingItems.value[downloadingItemsMap.get(message.id)].downloaded_size = message.chunk_length;
  console.log(downloadingItems.value[downloadingItemsMap.get(message.id)].downloaded_size);
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

const addDownload = async () => {
  const {status, data} = await createInvoke<Download[]>("add_download", {
    download: {
      id: 0,
      url: "https://upos-hz-mirrorakam.akamaized.net/upgcxcode/30/31/1449003130/1449003130_sr1-1-100035.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1723101733&gen=playurlv2&os=akam&oi=2406410771&trid=029f96cccb7b4f229cd8c110ac889bf3p&mid=182771883&platform=pc&og=hw&upsig=a4e94e9942900cf102adf66a6fab4a74&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&hdnts=exp=1723101733~hmac=8e15c1cd8b6598d9f74a95c7a272425c9a3bc524d90ec05db1cdcf1092dd54a6&bvc=vod&nettype=0&orderid=0,1&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=p_0_0&agrr=0&bw=743139&logo=80000000",
      file_name: "故人归乡",
      file_path: "D:\\download\\故人归乡.mp4",
      referer: "https://www.bilibili.com/bangumi/play/ep815162",
      total_size: 1032220135,
      downloaded_size: 0,
      status: "downloading",
      added_date: "31231",
      last_updated_date: "dada"
    }
  });
  if (status === "ok") {
    console.log(data);
    await loadData();
  }
}

const startDownload = async (id: number) => {
  // const {status, data} = await createInvoke<string>("start_downloading_file", {
  //   id: 30
  // });
  // console.log(status);
  // if (status !== "ok") {
  //   console.log(data);
  // }
  console.log(id);
  downloadingItems.value[downloadingItemsMap.get(id)].status = "downloading";
  console.log(downloadingItems.value[downloadingItemsMap.get(id)].status);
}

const stopDownload = async (id: number) => {
  // const {status, data} = await createInvoke<string>("stop_downloading_file", {
  //   id: 30
  // });
  // if (status !== "ok") {
  //   console.log(data);
  // }
  console.log(id);
  downloadingItems.value[downloadingItemsMap.get(id)].status = "paused";
  console.log(downloadingItems.value[downloadingItemsMap.get(id)].status);
}

const deleteDownloading = async (id: number) => {
  const {status, data} = await createInvoke("delete_download", {
    id: id,
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
        <el-button type="primary" size="small" @click="startDownload">全部开始</el-button>
        <el-button type="warning" size="small" @click="stopDownload">全部暂停</el-button>
        <el-button type="danger" size="small" @click="addDownload">删除选中</el-button>
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
              <template #default="scope">{{ scope.row.file_name }}</template>
            </el-table-column>
            <el-table-column width="200" align="center">
              <template #default="scope">
                <template v-if="(scope.row.status === PAUSE)">
                  <el-icon :size="16" color="#409efc" @click="startDownload(scope.row.id)">
                    <VideoPlay/>
                  </el-icon>
                </template>
                <template v-else>
                  <el-icon :size="16" color="#409efc" @click="stopDownload(scope.row.id)">
                    <VideoPause/>
                  </el-icon>
                </template>
                <el-divider direction="vertical" border-style="none"/>
                <el-icon :size="16" color="#409efc" @click="deleteDownloading(scope.row.id)">
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

el-icon:hover {
  color: var(--el-color-primary-light-9);
}
</style>