import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import Antd from "ant-design-vue";
import 'ant-design-vue/dist/antd.css';
//引入路由
import router from './router';
import 'lib-flexible/flexible';

createApp(App).use(Antd).use(router).mount("#app");
