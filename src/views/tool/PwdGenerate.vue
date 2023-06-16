<template>
  <div style="display: flex; flex-direction: column; justify-content: space-between; align-items: center; width: 100%; height: 8rem;">
    <div style="display: flex; flex-direction: row; width: 100%;">
      <div style="flex: 0.1;"></div>
      <a-tabs style="flex: 0.8;" :tabBarGutter="30" v-model:activeKey="activeKey" @change="tabChange">
        <a-tab-pane key="pwdGenerate" tab="密码生成器">
          <div style="display: flex; flex-direction: column; justify-items: center; justify-content: space-between; height: 2rem;">
            <a-space :size="spaceSize">
              <span style=" font-weight: bold;">所用字符：</span>
              <a-checkbox-group v-model:value="pwdData.pwdCheckedList" :options="pwdOptions" />
            </a-space>
            <a-space :size="spaceSize">
              <span style="font-weight: bold;">密码长度：</span>
              <a-input-number v-model:value="pwdData.pwdLength" :min="6" :max="50" placeholder="请输入密码长度"
                style="width: 9rem; " />
            </a-space>
            <a-space :size="spaceSize">
              <span style="font-weight: bold;">密码数量：</span>
              <a-input-number v-model:value="pwdData.pwdSize" :min="1" :max="50" placeholder="请输入密码数量"
              style="width: 9rem; "/>
            </a-space>
            <div style="display: flex;">
              <div style="display: flex; justify-content: space-between; width: 3.5rem;">
                <a-button @click="generatePwd">生成密码</a-button>
                <a-button @click="copyCmData">复制结果</a-button>
                <a-button @click="clearCmData">清空</a-button>
              </div>
            </div>
          </div>
        </a-tab-pane>
        <a-tab-pane key="uuid" tab="UUID在线生成">
          <div style="display: flex; flex-direction: column; justify-items: center; justify-content: space-between; height: 1.8rem;">
            <a-space :size="spaceSize">
              <span style="font-weight: bold;">UUID大写：</span>
              <a-radio-group v-model:value="uuidData.upper">
                <a-radio :value="true">是</a-radio>
                <a-radio :value="false">否</a-radio>
              </a-radio-group>
              <span style="font-weight: bold;">UUID带符号：</span>
              <a-radio-group v-model:value="uuidData.withSign">
                <a-radio :value="true">是</a-radio>
                <a-radio :value="false">否</a-radio>
              </a-radio-group>
            </a-space>
            <a-space :size="spaceSize">
              <span style="font-weight: bold;">生成UUID个数：</span>
              <a-input-number v-model:value="uuidData.uuidSize" :min="1" :max="50" placeholder="请输入生成UUID个数"
                style="width: 8.5rem;" />
            </a-space>
            
            <div style="display: flex;">
              <div style="display: flex; justify-content: space-between; width: 3.5rem;">
                <a-button @click="generateUUID">生成UUID</a-button>
                <a-button @click="copyCmData">复制结果</a-button>
                <a-button @click="clearCmData">清空</a-button>
              </div> 
            </div>
          </div>
        </a-tab-pane>
      </a-tabs>
    </div>
    <div style="display: flex; width: 100%; height: 5rem;">
      <div style="flex: 0.1;"></div>
      <div style="flex: 0.8;">
        <Codemirror v-model="cmData" placeholder="Code gose here..." :style="{ height: '5rem',  width: '10.1rem'}"
        :disabled="true"  
        :autofocus="true"
          :indent-with-tab="true"
          :tabSize="4" :extensions="extensions" />
      </div>
    </div>
  </div>
</template>
<script lang="ts">
  import { Codemirror } from 'vue-codemirror';
  import { javascript } from "@codemirror/lang-javascript";
  import { oneDark } from "@codemirror/theme-one-dark";
  import { defineComponent, ref, reactive, toRefs } from 'vue';

  const pwdOptions = [
      {label: '数字', value: 0, chooseValue: "0123456789"}, 
      {label: '小写字母', value: 1, chooseValue: "abcdefghijklmnolpqrstuvwxyz"}, 
      {label: '大写字母', value: 2, chooseValue: "ABCDEFGHIJKLMNOLPQRSTUVWXYZ"}, 
      {label: '标点符号', value: 3, chooseValue: "!@#%&*_=+"}, 
      {label: '字符不重复', value: 4}];
  const spaceSize = 8;
  const extensions = [javascript(), oneDark];
  export default defineComponent({
    components: {
      Codemirror,
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
        cmData: ""
      });
      
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
        activeKey: ref('pwdGenerate'),
        pwdOptions,
        spaceSize,
        generatePwd,
        extensions,
        clearCmData,
        copyCmData,
        generateUUID,
        tabChange
      };
    },
  });
</script>

<style scoped lang="css"> 
</style>
