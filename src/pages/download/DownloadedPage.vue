<script setup lang="ts">
import {onMounted, ref} from "vue";
import {Download} from "../../types";
import {Close, Folder} from "@element-plus/icons-vue";
import {createInvoke} from "../../utils/api.ts";

const downloadedItems = ref<Download[]>();
const downloadedItemsMap = new Map();
const multipleSelection = ref<Download[]>([]);
const search = ref("");

onMounted(async () => {
  await loadData();
})

const loadData = async () => {
  const {status, data} = await createInvoke<Download[]>("get_downloaded_files");
  if (status === "ok") {
    downloadedItems.value = [...data];
    downloadedItems.value.forEach((item, index) => {
      downloadedItemsMap.set(item.id, index);
    })
    console.log(downloadedItemsMap);
  }
}

const searchDownloaded = async () => {
  const {status, data} = await createInvoke<Download[]>("search_downloaded", {
    text: search.value,
  });
  if (status === "ok") {
    downloadedItems.value = data;
    console.log(downloadedItems.value);
  }
  search.value = "";
}

const deleteDownloaded = async (id: number) => {
  await createInvoke("delete_download", {
    id: id,
  });
  await loadData();
}

const openFileDir = async (filePath: string) => {
  await createInvoke("open_file_directory", {
    path: filePath
  });
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
  <div class="downloaded">
    <div class="control-bar">
      <el-input v-model="search" size="small" placeholder="输入文件名搜索" clearable
                style="width: 200px; margin-right: 20px" @keydown.enter="searchDownloaded"/>
      <el-button type="danger" size="small" @click="deleteChosenDownloading">删除选中</el-button>
    </div>

    <div class="download-items">
      <el-scrollbar>
        <el-table
            :data="downloadedItems"
            style="width: 100%; font-size: small"
            height="400"
        >
          <el-table-column type="selection" width="55"/>
          <el-table-column property="file_name" label="文件名" width="120">
            <template #default="scope">{{ scope.row.file_name }}</template>
          </el-table-column>
          <el-table-column width="200" align="center">
            <template #default="scope">
              <el-icon class="clickIcon" :size="16" color="#409efc" @click="openFileDir(scope.row.file_path)">
                <Folder/>
              </el-icon>
              <el-divider direction="vertical" border-style="none"/>
              <el-icon class="clickIcon" :size="16" color="#409efc" @click="deleteDownloaded(scope.row.id)">
                <Close/>
              </el-icon>
            </template>
          </el-table-column>
          <el-table-column property="total_size" label="文件大小">
            <template #default="scope">{{ Math.round(scope.row.total_size / (1024 * 1024)) }}MB</template>
          </el-table-column>
          <el-table-column property="date" label="时间" align="center">
            <template #default="scope">
              {{ scope.row.added_date }}
            </template>
          </el-table-column>
        </el-table>
      </el-scrollbar>
    </div>
  </div>
</template>

<style scoped>
.downloaded {
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