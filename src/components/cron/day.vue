<template>
  <div :val="value_">
    <el-radio-group v-model="type" class="cron-item">
      <div>
        <el-radio label="1">每日</el-radio>
      </div>
      <div class="item">
        <el-radio label="5">不指定</el-radio>
      </div>
      <div class="item">
        <el-radio label="2" size="small">周期</el-radio>
        <span style="margin-right: 5px">从</span>
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
          :max="31"
          controls-position="right"
          style="width: 100px"
        ></el-input-number>
        <span style="margin-left: 5px;">
          日循环执行
        </span>
      </div>
      <div class="item">
        <el-radio label="3">循环</el-radio>
        <span style="margin-left: 10px; margin-right: 5px">从</span>
        <el-input-number
          @change="type = '3'"
          v-model="loop.start"
          :min="1"
          :max="31"
          controls-position="right"
          style="width: 100px"
        ></el-input-number>
        <span style="margin-left: 5px; margin-right: 5px">日开始，每</span>
        <el-input-number
          @change="type = '3'"
          v-model="loop.end"
          :min="1"
          :max="31"
          controls-position="right"
          style="width: 100px"
        ></el-input-number>
        <span style="margin-left: 5px;">
          日执行一次
        </span>
      </div>
      <div class="item">
        <el-radio label="8">工作日</el-radio>
        <span style="margin-left: 10px; margin-right: 5px">本月</span>
        <el-input-number
          @change="type = '8'"
          v-model="work"
          :min="1"
          :max="7"
          controls-position="right"
          style="width: 100px"
        ></el-input-number>
        <span style="margin-left: 5px;">
          号，最近的工作日
        </span>
      </div>
      <div class="item">
        <el-radio label="6">本月最后一天</el-radio>
      </div>
      <div style="display: flex; margin-top: 10px">
        <el-radio label="4">指定</el-radio>
        <el-checkbox-group
          @change="
            (value: string[]) => {
              type = '4';
            }
          "
          v-model="appoint"
          style="width: 600px"
        >
          <el-row v-for="i in 4" :key="i">
            <template v-for="j in 10" :index="index" :key="i + '_col'">
              <el-col
                :span="2"
                :key="j"
                v-if="
                  parseInt(i - 1 + '' + (j - 1)) < 32 && !(i === 1 && j === 1)
                "
              >
                <el-checkbox :key="(i - 1) * 10 + (j - 1) + ''" :label="(i - 1) * 10 + (j - 1) + ''">{{
                  i - 1 + "" + (j - 1)
                }}</el-checkbox>
              </el-col>
            </template>
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
  name: "day",
  props: {
    modelValue: {
      type: String,
      default: "*",
    },
  },
  setup(props, { emit }) {
    const dayData = reactive({
      type: "5", // 类型
      cycle: {
        // 周期
        start: 1,
        end: 2,
      },
      loop: {
        // 循环
        start: 1,
        end: 1,
      },
      work: 1,
    });
    const appoint = ref<Array<String>>([]);

    const value_ = computed(() => {
      let result = [];
      switch (dayData.type) {
        case "1": // 每秒
          result.push("*");
          break;
        case "2": // 周期
          result.push(`${dayData.cycle.start}-${dayData.cycle.end}`);
          break;
        case "3": // 循环
          result.push(`${dayData.loop.start}/${dayData.loop.end}`);
          break;
        case "4": // 指定
          result.push(appoint.value.join(","));
          break;
        case "6": // 最后
          result.push("L");
          break;
        case "8": // 工作日
          result.push(`${dayData.work}W`);
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
      let arr = [];
      if (props.modelValue === "?") {
        dayData.type = "5";
      } else if (props.modelValue.indexOf("-") !== -1) {
        // 2周期
        arr = props.modelValue.split("-");
        if (arr.length === 2) {
          dayData.type = "2";
          dayData.cycle.start = parseInt(arr[0]);
          dayData.cycle.end = parseInt(arr[1]);
        }
      } else if (props.modelValue.indexOf("/") !== -1) {
        // 3循环
        arr = props.modelValue.split("/");
        if (arr.length === 2) {
          dayData.type = "3";
          dayData.loop.start = parseInt(arr[0]);
          dayData.loop.end = parseInt(arr[1]);
        }
      } else if (props.modelValue.indexOf("*") !== -1) {
        // 1每
        dayData.type = "1";
      } else if (props.modelValue.indexOf("L") !== -1) {
        // 6最后
        dayData.type = "6";
      } else if (props.modelValue.indexOf("W") !== -1) {
        // 8工作日
        dayData.type = "8";
        dayData.work = parseInt(props.modelValue.replace("W", ""));
      } else {
        // *
        dayData.type = "4";
        appoint.value = props.modelValue.split(",");
      }
    };

    watch(
      () => props.modelValue,
      () => updateVal()
    );

    onBeforeMount(() => {
      updateVal()
    })

    return {
      ...toRefs(dayData),
      value_,
      appoint,
    };
  },
});
</script>

<style lang="css">
.el-checkbox + .el-checkbox {
  margin-left: 10px;
}
</style>
