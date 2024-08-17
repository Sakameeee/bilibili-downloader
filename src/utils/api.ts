import {invoke} from "@tauri-apps/api/core";
import {Response} from "../types";
import {isPermissionGranted, requestPermission, sendNotification} from "@tauri-apps/plugin-notification";

export async function createInvoke<T = any>(api: string, params?: any): Promise<Response<T>> {
  const data: Response<T> = await invoke(api, params);

  if (data) {
    return data;
  } else {
    return {
      status: "err",
      data: null,
      err: "data is undefined",
    } as Response<T>;
  }
}

export async function notify(title: string, body: string) {
  // 你有发送通知的权限吗？
  let permissionGranted = await isPermissionGranted();

  // 如果没有，我们需要请求它
  if (!permissionGranted) {
    const permission = await requestPermission();
    permissionGranted = permission === 'granted';
  }

  // 一旦获得许可，我们就可以发送通知
  if (permissionGranted) {
    sendNotification({title: title, body: body});
  }
}