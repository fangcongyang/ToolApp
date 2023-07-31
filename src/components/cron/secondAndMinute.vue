<!-- 秒,分钟 -->
<template>
  <div :val="value_" >
    <el-radio-group v-model="type" class="cron-item">
      <div>
        <el-radio label="1" size="small">每{{ lable }}</el-radio>
      </div>
      <div class="item">
        <el-radio label="2" size="small">周期</el-radio>
        <span style="margin-right: 5Px;">从</span>
        <el-input-number
          @change="type = '2'"
          v-model="cycle.start"
          :min="1"
          :max="cycle.end"
          controls-position="right"
          style="width: 100px"
        ></el-input-number>
        <span style="margin-left: 5px; margin-right: 5px">至</span>
        <el-input-number
          @change="type = '2'"
          v-model="cycle.end"
          :min="cycle.start + 1"
          :max="59"
          controls-position="right"
          style="width: 100px"
        ></el-input-number>
        <span style="margin-left: 5px;">
          {{ lable }}循环执行
        </span>
      </div>
      <div class="item">
        <el-radio label="3" size="small">循环</el-radio>
        <span style="margin-right: 5px">从</span>
        <el-input-number
          @change="type = '3'"
          :min="0"
          :max="59"
          v-model="loop.start"
          controls-position="right"
          style="width: 100px"
        ></el-input-number>
        <span style="margin-left: 5px; margin-right: 5px"
          >{{ lable }}开始，每</span
        >
        <el-input-number
          @change="type = '3'"
          :min="1"
          :max="59"
          v-model="loop.end"
          controls-position="right"
          style="width: 100px"
        ></el-input-number>
        <span style="margin-left: 5px;">
          {{ lable }}执行一次
        </span>
      </div>
      <div style="display: flex; margin-top: 10px">
        <el-radio label="4" size="small">指定</el-radio>
        <el-checkbox-group
          @change="(value: string[]) => {type ='4';}"
          v-model="appoint"
          style="width: 600px"
        >
          <el-row v-for="i in 6" :key="i">
            <el-col :span="2" v-for="j in 10" :key="j">
              <el-checkbox :key="(i - 1) * 10 + (j - 1) + ''" :label="(i - 1) * 10 + (j - 1) + ''">{{
                i - 1 + "" + (j - 1)
              }}</el-checkbox>
            </el-col>
          </el-row>
        </el-checkbox-group>
      </div>
    </el-radio-group>
  </div>
</template>

<script lang="ts">
import {
  defineComponent,
  watch,
  computed,
  ref,
  reactive,
  toRefs,
  onBeforeMount,
} from "vue";

export default defineComponent({
  name: "secondAndMinute",
  props: {
    modelValue: {
      type: String,
      default: "*",
    },
    lable: String,
  },
  setup(props, { emit }) {
    const data = reactive({
      type: "1", // 类型
      cycle: {
        // 周期
        start: 1,
        end: 2,
      },
      loop: {
        // 循环
        start: 0,
        end: 1,
      },
    });
    const appoint = ref<Array<String>>([]);
    const value_ = computed(() => {
      let result = [];
      switch (data.type) {
        case "1": // 每秒
          result.push("*");
          break;
        case "2": // 年期
          result.push(`${data.cycle.start}-${data.cycle.end}`);
          break;
        case "3": // 循环
          result.push(`${data.loop.start}/${data.loop.end}`);
          break;
        case "4": // 指定
          result.push(appoint.value.join(","));
          break;
        default: // 不指定
          result.push("?");
          break;
      }
      emit('update:modelValue', result.join(""));
      return result.join("");
    });

    const updateVal = () => {
      if (!props.modelValue) {
        return;
      }
      let arr;
      if (props.modelValue === "?") {
        data.type = "5";
      } else if (props.modelValue.indexOf("-") !== -1) {
        // 2周期
        arr = props.modelValue.split("-");
        if (arr.length === 2) {
          data.type = "2";
          data.cycle.start = parseInt(arr[0]);
          data.cycle.end = parseInt(arr[1]);
        }
      } else if (props.modelValue.indexOf("/") !== -1) {
        // 3循环
        arr = props.modelValue.split("/");
        if (arr.length === 2) {
          data.type = "3";
          data.loop.start = parseInt(arr[0]);
          data.loop.end = parseInt(arr[1]);
        }
      } else if (props.modelValue.indexOf("*") !== -1) {
        // 1每
        data.type = "1";
      } else {
        // *
        data.type = "4";
        appoint.value = props.modelValue.split(",");
      }
    };

    watch(
      () => props.modelValue,
      () => updateVal()
    );

    onBeforeMount(() => {
      updateVal();
    });

    return {
      ...toRefs(data),
      appoint,
      value_,
    };
  },
});
</script>

<style lang="scss">
.el-checkbox + .el-checkbox {
  margin-left: 10px;
}
</style>
