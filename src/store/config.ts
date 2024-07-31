import {defineStore} from "pinia";
import {ref, watch} from "vue";
import {BiliConfig} from "../types";
// import {createInvoke} from "../utils/api";

export const useConfigStore = defineStore("useConfigStore", () => {
  const config = ref<BiliConfig>();

  watch(config, async newConfig => {
    await changeConfig(newConfig);
  })

  const loadConfig = async () => {
    return null;
  }

  const changeConfig = async (config: BiliConfig | undefined) => {
    return config;
  }

  return {
    config,
    loadConfig,
  }
});