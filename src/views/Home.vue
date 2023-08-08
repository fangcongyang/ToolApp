<template>
  <div class="main-content">
    <el-tabs v-model="homeConf.activeMenu">
      <el-tab-pane label="开发类" name="developer">
        <el-row :gutter="24">
          <el-col :span="6">
            <el-card shadow="hover" >
              <div @click="wa_window('在线ssh', 'index.html#/webssh')">
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
            <el-card shadow="hover" @click="wa_window('开发常用工具', 'index.html#/onlineTool/json')">
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
            <el-card shadow="hover" @click="wa_window('开发常用工具', 'index.html#/onlineTool/generate')">
              <div class="home-item">
                <SvgIcon name="generate" title="json" size="48"></SvgIcon>
                <div>
                  <span>在线生成</span>
                  <p>密码生成，uuid生成，cron生成</p>
                </div>
              </div>
            </el-card>
          </el-col>
          <el-col :span="6">
            <el-card shadow="hover" @click="wa_window('开发常用工具', 'index.html#/onlineTool/generate')">
              <div class="home-item">
                <SvgIcon name="generate" title="json" size="48"></SvgIcon>
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
  import { useRouter } from 'vue-router';
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

      const wa_window = (title : string, href : string) => {
        invoke('wa_window', { title: title, label: title, url: href });
      }

      return {
        current,
        wa_window,
        homeConf
      }
    }
  });
</script>
