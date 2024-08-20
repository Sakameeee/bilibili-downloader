import router from "../router";
import {useConfigStore} from "../store/config.ts";
import {ElMessage} from "element-plus";

router.beforeEach(async (to) => {
  if (to.name === "音视频下载" || to.name === "番剧下载") {
    const store = useConfigStore();
    const config = store.config;

    if (config?.cookie === "") {
      ElMessage({
        message: '请先配置 cookie',
        type: 'warning',
      });
      await router.replace("/config");
      return;
    }
  }
});