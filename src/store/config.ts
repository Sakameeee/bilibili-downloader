import {defineStore} from "pinia";
import {ref} from "vue";
import {BiliConfig} from "../types";
import {createInvoke} from "../utils/api.ts";

export const useConfigStore = defineStore("useConfigStore", () => {
  const config = ref<BiliConfig>();

  const loadConfig = async () => {
    const {status, data} = await createInvoke<BiliConfig>("get_config");

    if (status === "ok") {
      config.value = {...data};
      // console.log(config.value);
      return true;
    }
    return false;
  }

  const changeConfig = async (newConfig: BiliConfig) => {
    const {status} = await createInvoke<BiliConfig>("update_config", {config: newConfig});

    if (status === "ok" && newConfig) {
      config.value = newConfig;
      return true;
    }
    return false;
  }

  return {
    config,
    loadConfig,
    changeConfig
  }
});