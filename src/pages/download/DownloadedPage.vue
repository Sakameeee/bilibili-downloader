<script setup lang="ts">
import {ref} from "vue";
import {Download} from "../../types";
import {Close, Folder} from "@element-plus/icons-vue";

const downloadedItems = ref<Download[]>([
  {
    id: 1,
    url: "https://xy61x159x92x163xy.mcdn.bilivideo.cn:4483/upgcxcode/30/31/1449003130/1449003130_sr1-1-100035.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722607117&gen=playurlv2&os=mcdn&oi=607962291&trid=0000224e06a3daac483eab24110c2f0369c7p&mid=182771883&platform=pc&og=hw&upsig=839dc4bb08565942b1703a9f15b70c9c&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50002941&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=p_0_0&agrr=0&bw=743139&logo=A0020000",
    file_name: "故人归乡",
    file_path: "D:\\code\\frontend\\tauri\\rust-test\\故人归乡.mp4",
    referer: "https://www.bilibili.com/bangumi/play/ep815162",
    total_size: 1032220135,
    downloaded_size: 490278303,
    status: "downloading",
    added_date: "2024/8/3",
    last_updated_date: "2024/8/3",
  },
  {
    id: 2,
    url: "https://xy61x159x92x163xy.mcdn.bilivideo.cn:4483/upgcxcode/30/31/1449003130/1449003130_sr1-1-100035.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722607117&gen=playurlv2&os=mcdn&oi=607962291&trid=0000224e06a3daac483eab24110c2f0369c7p&mid=182771883&platform=pc&og=hw&upsig=839dc4bb08565942b1703a9f15b70c9c&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50002941&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=p_0_0&agrr=0&bw=743139&logo=A0020000",
    file_name: "故人归乡",
    file_path: "D:\\code\\frontend\\tauri\\rust-test\\故人归乡.mp4",
    referer: "https://www.bilibili.com/bangumi/play/ep815162",
    total_size: 1032220135,
    downloaded_size: 490278303,
    status: "downloading",
    added_date: "2024/8/3",
    last_updated_date: "2024/8/3",
  },
  {
    id: 3,
    url: "https://xy61x159x92x163xy.mcdn.bilivideo.cn:4483/upgcxcode/30/31/1449003130/1449003130_sr1-1-100035.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722607117&gen=playurlv2&os=mcdn&oi=607962291&trid=0000224e06a3daac483eab24110c2f0369c7p&mid=182771883&platform=pc&og=hw&upsig=839dc4bb08565942b1703a9f15b70c9c&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50002941&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=p_0_0&agrr=0&bw=743139&logo=A0020000",
    file_name: "故人归乡",
    file_path: "D:\\code\\frontend\\tauri\\rust-test\\故人归乡.mp4",
    referer: "https://www.bilibili.com/bangumi/play/ep815162",
    total_size: 1032220135,
    downloaded_size: 490278303,
    status: "downloading",
    added_date: "2024/8/3",
    last_updated_date: "2024/8/3",
  },
  {
    id: 4,
    url: "https://xy61x159x92x163xy.mcdn.bilivideo.cn:4483/upgcxcode/30/31/1449003130/1449003130_sr1-1-100035.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722607117&gen=playurlv2&os=mcdn&oi=607962291&trid=0000224e06a3daac483eab24110c2f0369c7p&mid=182771883&platform=pc&og=hw&upsig=839dc4bb08565942b1703a9f15b70c9c&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50002941&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=p_0_0&agrr=0&bw=743139&logo=A0020000",
    file_name: "故人归乡",
    file_path: "D:\\code\\frontend\\tauri\\rust-test\\故人归乡.mp4",
    referer: "https://www.bilibili.com/bangumi/play/ep815162",
    total_size: 1032220135,
    downloaded_size: 490278303,
    status: "downloading",
    added_date: "2024/8/3",
    last_updated_date: "2024/8/3",
  },
  {
    id: 5,
    url: "https://xy61x159x92x163xy.mcdn.bilivideo.cn:4483/upgcxcode/30/31/1449003130/1449003130_sr1-1-100035.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722607117&gen=playurlv2&os=mcdn&oi=607962291&trid=0000224e06a3daac483eab24110c2f0369c7p&mid=182771883&platform=pc&og=hw&upsig=839dc4bb08565942b1703a9f15b70c9c&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50002941&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=p_0_0&agrr=0&bw=743139&logo=A0020000",
    file_name: "故人归乡",
    file_path: "D:\\code\\frontend\\tauri\\rust-test\\故人归乡.mp4",
    referer: "https://www.bilibili.com/bangumi/play/ep815162",
    total_size: 1032220135,
    downloaded_size: 490278303,
    status: "downloading",
    added_date: "2024/8/3",
    last_updated_date: "2024/8/3",
  },
  {
    id: 6,
    url: "https://xy61x159x92x163xy.mcdn.bilivideo.cn:4483/upgcxcode/30/31/1449003130/1449003130_sr1-1-100035.m4s?e=ig8euxZM2rNcNbdlhoNvNC8BqJIzNbfqXBvEqxTEto8BTrNvN0GvT90W5JZMkX_YN0MvXg8gNEV4NC8xNEV4N03eN0B5tZlqNxTEto8BTrNvNeZVuJ10Kj_g2UB02J0mN0B5tZlqNCNEto8BTrNvNC7MTX502C8f2jmMQJ6mqF2fka1mqx6gqj0eN0B599M=&uipk=5&nbs=1&deadline=1722607117&gen=playurlv2&os=mcdn&oi=607962291&trid=0000224e06a3daac483eab24110c2f0369c7p&mid=182771883&platform=pc&og=hw&upsig=839dc4bb08565942b1703a9f15b70c9c&uparams=e,uipk,nbs,deadline,gen,os,oi,trid,mid,platform,og&mcdnid=50002941&bvc=vod&nettype=0&orderid=0,3&buvid=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc&build=0&f=p_0_0&agrr=0&bw=743139&logo=A0020000",
    file_name: "故人归乡",
    file_path: "D:\\code\\frontend\\tauri\\rust-test\\故人归乡.mp4",
    referer: "https://www.bilibili.com/bangumi/play/ep815162",
    total_size: 1032220135,
    downloaded_size: 490278303,
    status: "downloading",
    added_date: "2024/8/3",
    last_updated_date: "2024/8/3",
  },
])
</script>

<template>
  <div class="downloaded">
    <div class="control-bar">
      <el-button type="danger" size="small">删除选中</el-button>
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
              <el-icon :size="16" color="#409efc">
                <Folder/>
              </el-icon>
              <el-divider direction="vertical" border-style="none"/>
              <el-icon :size="16" color="#409efc">
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
</style>