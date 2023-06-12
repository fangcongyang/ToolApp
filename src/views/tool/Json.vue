<template>
  <div style="display: flex; flex-direction: column; justify-content: space-between; align-items: center; width: 100%;">
    <div style="display: flex; flex-direction: row; width: 100%;">
      <div style="flex: 0.1;"></div>
      <a-tabs style="flex: 0.8;" :tabBarGutter="30" v-model:activeKey="activeKey" @change="tabChange">
        <a-tab-pane key="json" tab="JSON格式化">
          <div style="display: flex; flex-direction: column; justify-items: center; justify-content: space-between; height: 6.7rem;">
            <a-space :size="spaceSize">
              <a-textarea style="width: 9rem;height: 2.5rem;" v-model:value="json.oldValue" placeholder="放入json原文" allow-clear />
            </a-space>
            <div style="display: flex; flex-direction: column; justify-content: space-between; height: 1rem;">
              <div style="display: flex; flex-direction: row; justify-content: space-between; align-items: center;">
                <a-space :size="spaceSize">
                  <span style="font-size: medium; font-weight: bold;">缩进量：</span>
                  <a-input-number v-model:value="json.indent" :min="2" :max="8" placeholder="请输入缩进量"
                    style="width: 1rem;" />
                  <a-radio v-model:checked="json.mark" @click="changeMark">引号</a-radio>
                </a-space>
              </div>
              <div style="display: flex; justify-content: space-between; width: 8rem;">
                <a-button @click="escapeJson('1')">JSON转义</a-button>
                <a-button @click="escapeJson('2')">去除转义</a-button>
                <a-button @click="formatJson">格式化JSON</a-button>
                <a-button @click="compressJson">压缩JSON</a-button>
                <a-button @click="copyCmData">复制结果</a-button>
                <a-button @click="clearCmData">清空</a-button>
              </div>
            </div>
            <div>
              <Codemirror v-model="json.newData" placeholder="Code gose here..." :style="{ height: '3rem',  width: '9rem'}" 
                :autofocus="true"
                :indent-with-tab="true"
                :tabSize="json.indent" :extensions="extensions" />
            </div>
          </div>
        </a-tab-pane>
      </a-tabs>
    </div>
  </div>
</template>
<script lang="ts">
  import { keymap, EditorView } from "@codemirror/view"
  import { Codemirror } from 'vue-codemirror';
  import { json } from "@codemirror/lang-json";
  import { indentWithTab } from "@codemirror/commands";
  import { oneDark } from "@codemirror/theme-one-dark";
  import { defineComponent, ref, reactive, toRefs } from 'vue';
  
  const spaceSize = 8;
  const extensions = [
    json(), 
    oneDark, 
    keymap.of([indentWithTab]), 
    EditorView.editable.of(false),
    ];
  export default defineComponent({
    components: {
      Codemirror
    },
    setup() {
      const state = reactive({
        json: {
          pwdCheckedList: [0, 1, 2],
          indent: 4,
          pwdSize: 10,
          oldValue: "",
          mark: true,
          newData: ""
        },
        uuidData: {
          uuidSize: 1,
          upper: false,
          withSign: false,
        },
        indeterminate: true,
        checkAll: false,
        cmData: ""
      });
      
      const compressJson = () => {
        try {
          let json = JSON.stringify(JSON.parse(state.json.oldValue), null);
          state.json.newData = markHandler(json);
        } catch (error) {
          state.json.newData = "无效的json数据";
        }
      }

      const escapeJson = (ii: string) => {
        if (ii === '1') {
          state.json.oldValue = state.json.oldValue.replace(/\\/g, "\\\\").replace(/\"/g, "\\\"");
        }
        if (ii === '2') {
          state.json.oldValue = state.json.oldValue.replace(/\\\\/g, "\\").replace(/\\\"/g, '\"');
        }
      }

      const formatJson = () => {
        try {
          let json = JSON.stringify(JSON.parse(state.json.oldValue), null, '\t');
          state.json.newData = markHandler(json);
        } catch (error) {
          state.json.newData = "无效的json数据";
        }
      }

      const markHandler = (json: string) => {
        if (!state.json.mark) {
          json = json.replace(/\"(\\[^]|[^\\\"])*\"\s*:?/g, function (match: string) {
                if (/:$/.test(match)) return match.replace(/^\"|\"(?=\s*:$)/g, '');
                else return match;
            });
          }
        return json;
      }

      /**
       * 清除cm数据
       */
      const clearCmData = () => {
        state.json.newData = ""
      }

      const copyCmData = () => {
        navigator.clipboard.writeText(state.json.newData)
        .then(() => {
        })
        .catch((err) => {
        })
      }

      const tabChange = () => {
        state.cmData = "";
      }

      const changeMark = () => {
        state.json.mark = !state.json.mark;
      }

      return {
        ...toRefs(state),
        activeKey: ref('json'),
        spaceSize,
        compressJson,
        extensions,
        clearCmData,
        copyCmData,
        changeMark,
        tabChange,
        formatJson,
        escapeJson
      };
    },
  });
</script>

<style scoped lang="css"> 
</style>
