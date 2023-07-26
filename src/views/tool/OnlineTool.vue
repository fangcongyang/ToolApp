<template>
  <div class="main-content">
    <el-menu
      :default-active="activeMenu"
      mode="horizontal"
      @open="handleClick"
      :ellipsis="false"
      router
    >
      <el-menu-item>LOGO</el-menu-item>
      <div class="flex-grow" />
      <el-menu-item index="/onlineTool/pwdGenerate/pwdGenerate">密码生成</el-menu-item>
      <el-menu-item index="/onlineTool/json/json">JSON工具</el-menu-item>
    </el-menu>
    <div class="menu-content">
      <RouterView />
    </div>
  </div>
</template>
<script lang="ts">
  import { defineComponent, ref, onMounted } from 'vue';
  import { useRouter, useRoute, RouterView } from 'vue-router';
  
  export default defineComponent({
    components: {
    },

    setup() {
      const router = useRouter();
      const route = useRoute();
      const activeMenu = ref<string>("/onlineTool/pwdGenerate/pwdGenerate");

      const handleClick = (key: string, keyPath: string[]) => {
        if (key == "logo") {
          return;
        }
        router.push({ path: key, });
      }

      onMounted(() => {
        const menuActiveKey = route.params.menuKey ? "/onlineTool/" + route.params.menuKey + "/" + route.params.menuKey : activeMenu.value;
        activeMenu.value = menuActiveKey;
        router.push({ path: activeMenu.value, });
      })

      return {
        activeMenu,
        handleClick
      }
    }
  });
</script>