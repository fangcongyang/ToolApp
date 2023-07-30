<template>
  <div :val="value_">
    <el-radio-group v-model="type" class="cron-item">
      <div>
        <el-radio label="1">每时</el-radio>
      </div>
      <div class="item">
        <el-radio label="2" size="small">周期</el-radio>
        <span style="margin-right: 5px">从</span>
        <el-input-number
          @change="type = '2'"
          v-model="cycle.start"
          :min="0"
          :max="cycle.end"
          controls-position="right"
          style="width: 100px"
        ></el-input-number>
        <span style="margin-left: 5px; margin-right: 5px">至</span>
        <el-input-number
          @change="type = '2'"
          v-model="cycle.end"
          :min="cycle.start + 1"
          :max="23"
          controls-position="right"
          style="width: 100px"
        ></el-input-number>
        <span style="margin-left: 5px;">
          时循环执行
        </span>
      </div>
      <div class="item">
        <el-radio label="3" size="small">循环</el-radio>
        <span style="margin-right: 5px">从</span>
        <el-input-number
          @change="type = '3'"
          v-model="loop.start"
          :min="0"
          :max="23"
          controls-position="right"
          style="width: 100px"
        ></el-input-number>
        <span style="margin-left: 5px; margin-right: 5px">时开始，每</span>
        <el-input-number
          @change="type = '3'"
          v-model="loop.end"
          :min="1"
          :max="23"
          controls-position="right"
          style="width: 100px"
        ></el-input-number>
        <span style="margin-left: 5px;">
          时执行一次
        </span>
      </div>
      <div style="display: flex; margin-top: 10px">
        <el-radio label="4" size="small">指定</el-radio>
        <el-checkbox-group
          @change="
            (value: string[]) => {
              type = '4';
            }
          "
          v-model="appoint"
          style="width: 600px"
        >
          <el-row v-for="i in 3" :key="i">
            <template v-for="j in 10" :key="i + '_col'">
              <el-col
                :span="2"
                :key="j"
                v-if="parseInt(i - 1 + '' + (j - 1)) < 24"
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
  name: "hour",
  props: {
    modelValue: {
      type: String,
      default: "*",
    },
  },

  setup(props, { emit }) {
    const hourData = reactive({
      type: "1", // 类型
      cycle: {
        // 周期
        start: 0,
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
      let result = []
      switch (hourData.type) {
        case '1': // 每秒
          result.push('*')
          break
        case '2': // 年期
          result.push(`${hourData.cycle.start}-${hourData.cycle.end}`)
          break
        case '3': // 循环
          result.push(`${hourData.loop.start}/${hourData.loop.end}`)
          break
        case '4': // 指定
          result.push(appoint.value.join(','))
          break
        default: // 不指定
          result.push('?')
          break
      };
      emit('update:modelValue', result.join(""));
      return result.join('')
    })

    const updateVal = () => {
      if (!props.modelValue) {
        return
      }
      let arr = []
      if (props.modelValue === '?') {
        hourData.type = '5'
      } else if (props.modelValue.indexOf('-') !== -1) { // 2周期
        arr = props.modelValue.split('-')
        if (arr.length === 2) {
          hourData.type = '2'
          hourData.cycle.start = parseInt(arr[0])
          hourData.cycle.end = parseInt(arr[1])
        }
      } else if (props.modelValue.indexOf('/') !== -1) { // 3循环
        arr = props.modelValue.split('/')
        if (arr.length === 2) {
          hourData.type = '3'
          hourData.loop.start = parseInt(arr[0])
          hourData.loop.end = parseInt(arr[1])
        }
      } else if (props.modelValue.indexOf('*') !== -1) { // 1每
        hourData.type = '1'
      } else { // *
        hourData.type = '4'
        appoint.value = props.modelValue.split(',')
      }
    }

    watch(() => props.modelValue,
    () => updateVal());

    onBeforeMount(() => {
      updateVal();
    })

    return {
      ...toRefs(hourData),
      appoint,
      value_,
    };
  },
});
</script>

<style lang="css">
.el-checkbox + .el-checkbox {
  margin-left: 10px;
}
</style>
