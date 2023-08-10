<template>
  <div class="main-content">
    <el-tabs v-model="homeConf.activeMenu">
      <el-tab-pane label="开发类" name="developer">
        <el-row :gutter="24">
          <el-col :span="6" class="home-col">
            <el-card shadow="hover" >
              <div @click="wa_window('在线ssh', '/webssh', '')">
                <div class="home-item">
                  <SvgIcon name="shell" title="json" size="48"></SvgIcon>
                  <div>
                    <span>SSH</span>
                    <p>ssh连接</p>
                  </div>
                </div>
              </div>
            </el-card>
          </el-col>
          <el-col :span="6">
            <el-card shadow="hover" @click="wa_window('开发常用工具', '/onlineTool/json', 'json')">
              <div class="home-item">
                <SvgIcon name="json" title="json" size="48"></SvgIcon>
                <div>
                  <span>JSON</span>
                  <p>json格式化、压缩，语法检查</p>
                </div>
              </div>
            </el-card>
          </el-col>
          <el-col :span="6">
            <el-card shadow="hover" @click="wa_window('开发常用工具', '/onlineTool/generate', 'pwdGenerate')">
              <div class="home-item">
                <SvgIcon name="generate" title="json" size="48"></SvgIcon>
                <div>
                  <span>随机密码生成</span>
                  <p>随机密码生成</p>
                </div>
              </div>
            </el-card>
          </el-col>
          <el-col :span="6">
            <el-card shadow="hover" @click="wa_window('开发常用工具', '/onlineTool/generate', 'uuid')">
              <div class="home-item">
                <SvgIcon name="uuid" title="uuid" size="48"></SvgIcon>
                <div>
                  <span>uuid生成</span>
                  <p>uuid生成</p>
                </div>
              </div>
            </el-card>
          </el-col>
          <el-col :span="6">
            <el-card shadow="hover" @click="wa_window('开发常用工具', '/onlineTool/generate', 'cron')">
              <div class="home-item">
                <SvgIcon name="cron" title="cron" size="48"></SvgIcon>
                <div>
                  <span>cron生成</span>
                  <p>cron生成</p>
                </div>
              </div>
            </el-card>
          </el-col>
          <el-col :span="6">
            <el-card shadow="hover" @click="wa_window('开发常用工具', '/onlineTool/codec', 'base64Encrypt')">
              <div class="home-item">
                <SvgIcon name="encrypt" title="加密" size="48"></SvgIcon>
                <div>
                  <span>base64编码</span>
                  <p>base64编码</p>
                </div>
              </div>
            </el-card>
          </el-col>
          <el-col :span="6">
            <el-card shadow="hover">
              <div class="home-item">
                <SvgIcon name="setting" title="系统设置" size="48"></SvgIcon>
                <div>
                  <span>系统设置</span>
                  <p>系统设置</p>
                </div>
              </div>
            </el-card>
          </el-col>
        </el-row>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<script lang="ts">
  import { defineComponent, ref } from 'vue';
  import { invoke } from "@tauri-apps/api/tauri";
  import { useCoreStore } from "@/store";
  import { storeToRefs } from "pinia";
  import SvgIcon from "@/components/SvgIcon.vue";
  
  export default defineComponent({
    components: {
      SvgIcon
    },

    setup() {
      const coreStore = useCoreStore();
      const { homeConf } = storeToRefs(coreStore);

      const current = ref<string[]>(['']);

      const wa_window = (title : string, path: string, pathActive: string) => {
        let pathArr = path.split("/");
        pathArr.push(pathActive);
        let activeKeyMap = new Map<string, string>();
        let pathName = "/";
        for (let i = 1; i < pathArr.length - 1; i++) {
          pathName += pathArr[i] + "/";
          activeKeyMap.set(pathName.substring(0, pathName.length - 1), pathArr[i + 1]);
        }
        invoke("save_route_active_keys", { activeKeyMap: activeKeyMap});
        invoke('wa_window', { title: title, label: title, url: "index.html#", path: path + "?pathActive=" + pathActive });
      }

      return {
        current,
        wa_window,
        homeConf
      }
    }
  });
</script>
