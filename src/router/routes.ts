import DownloadVideo from "../pages/DownloadVideo.vue";
import {RouteRecordRaw} from "vue-router";
import DownloadAnime from "../pages/DownloadAnime.vue";
import Config from "../pages/ConfigPage.vue";
import Search from "../pages/SearchPage.vue";
import DownloadInfo from "../pages/download/DownloadInfo.vue";
import DownloadingPage from "../pages/download/DownloadingPage.vue";
import DownloadedPage from "../pages/download/DownloadedPage.vue";

export const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    redirect: "/video"
  },
  {
    path: "/video",
    name: "音视频搜索",
    component: Search,
  },
  {
    path: "/video/:bvid",
    name: "音视频下载",
    component: DownloadVideo,
  },
  {
    path: "/anime",
    name: "番剧搜索",
    component: Search,
  },
  {
    path: "/anime/:epid",
    name: "番剧下载",
    component: DownloadAnime,
  },
  {
    path: "/download",
    name: "下载详情",
    component: DownloadInfo,
    children: [
      {
        path: "/download/downloading",
        name: "下载页面",
        component: DownloadingPage,
      },
      {
        path: "/download/downloaded",
        name: "已下载页面",
        component: DownloadedPage
      }
    ]
  },
  {
    path: "/config",
    name: "设置",
    component: Config,
  }
];