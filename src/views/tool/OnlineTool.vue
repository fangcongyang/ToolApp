<template>
  <a-layout>
    <a-layout-header :style="{display: 'flex', background: '#021529', padding: 0}">
      <div style="flex: calc(1 / 10);"></div>
      <a-menu style="flex: calc(4/5);" theme="dark" v-model:selectedKeys="current" mode="horizontal" @click="handleClick">
        <a-menu-item key="/" >
          <template #icon>
            <HomeOutlined />
          </template>
          首页
        </a-menu-item>
        <a-menu-item key="/onlineTool/pwdGenerate" >
          密码生成
        </a-menu-item>
        <a-menu-item key="/onlineTool/json" >
          JSON工具
        </a-menu-item>
      </a-menu>
    </a-layout-header>
    <a-layout-content>
      <RouterView></RouterView>
    </a-layout-content>
    <a-layout-footer></a-layout-footer>
  </a-layout>
</template>
<script lang="ts">
  import { defineComponent, ref } from 'vue';
  import { HomeOutlined } from '@ant-design/icons-vue';
  import type { MenuProps } from 'ant-design-vue';
  import { useRouter, RouterView } from 'vue-router';
  
  export default defineComponent({
    components: {
      HomeOutlined
    },

    setup() {
      const router = useRouter();
      const current = ref<string[]>(['/onlineTool/pwdGenerate']);
      const handleClick: MenuProps['onClick'] = menuInfo => {
        router.push({ path: menuInfo.key.toString(), });
      };
      return {
        current,
        handleClick
      }
    }
  });
</script>

<style scoped lang="css"> 
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
