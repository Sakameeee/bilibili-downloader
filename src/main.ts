import {createApp} from "vue";
import App from "./App.vue";
import {createPinia} from "pinia";
import ElementPlus from 'element-plus';
import 'element-plus/dist/index.css';
import 'element-plus/theme-chalk/dark/css-vars.css';
import * as ElementPlusIconsVue from '@element-plus/icons-vue';
import router from "./router";

const pinia = createPinia();
const app = createApp(App);
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component)
}
app.use(pinia);
app.use(router);
app.use(ElementPlus);
app.mount("#app");
