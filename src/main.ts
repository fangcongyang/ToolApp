import { createApp } from "vue";
import "@/assets/scss/theme.scss";
import App from "./App.vue";
import Antd from "ant-design-vue";
import { createPinia } from "pinia";
import "virtual:svg-icons-register";
import "ant-design-vue/dist/antd.css";
import ElementPlus from "element-plus";
import "element-plus/theme-chalk/index.css";
//引入路由
import router from "./router";
import "lib-flexible/flexible";

const pinia = createPinia();

createApp(App)
  .use(Antd)
  .use(ElementPlus)
  .use(pinia)
  .use(router)
  .mount("#app");
