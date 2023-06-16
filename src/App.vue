<template>
  <div style="height: 100%; display: flex; flex-direction: column;">
    <div data-tauri-drag-region style="display: flex; flex-direction: row-reverse; height: 40px; background-color: #F0F2F5;">
      <WinTool :minimizable="true" :maximizable="true" :closable="true"/>
    </div>
    <div style="display: flex;flex: 1; flex-direction: column; border-radius: 30px;">
      <RouterView />
    </div>
  </div>
</template>

<script lang="ts">
  import { defineComponent } from 'vue';
  import WinTool from "./components/WinTool.vue";
  import { RouterView } from 'vue-router';
  
  export default defineComponent({
    components: {
      WinTool
    },
    setup() {

    }
  })
</script>
