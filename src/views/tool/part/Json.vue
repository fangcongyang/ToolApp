<template>
  <div class="part">
    <div class="part-content">
      <el-tabs v-model="activeKey" @tab-change="tabChange">
        <el-tab-pane name="json" label="JSON格式化">
          <div class="pane-item">
            <el-input type="textarea" :rows="6" v-model="json.oldValue" placeholder="放入json原文" clearable />
            <div style="display: flex; flex-direction: row; flex-wrap: wrap; justify-content: space-between; align-items: center; margin-top: 10px;">
              <el-space wrap :size="spaceSize">
                <span style="font-weight: bold;">缩进量：</span>
                <el-input-number v-model="json.indent" :min="2" :max="8" placeholder="请输入缩进量"
                  style="width: 1rem;" />
                <el-radio v-model:checked="json.mark" @click="changeMark">引号</el-radio>
                <el-button @click="escapeJson('1')">JSON转义</el-button>
                <el-button @click="escapeJson('2')">去除转义</el-button>
                <el-button @click="formatJson">格式化JSON</el-button>
                <el-button @click="compressJson">压缩JSON</el-button>
                <el-button @click="copyCmData">复制结果</el-button>
                <el-button @click="clearCmData">清空</el-button>
              </el-space>
            </div>
            <div class="pane-part-code">
              <Codemirror v-model="json.newData" placeholder="Code gose here..." :style="{ height: '100%',  width: '100%'}" 
                :autofocus="true"
                :indent-with-tab="true"
                :tabSize="json.indent" :extensions="extensions" />
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
  import { json } from "@codemirror/lang-json";
  import { indentWithTab } from "@codemirror/commands";
  import { oneDark } from "@codemirror/theme-one-dark";
  import { defineComponent, ref, reactive, toRefs } from 'vue';
  
  const spaceSize = 4;
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
        cmData: "",
        activeKey: "json",
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

<style lang="css"> 
.ant-tabs-content {
    height: 100% !important;
  }
</style>
