<template>
  <a-layout>
    <a-layout-header style="background-color: #fff;">
      <a-menu v-model:selectedKeys="current" mode="horizontal" @click="handleClick">
        <a-menu-item key="onlineTool" >
          <template #icon>
            <setting-outlined />
          </template>
          在线工具
        </a-menu-item> 
        <a-menu-item key="webSsh" >
          <template #icon>
            <setting-outlined />
          </template>
          ssh
        </a-menu-item>
        <a-sub-menu key="sub1">
          <template #icon>
            <setting-outlined />
          </template>
          <template #title>Navigation Three - Submenu</template>
          <a-menu-item-group title="Item 1">
            <a-menu-item key="setting:1">Option 1</a-menu-item>
            <a-menu-item key="setting:2">Option 2</a-menu-item>
          </a-menu-item-group>
          <a-menu-item-group title="Item 2">
            <a-menu-item key="setting:3">Option 3</a-menu-item>
            <a-menu-item key="setting:4">Option 4</a-menu-item>
          </a-menu-item-group>
        </a-sub-menu>
        <a-sub-menu>
          <template #icon>
            <setting-outlined />
          </template>
          <template #title>视频VIP免费</template>
          <a-menu-item key="movieVip">
            <a @click="wa_window('酉灿VIP', 'https://vip.superso.top/')" target="_blank" rel="noopener noreferrer">
              酉灿VIP
            </a>
          </a-menu-item>
          <a-menu-item key="movieVip">
            <a @click="wa_window('骷髅村', 'https://kuloucun.com/vip/')" target="_blank" rel="noopener noreferrer">
              骷髅村
            </a>
          </a-menu-item>
          <a-menu-item key="movieVip">
            <a @click="wa_window('资源网站', 'https://www.watchtving.com')" target="_blank" rel="noopener noreferrer">
              资源网站
            </a>
          </a-menu-item>
          <a-menu-item key="movieVip">
            <a @click="wa_window('auete', 'https://auete.org/')" target="_blank" rel="noopener noreferrer">
              auete
            </a>
          </a-menu-item>
        </a-sub-menu>
      </a-menu>
    </a-layout-header>
    <a-layout-content>
      <div class="container">
        <h1>Welcome to Tauri!</h1>

        <div class="row">
          <a href="https://vitejs.dev" target="_blank">
            <img src="/vite.svg" class="logo vite" alt="Vite logo" />
          </a>
          <a href="https://tauri.app" target="_blank">
            <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
          </a>
          <a href="https://vuejs.org/" target="_blank">
            <img src="../assets/vue.svg" class="logo vue" alt="Vue logo" />
          </a>
        </div>

        <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>

        <p>
          Recommended IDE setup:
          <a href="https://code.visualstudio.com/" target="_blank">VS Code</a>
          +
          <a href="https://github.com/johnsoncodehk/volar" target="_blank">Volar</a>
          +
          <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank"
            >Tauri</a
          >
          +
          <a href="https://github.com/rust-lang/rust-analyzer" target="_blank"
            >rust-analyzer</a
          >
        </p>

        <Greet />
      </div>
    </a-layout-content>
    <a-layout-footer>Footer</a-layout-footer>
  </a-layout>
</template>

<script lang="ts">
  import Greet from "./components/Greet.vue";
  import { defineComponent, ref } from 'vue';
  import { ToolOutlined, AppstoreOutlined, SettingOutlined } from '@ant-design/icons-vue';
  import type { MenuProps } from 'ant-design-vue';
  import { useRouter } from 'vue-router';
  import { invoke } from "@tauri-apps/api/tauri";
  
  export default defineComponent({
    components: {
      ToolOutlined,
      AppstoreOutlined,
      SettingOutlined,
    },

    setup() {
      const router = useRouter();
      const current = ref<string[]>(['']);
      const handleClick: MenuProps['onClick'] = menuInfo => {
        switch (menuInfo.key.toString()) {
          case "movieVip":
            break;
          default:
            router.push({ path: menuInfo.key.toString(), });
            break;
        }
      };
      const wa_window = (title : string, href : string) => {
        invoke('wa_window', { title: title, label: title, url: href });
      };
      return {
        current,
        handleClick,
        wa_window
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
