<template>
  <div class="part">
    <div class="part-content">
      <el-tabs v-model="activeKey" @tab-change="tabChange">
        <el-tab-pane name="base64Encrypt" label="base64加密">
          <div class="pane-item">
            <el-input type="textarea" :rows="6" v-model="base64EncryptTxt.oldValue" placeholder="被加密数据" clearable />
            <div style="display: flex; flex-direction: row; flex-wrap: wrap; justify-content: space-between; align-items: center; margin-top: 10px;">
              <el-space wrap :size="spaceSize">
                <el-button @click="base64Encrypt">Base64编码</el-button>
                <el-button @click="copyCmData">复制结果</el-button>
                <!-- <el-button @click="compressJson">下载TXT</el-button> -->
                <el-button @click="clearCmData">清空</el-button>
              </el-space>
            </div>
            <div class="pane-part-code">
              <Codemirror v-model="base64EncryptTxt.newData" placeholder="Code gose here..." :style="{ height: '100%',  width: '100%'}" 
                :autofocus="true"
                :indent-with-tab="true"
                :extensions="extensions" />
            </div>
          </div>
        </el-tab-pane>
        <el-tab-pane name="base64Decrypt" label="base64解密">
          <div class="pane-item">
            <el-input type="textarea" :rows="6" v-model="base64DecryptTxt.oldValue" placeholder="加密数据" clearable />
            <div style="display: flex; flex-direction: row; flex-wrap: wrap; justify-content: space-between; align-items: center; margin-top: 10px;">
              <el-space wrap :size="spaceSize">
                <el-button @click="base64Decrypt">Base64解码</el-button>
                <el-button @click="copyCmData">复制结果</el-button>
                <!-- <el-button @click="compressJson">下载TXT</el-button> -->
                <el-button @click="clearCmData">清空</el-button>
              </el-space>
            </div>
            <div class="pane-part-code">
              <Codemirror v-model="base64DecryptTxt.newData" placeholder="Code gose here..." :style="{ height: '100%',  width: '100%'}" 
                :autofocus="true"
                :indent-with-tab="true"
                :extensions="extensions" />
            </div>
          </div>
        </el-tab-pane>
      </el-tabs>
    </div>
  </div>
</template>
<script lang="ts">
  import { keymap, EditorView } from "@codemirror/view"
  import { Codemirror } from 'vue-codemirror';
  import { indentWithTab } from "@codemirror/commands";
  import { oneDark } from "@codemirror/theme-one-dark";
  import { defineComponent, ref, reactive, toRefs, onMounted, watch } from 'vue';
  import { invoke } from "@tauri-apps/api/tauri";
  import { useRoute } from 'vue-router';
  import { encode, decode } from 'js-base64';
  import { writeText } from '@tauri-apps/api/clipboard';
  
  const spaceSize = 4;
  const extensions = [
    oneDark, 
    keymap.of([indentWithTab]), 
    EditorView.editable.of(false),
  ];
  export default defineComponent({
    name: 'Codec',
    components: {
      Codemirror
    },
    setup() {
      const state = reactive({
        base64EncryptTxt: {
          oldValue: "",
          newData: "",
        },
        base64DecryptTxt: {
          oldValue: "",
          newData: "",
        },
      });
      const activeKey = ref<string>("base64Encrypt");
      const routePath = "/onlineTool/codec";
      const route = useRoute();
      
      const base64Decrypt = () => {
        try {
          state.base64DecryptTxt.newData = decode(state.base64DecryptTxt.oldValue);
        } catch (error) {
          state.base64DecryptTxt.newData = "base64解密失败";
        }
      }

      const base64Encrypt = () => {
        try {
          state.base64EncryptTxt.newData = encode(state.base64EncryptTxt.oldValue);
        } catch (error) {
          state.base64EncryptTxt.newData = "base64加密失败";
        }
      }

      /**
       * 清除cm数据
       */
      const clearCmData = () => {
        state.base64EncryptTxt.newData = ""
      }

      const copyCmData = async () => {
        let data = "";
        switch (activeKey.value) {
          case "base64Encrypt":
            data = state.base64EncryptTxt.newData;
            break;
          case "base64Decrypt":
            data = state.base64DecryptTxt.newData;
            break;
        }
        await writeText(data)
      }

      const tabChange = () => {
        switch (activeKey.value) {
          case "base64Encrypt":
            state.base64EncryptTxt.newData = "";
            break;
          case "base64Decrypt":
            state.base64DecryptTxt.newData = "";
            break;
        }
      }

      const refreshMenu = async () => {
        let ak = await invoke("get_route_active_key", { path: routePath});
        if (ak) {
          activeKey.value = ak as string;
        }
      }

      watch(() => route.query.pathActive,
      () => {
        refreshMenu();
      }, {deep: true});

      onMounted(() => {
        refreshMenu();
      })

      return {
        ...toRefs(state),
        spaceSize,
        base64Decrypt,
        extensions,
        clearCmData,
        copyCmData,
        tabChange,
        base64Encrypt,
        activeKey
      };
    },
  });
</script>

<style lang="css"> 
.ant-tabs-content {
    height: 100% !important;
  }
</style>
