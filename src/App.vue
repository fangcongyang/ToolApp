<template>
  <el-config-provider :locale="local">
    <div id="main" :class="systemConf.theme">
      <div class="main-body">
        <WinTool :minimizable="true" :maximizable="true" :closable="true"/>
        <RouterView />
      </div>
    </div>
  </el-config-provider>
</template>

<script lang="ts">
  import { defineComponent, onBeforeMount, watch, ref } from 'vue';
  import { ElConfigProvider } from "element-plus";
  import WinTool from "./components/WinTool.vue";
  import { RouterView } from 'vue-router';
  import { useCoreStore } from "@/store";
  import { storeToRefs } from "pinia";
  import { useDark, useToggle } from "@vueuse/core";
  import zhCn from 'element-plus/es/locale/lang/zh-cn'

  export default defineComponent({
    components: {
      WinTool,
      ElConfigProvider,
    },
    setup() {
      const coreStore = useCoreStore();
      const { getSystemConf } = coreStore;
      const { systemConf } = storeToRefs(coreStore);

      const local = ref(zhCn);
      const isDark = useDark();
      const toggleDark = useToggle(isDark);

      const initTheme = () => {
        if (systemConf.value.theme == "theme-dark") {
          isDark.value = false;
        } else {
          isDark.value = true;
        }
        toggleDark();
      }
      
      onBeforeMount(async () => {
        await getSystemConf();
        initTheme();
      })

      watch(() => systemConf.value.theme, 
      () => {
        initTheme();
      });

      return {
        local,
        systemConf,
      }
    },
  })
</script>
<style lang="scss">
#main {
  // overflow: hidden;
  display: flex;
  justify-content: space-between;
  align-items: flex-start;

  .main-body {
    flex: 1;
    height: 100vh;
    width: 100%;
    display: flex;
    justify-content: flex-start;
    align-items: flex-start;
    flex-direction: column;
  }
}
</style>
