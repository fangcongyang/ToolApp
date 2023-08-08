<template>
  <div class="frame">
    <span
      class="top"
      @click="handleAlwaysTop"
    >
      <SvgIcon
        name="wintool-ontop"
        :title="data.isAlwaysOnTop ? '取消置顶' : '置顶'"
        :style-var="{ width: '8px', height: '14px' }"
        :color="data.isAlwaysOnTop ? '#555555' : '#ffffff'"
      ></SvgIcon>
    </span>
    <span v-if="minimizable" class="min" @click="handleWinMin">
      <SvgIcon
        name="wintool-min"
        title="最小化"
        :style-var="{ width: '8px', height: '14px' }"
        color="#ffffff"
      ></SvgIcon>
    </span>
    <span
      v-if="maximizable && data.isResizable"
      class="max"
      @click="handleWinMax2Min"
    >
      <SvgIcon
        name="wintool-max"
        :title="data.isMaximized ? '还原' : '最大化'"
        :style-var="{ width: '8px', height: '14px' }"
        color="#ffffff"
      ></SvgIcon>
    </span>
    <span v-if="closable" class="close" @click="handleWinClose">
      <SvgIcon
        title="关闭"
        name="wintool-close"
        :style-var="{ width: '8px', height: '14px' }"
        color="#ffffff"
      ></SvgIcon>
    </span>
  </div>
</template>
<script lang="ts" setup>
import { onMounted, reactive } from "vue";
import { appWindow, LogicalSize, PhysicalSize } from "@tauri-apps/api/window";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { useCoreStore } from "@/store";
import { storeToRefs } from "pinia";
import SvgIcon from "@/components/SvgIcon.vue";

const coreStore = useCoreStore();
const { systemConf } = storeToRefs(coreStore);

const data = reactive({
  isMaximized: false,
  isResizable: true,
  isAlwaysOnTop: false,
  fullscreen: false,
});

const props = defineProps({
  minimizable: {
    type: Boolean,
    required: true,
  },
  maximizable: {
    type: Boolean,
    required: true,
  },
  closable: {
    type: Boolean,
    required: false,
    default: true,
  },
});

onMounted(async () => {
  data.isMaximized = await appWindow.isMaximized();
  data.isResizable = await appWindow.isResizable();
  listen("tauri://resize", async () => {
    data.isMaximized = await appWindow.isMaximized();
  });
});

// 最小化
const handleAlwaysTop = async () => {
  data.isAlwaysOnTop = !data.isAlwaysOnTop;
  await appWindow.setAlwaysOnTop(data.isAlwaysOnTop);
};

// 最小化
const handleWinMin = async () => {
  await appWindow.minimize();
};
// 最大化/还原
const handleWinMax2Min = async () => {
  const resizable = await appWindow.isResizable();
  if (!resizable) return;
  await appWindow.setFullscreen(!data.fullscreen);
  if (data.fullscreen) {
    const physicalSize: PhysicalSize = await appWindow.innerSize();
    if ((physicalSize.width) != systemConf.value.mainWidth) {
      await appWindow.setSize(
        new LogicalSize(systemConf.value.mainWidth, systemConf.value.mainHeight)
      );
    }
  }

  data.fullscreen = !data.fullscreen;
};
// 关闭
const handleWinClose = async () => {
  if(appWindow.label.indexOf('main') > -1) {
    await invoke("exist_app", {});
  } else {
      switch (appWindow.label) {
          case 'webssh':
              await invoke("close_webssh", {});
              break;
          default:
              break;    
      }
      await appWindow.close()
  }
};
</script>
