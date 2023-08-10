<template>
  <div class="main-content">
    <el-menu
      :default-active="activeMenu"
      mode="horizontal"
      @select="handleClick"
      :ellipsis="false"
      router
    >
      <el-menu-item>LOGO</el-menu-item>
      <div class="flex-grow" />
      <el-menu-item index="/onlineTool/generate">在线生成</el-menu-item>
      <el-menu-item index="/onlineTool/json">JSON工具</el-menu-item>
      <el-menu-item index="/onlineTool/codec">编解码</el-menu-item>
    </el-menu>
    <div class="menu-content">
      <RouterView />
    </div>
  </div>
</template>
<script lang="ts">
  import { defineComponent, ref, onMounted, watch } from 'vue';
  import { useRouter, useRoute, RouterView } from 'vue-router';
  import { invoke } from "@tauri-apps/api/tauri";
  
  export default defineComponent({
    components: {
    },

    setup() {
      const router = useRouter();
      const route = useRoute();
      const routePath = "/onlineTool";
      const activeMenu = ref<string>("/onlineTool/generate");

      const handleClick = (key: string, keyPath: string[]) => {
        if (key == "logo") {
          return;
        }
        invoke("save_route_active_key", { path: routePath, activeKey: key.substring(key.lastIndexOf("/") + 1)});
        router.push({ path: key, query: route.query});
      }
      
      const refreshActiveKey = async () => {
        const menuActiveKey = await invoke("get_route_active_key", { path: routePath}) as string;
        activeMenu.value = routePath + "/" + (menuActiveKey ? menuActiveKey : "generate");
      }

      const refreshMenu = async () => {
        await refreshActiveKey();
        router.push({ path: activeMenu.value,  query: route.query});
      }

      watch(() => route.query.pathActive,
      async () => {
        refreshActiveKey();
      }, {deep: true});

      onMounted(() => {
        refreshMenu();
      })

      return {
        activeMenu,
        handleClick
      }
    }
  });
</script>