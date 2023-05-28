<template>
  <div class="ssh-container" ref="terminal"></div>
</template>
  
<script setup lang="ts">
  import { ref, reactive, onMounted, onBeforeUnmount } from 'vue';
  import { Terminal, ITerminalOptions, ITerminalInitOnlyOptions } from 'xterm'
  import { FitAddon } from 'xterm-addon-fit';
  import 'xterm/css/xterm.css';
  import { debounce } from 'lodash'

  const sshId = crypto.randomUUID();

  const message = (mesType: string, config: any, data: any) =>
    JSON.stringify({
      sshId: sshId,
      mesType: mesType,
      data: data,
      config: config
    })

  const packStdin = (data: any) =>
    ({
      operType: '1',
      command: data
    })

  const packResize = (cols: any, rows: any) =>
    ({
      operType: '2',
      cols: cols,
      rows: rows
    })

  const option: ITerminalOptions & ITerminalInitOnlyOptions = {
    lineHeight: 1.0,
    cursorBlink: true,
    cursorStyle: 'block', // 光标样式 'block' | 'underline' | 'bar'
    fontSize: 16,
    fontFamily: "Monaco, Menlo, Consolas, 'Courier New', monospace",
    theme: {
      background: '#181d28'
    },
    cols: 30 // 初始化的时候不要设置fit，设置col为较小值（最小为可展示initText初始文字即可）方便屏幕缩放
  }

  const props = defineProps({
    serverInfo: {
      type: Object,
      required: true
    }
  });

  const initText = '连接中...\r\n';
  const terminal = ref<HTMLDivElement | null>(null);
  let term = ref<Terminal | null>(null);
  const fitAddon = new FitAddon();
  const ws = new WebSocket('ws://127.0.0.1:59999/wssh');

  const state = reactive({
    first: true,
  });
  
  const initTerm = () => {
    term.value = new Terminal(option);
    term.value.open(terminal.value!)
    term.value.loadAddon(fitAddon)
    // this.fitAddon.fit() // 初始化的时候不要使用fit
    setTimeout(() => {
      fitAddon.fit()
    }, 500); // 必须延时处理
  };

  const isWsOpen = () => {
    return ws && ws.readyState === 1
  };

  const onTerminalKeyPress = () => {
    term.value!.onData((data: any) => {
      isWsOpen() && ws.send(message("2", null, packStdin(data)))
    })
  };
  
  
  const onResize = debounce(() => 
    fitAddon.fit(), 800
  );
  
  // resize 相关
  const resizeRemoteTerminal = () => {
    const { cols, rows } = term.value!;
    // 调整后端终端大小 使后端与前端终端大小一致
    isWsOpen() && ws.send(message("2", null, packResize(cols, rows)))
  };

  // socket
  const initSocket = () => {
    term.value!.write(initText);
    const localTerm = term.value!;
    ws.onopen = () => {
      ws.send(message("1", props.serverInfo, null));
    }
    ws.onclose = () => {
      localTerm.write("\r\n连接关闭...\r\n");
      // state.term.write("未连接， 3秒后重连...\r\n");
      // setTimeout(() => {
      //   initSocket();
      // }, 3000)
    }

    ws.onerror = () => {
      console.log('websoket连接失败，请刷新！')
    }
    
    ws.onmessage = (res: { data: any; }) => {
      const data = res.data
      // console.log("receive: " + data)
      // 第一次连接成功将 initText 清空
      if (state.first) {
        state.first = false
        term.value!.reset()
        term.value!.element && term.value!.focus()
        resizeRemoteTerminal()
      }
      term.value!.write(data)
    }
  };

  const onTerminalResize = () => {
    window.addEventListener("resize", onResize);
    term.value!.onResize(resizeRemoteTerminal)
  }

  const removeResizeListener = () => {
    window.removeEventListener("resize", onResize);
  }

  defineExpose({ onResize });

  onMounted(() => {
    initTerm();
    initSocket();

    onTerminalResize();
    onTerminalKeyPress();
  });

  onBeforeUnmount(() => {
    removeResizeListener();
    ws.close(3066, "客户端主动断开");
  })
</script>
<style lang="css">

  html, body {
    width: 100%;
    margin: 0;
    padding: 0;
  }
  
  .ssh-container {
    overflow: hidden;
    width: 100%;
    height: 100%;
    border-radius: 4px;
    background: rgb(24, 29, 40);
    padding: 0px;
    color: rgb(255, 255, 255);
  }

  .ssh-container .xterm-scroll-area::-webkit-scrollbar-thumb {
      background-color: #b7c4d1;
      /* 滚动条的背景颜色 */
  }
</style>
  
  
  