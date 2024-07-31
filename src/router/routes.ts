import DownloadVideo from "../pages/DownloadVideo.vue";
import {RouteRecordRaw} from "vue-router";
import DownloadAnime from "../pages/DownloadAnime.vue";
import Config from "../pages/ConfigPage.vue";
import Search from "../pages/SearchPage.vue";

export const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "音视频搜索",
    component: Search,
  },
  {
    path: "/video",
    name: "音视频搜索",
    component: Search,
    children: [
      {
        path: "/video/download",
        name: "音视频下载",
        component: DownloadVideo,
      }
    ]
  },
  {
    path: "/anime",
    name: "番剧搜索",
    component: Search,
    children: [
      {
        path: "/anime/download",
        name: "番剧下载",
        component: DownloadAnime,
      }
    ]
  },
  {
    path: "/config",
    name: "设置",
    component: Config,
  }
];