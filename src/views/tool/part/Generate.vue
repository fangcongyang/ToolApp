<template>
  <div class="part">
    <div class="part-content-one">
      <el-tabs v-model="activeKey" @tab-change="tabChange">
        <el-tab-pane name="pwdGenerate" label="密码生成器">
          <div class="pane-item">
            <el-space :size="spaceSize">
              <div class="row">
                <div class="row-title">
                  <span>所用字符：</span>
                </div>
              </div>
              <el-checkbox-group v-model="pwdData.pwdCheckedList">
                <el-checkbox v-for="pwdOption in pwdOptions" :key="pwdOption.label" :label="pwdOption.value">
                  {{pwdOption.label}}
                </el-checkbox>  
              </el-checkbox-group>
            </el-space>
            <div class="row">
              <div class="row-title">
                <span>密码长度：</span>
              </div>
              <el-input-number v-model="pwdData.pwdLength" :min="6" :max="50" placeholder="请输入密码长度" controls-position="right" />
            </div>
            <div class="row">
              <div class="row-title">
                <span>密码数量：</span>
              </div>
              <el-input-number v-model="pwdData.pwdSize" :min="1" :max="50" placeholder="请输入密码数量" controls-position="right" />
            </div>
            <div style="display: flex;">
              <el-space :size="spaceSize">
                <el-button @click="generatePwd">生成密码</el-button>
                <el-button @click="copyCmData">复制结果</el-button>
                <el-button @click="clearCmData">清空</el-button>
              </el-space>
            </div>
          </div>
        </el-tab-pane>
        <el-tab-pane name="uuid" label="UUID在线生成">
          <div class="pane-item">
            <el-space :size="spaceSize">
              <div class="row">
                <div class="row-title">
                  <span>UUID大写：</span>
                </div>
                <el-radio-group v-model="uuidData.upper">
                  <el-radio :label="true">是</el-radio>
                  <el-radio :label="false">否</el-radio>
                </el-radio-group>
              </div>
              <div class="row">
                <div class="row-title">
                  <span>UUID带符号：</span>
                </div>
                <el-radio-group v-model="uuidData.withSign">
                  <el-radio :label="true">是</el-radio>
                  <el-radio :label="false">否</el-radio>
                </el-radio-group>
              </div>
            </el-space>
            <div class="row">
              <div class="row-title">
                <span>生成UUID个数：</span>
              </div>
              <el-input-number v-model="uuidData.uuidSize" :min="1" :max="50" controls-position="right" placeholder="请输入生成UUID个数" />
            </div>
            
            <div style="display: flex;">
              <el-space :size="spaceSize">
                <el-button @click="generateUUID">生成UUID</el-button>
                <el-button @click="copyCmData">复制结果</el-button>
                <el-button @click="clearCmData">清空</el-button>
              </el-space>
            </div>
          </div>
        </el-tab-pane>
        <el-tab-pane name="cron" label="cron在线生成">
          <div class="pane-cron">
            <cronShow v-model="execTime"></cronShow>
          </div>
        </el-tab-pane>
      </el-tabs>
    </div>
    <div class="part-code" v-if="activeKey != 'cron'">
      <Codemirror v-model="cmData" placeholder="Code gose here..." :style="{ height: '100%',  width: '100%'}"
        :disabled="true"  
        :autofocus="true"
        :indent-with-tab="true"
        :tabSize="4" :extensions="extensions" />
    </div>
  </div>
</template>
<script lang="ts">
  import { defineComponent, ref, reactive, toRefs } from 'vue';
  import { Codemirror } from 'vue-codemirror';
  import { javascript } from "@codemirror/lang-javascript";
  import { oneDark } from "@codemirror/theme-one-dark";
  import cronShow from '@/components/cronShow.vue';

  const pwdOptions = [
      {label: '数字', value: 0, chooseValue: "0123456789"}, 
      {label: '小写字母', value: 1, chooseValue: "abcdefghijklmnolpqrstuvwxyz"}, 
      {label: '大写字母', value: 2, chooseValue: "ABCDEFGHIJKLMNOLPQRSTUVWXYZ"}, 
      {label: '标点符号', value: 3, chooseValue: "!@#%&*_=+"}, 
      {label: '字符不重复', value: 4}];
  const extensions = [javascript(), oneDark];
  export default defineComponent({
    components: {
      Codemirror,
      cronShow,
    },
    setup() {
      const state = reactive({
        pwdData: {
          pwdCheckedList: [0, 1, 2],
          pwdLength: 6,
          pwdSize: 10,
        },
        uuidData: {
          uuidSize: 1,
          upper: false,
          withSign: false,
        },
        indeterminate: true,
        checkAll: false,
        cmData: "",
        spaceSize: 4,
      });
      const activeKey = ref<string>("pwdGenerate");
      const execTime = ref<String>("");
      
      /**
       * 生成随机密码
       */
      const generatePwd = () => {
        let pwdStr = "";
        const pwdData = state.pwdData;
        pwdOptions.filter(item => pwdData.pwdCheckedList.includes(item.value)).map(item1 => pwdStr += item1.chooseValue);
        if (!pwdStr) {
          return;
        }
        // 不重复获取字符 
        let repeat = false;
        if (pwdData.pwdCheckedList.includes(4)) {
          repeat = true;
        }
        let pwdArr: Array<String> = [];
        let pwd = "";
        let i, j;
        for (let index = 0; index < pwdData.pwdSize; index++) {
          j = 0; 
          pwd = "";
          while (j < pwdData.pwdLength) {
            i = Math.round(Math.random() * pwdStr.length)
            if (j == pwdStr.length) {
              return;
            }
            if (repeat && !pwd.includes(pwdStr.charAt(i))) {
              pwd += pwdStr.charAt(i);
              j++;
            } else if (!repeat) {
              pwd += pwdStr.charAt(i);
              j++;
            }
          }
          pwdArr.push(pwd);
        }
        state.cmData = pwdArr.join("\r\n");
      }

      /**
       * 清除cm数据
       */
      const clearCmData = () => {
        state.cmData = ""
      }

      const copyCmData = () => {
        navigator.clipboard.writeText(state.cmData)
        .then(() => {
        })
        .catch((err) => {
        })
      }

      /**
       * 生成uuid
       */
      const generateUUID = () => {
        let uuidArr = [];
        let uuid;
        for (let index = 0; index < state.uuidData.uuidSize; index++) {
          uuid = crypto.randomUUID();
          if (state.uuidData.upper) {
            uuid = uuid.toUpperCase();
          }
          if (!state.uuidData.withSign) {
            uuid = uuid.replaceAll("-", "");
          }
          uuidArr.push(uuid);
        }
        state.cmData = uuidArr.join("\r\n");
      }

      const tabChange = () => {
        state.cmData = "";
      }

      return {
        ...toRefs(state),
        activeKey,
        pwdOptions,
        generatePwd,
        extensions,
        clearCmData,
        copyCmData,
        generateUUID,
        tabChange,
        execTime
      };
    },
  });
</script>
