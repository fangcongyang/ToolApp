<template>
  <div class="cron" :val="value_">
    <el-tabs v-model="activeName">
      <el-tab-pane label="秒" name="s">
        <second-and-minute v-model="sVal" lable="秒"></second-and-minute>
      </el-tab-pane>
      <el-tab-pane label="分" name="m">
        <second-and-minute v-model="mVal" lable="分"></second-and-minute>
      </el-tab-pane>
      <el-tab-pane label="时" name="h">
        <hour v-model="hVal" lable="时"></hour>
      </el-tab-pane>
      <el-tab-pane label="日" name="d">
        <day v-model="dVal" lable="日"></day>
      </el-tab-pane>
      <el-tab-pane label="月" name="month">
        <month v-model="monthVal" lable="月"></month>
      </el-tab-pane>
      <el-tab-pane label="周" name="week">
        <week v-model="weekVal" lable="周"></week>
      </el-tab-pane>
      <el-tab-pane label="年" name="year">
        <year v-model="yearVal" lable="年"></year>
      </el-tab-pane>
    </el-tabs>
    <div>
      <span style="font-weight: 800">最近运行时间: </span>
      <div style="height: 100px; margin-top: 10px">
        <span
          style="display: block"
          v-for="recentExecuteTime in recentExecuteTimes"
          >{{ recentExecuteTime }}</span
        >
      </div>
    </div>
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
} from "vue";
import SecondAndMinute from "./cron/SecondAndMinute.vue";
import hour from "./cron/hour.vue";
import day from "./cron/day.vue";
import month from "./cron/month.vue";
import week from "./cron/week.vue";
import year from "./cron/year.vue";
import { invoke } from "@tauri-apps/api";
import { ElMessage } from "element-plus";
import _ from "lodash";

export default defineComponent({
  name: "cron",
  components: {
    SecondAndMinute,
    hour,
    day,
    month,
    week,
    year,
  },
  props: {
    modelValue: {
      type: String,
      default: "",
    },
  },
  setup(props, { emit }) {
    const cronData = reactive({
      activeName: "s",
      sVal: "*",
      mVal: "*",
      hVal: "*",
      dVal: "*",
      monthVal: "*",
      weekVal: "?",
      yearVal: "",
    });
    const recentExecuteTimes = ref<Array<string>>([]);
    const value_ = computed(() => {
      if (!cronData.dVal && !cronData.weekVal) {
        return "";
      }
      if (cronData.dVal === "?" && cronData.weekVal === "?") {
        if (cronData.activeName === "d") { 
          cronData.weekVal = "*";
        }
        if (cronData.activeName === "week") { 
          cronData.dVal = "*";
        }
      }
      if (cronData.dVal !== "?" && cronData.weekVal !== "?") {
        if (cronData.activeName === "d") { 
          cronData.weekVal = "?";
        }
        if (cronData.activeName === "week") { 
          cronData.dVal = "?";
        }
      }
      let v = `${cronData.sVal} ${cronData.mVal} ${cronData.hVal} ${cronData.dVal} ${cronData.monthVal} ${cronData.weekVal} ${cronData.yearVal}`;
      if (v !== props.modelValue) {
        executeTimes(v);
      }
      return v;
    });
    const executeTimes = _.debounce(async (v: string) => {
      recentExecuteTimes.value = await invoke("next_trigger_time", {
        expression: v,
      });
      emit('update:modelValue', v);
      if (recentExecuteTimes.value.length == 0) {
        ElMessage.error("表达式错误");
      }
    }, 500)
    const updateVal = async () => {
      if (!props.modelValue) {
        return;
      }
      let arrays = props.modelValue.split(" ");
      cronData.sVal = arrays[0];
      cronData.mVal = arrays[1];
      cronData.hVal = arrays[2];
      cronData.dVal = arrays[3];
      cronData.monthVal = arrays[4];
      cronData.weekVal = arrays[5];
      cronData.yearVal = arrays[6];
      recentExecuteTimes.value = await invoke("next_trigger_time", {
        expression: props.modelValue,
      });
    };

    watch(
      () => props.modelValue,
      () => {
        updateVal()
      }
    );
    
    return {
      ...toRefs(cronData),
      recentExecuteTimes,
      value_,
    };
  },
});
</script>

<style lang="css">
.cron {
  text-align: left;
  padding: 10px;
  background: #fff;
  border: 1px solid #dcdfe6;
  box-shadow: 0 2px 4px 0 rgba(0, 0, 0, 0.12), 0 0 6px 0 rgba(0, 0, 0, 0.04);
}
</style>
