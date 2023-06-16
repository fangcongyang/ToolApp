<!-- 右上角操作按钮 -->
<template>
    <div class="tauri-win-btn">
        <slot />
        <div class="wbtn">
            <span :title="data.isAlwaysOnTop ? '取消置顶' : '置顶'" @click="handleAlwaysTop">
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 18 18" fill="none">
                    <path :style="{fill: data.isAlwaysOnTop ? '#22B339' : ''}" d="M12.8042,2.575l-1.7967,2.63511l3.2593,5.21489h-4.8418v4.575h-0.85v-4.575h-4.84181l3.25931,-5.21489l-1.79666,-2.63511zM7.8604,5.42525l-2.59359,4.14975h7.46639l-2.7407,-4.38511l1.2033,-1.76489h-4.39164l1.20334,1.76489z"></path>
                </svg>            
            </span>
        </div>
        <div class="wbtn">
            <span v-if="minimizable" title="最小化" @click="handleWinMin">
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 18 18" fill="none">
                    <path d="M4.5,8.575h9v0.85h-9z"></path>
                </svg>
            </span>
        </div>
        <div class="wbtn">
            <span v-if="maximizable && data.isResizable" :title="data.isMaximized ? '还原' : '最大化'" @click="handleWinMax2Min">
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 18 18" fill="none">
                    <path d="M13.925,4.075v9.85h-9.85v-9.85zM4.925,13.075h8.15v-8.15h-8.15z"></path>
                </svg>
            </span>
        </div>
        <div class="wbtn">
            <span v-if="closable" title="关闭" @click="handleWinClose">
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 18 18" fill="none">
                    <path style="opacity:1;" d="M4.19948,4.80052l0.60104,-0.60104l4.19948,4.19948l4.1995,-4.19948l0.601,0.60104l-4.19946,4.19948l4.19946,4.1995l-0.601,0.601l-4.1995,-4.19946l-4.19948,4.19946l-0.60104,-0.601l4.19948,-4.1995z"></path>
                </svg>
            </span>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { onMounted, reactive } from 'vue'
import { appWindow } from '@tauri-apps/api/window'
import { listen } from '@tauri-apps/api/event'
import { exit } from '@tauri-apps/api/process'
  import { invoke } from "@tauri-apps/api/tauri";

const data = reactive({
    isMaximized: false,
    isResizable: true,
    isAlwaysOnTop: false,
})

const props = defineProps({
    minimizable: {
        type: Boolean,
        required: true
    },
    maximizable: {
        type: Boolean,
        required: true
    },
    closable: {
        type: Boolean,
        required: false,
        default: true
    }
});

onMounted(async() => {
    data.isMaximized = await appWindow.isMaximized()
    data.isResizable = await appWindow.isResizable()
    listen('tauri://resize', async() => {
        data.isMaximized = await appWindow.isMaximized()
    })
})

// 最小化
const handleAlwaysTop = async() => {
    data.isAlwaysOnTop = !data.isAlwaysOnTop
    await appWindow.setAlwaysOnTop(data.isAlwaysOnTop)
}

// 最小化
const handleWinMin = async() => {
    await appWindow.minimize()
}
// 最大化/还原
const handleWinMax2Min = async() => {
    const resizable = await appWindow.isResizable()
    if(!resizable) return
    await appWindow.toggleMaximize()
}
// 关闭
const handleWinClose = async() => {
    if(appWindow.label.indexOf('main') > -1) {
        await exit(0);
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
}
</script>
<style type="text/css" scoped>
    svg {
        fill: #000;
        padding: 3px;
    }

    path {
        opacity:1;
    }

    svg:hover {
        background-color: #E1E1E1;
    }
</style>
<style type="css">
    .tauri-win-btn {
        height: 38px;
        display: flex;
        flex-direction: row;
        width: 2rem;
        align-items: center;
    }

    .wbtn {
        height: 18px;
        flex: 1; 
        cursor: pointer;
    }
</style>