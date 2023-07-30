<template>
  <div :val="value_">
    <el-radio-group v-model="type" class="cron-item">
      <div>
        <el-radio label="1">每月</el-radio>
      </div>
      <div class="item">
        <el-radio label="5">不指定</el-radio>
      </div>
      <div class="item">
        <el-radio label="2">周期</el-radio>
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
          :min="loop.start + 1"
          :max="12"
          controls-position="right"
          style="width: 100px"
        ></el-input-number>
        月
      </div>
      <div style="margin-top: 10px">
        <el-radio label="3">循环</el-radio>
        <span style="margin-right: 5px">从</span>
        <el-input-number
          @change="type = '3'"
          v-model="loop.start"
          :min="1"
          :max="12"
          controls-position="right"
          style="width: 100px"
        ></el-input-number>
        <span style="margin-left: 5px; margin-right: 5px">月开始，每</span>
        <el-input-number
          @change="type = '3'"
          v-model="loop.end"
          :min="2"
          :max="12"
          controls-position="right"
          style="width: 100px"
        ></el-input-number>
        <span style="margin-left: 5px;">月执行一次</span>
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
          <el-row>
            <el-col :span="2" v-for="j in 12" :key="j">
              <el-checkbox :key="j" :label="j">{{ j }}</el-checkbox>
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
  name: "month",
  props: {
    modelValue: {
      type: String,
      default: "*",
    },
  },
  setup(props, { emit }) {
    const monthData = reactive({
      type: "1", // 类型
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
      last: 0,
    });
    const appoint = ref<Array<String>>([]);
    const value_ = computed(() => {
      let result = []
      switch (monthData.type) {
        case '1': // 每秒
          result.push('*')
          break
        case '2': // 年期
          result.push(`${monthData.cycle.start}-${monthData.cycle.end}`)
          break
        case '3': // 循环
          result.push(`${monthData.loop.start}/${monthData.loop.end}`)
          break
        case '4': // 指定
          result.push(appoint.value.join(','))
          break
        case '6': // 最后
          result.push(`${monthData.last === 0 ? '' : monthData.last}L`)
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
      let arr
      if (props.modelValue === '?') {
        monthData.type = '5'
      } else if (props.modelValue.indexOf('-') !== -1) { // 2周期
        arr = props.modelValue.split('-')
        if (arr.length === 2) {
          monthData.type = '2'
          monthData.cycle.start = parseInt(arr[0])
          monthData.cycle.end = parseInt(arr[1])
        }
      } else if (props.modelValue.indexOf('/') !== -1) { // 3循环
        arr = props.modelValue.split('/')
        if (arr.length === 2) {
          monthData.type = '3'
          monthData.loop.start = parseInt(arr[0])
          monthData.loop.end = parseInt(arr[1])
        }
      } else if (props.modelValue.indexOf('*') !== -1) { // 1每
        monthData.type = '1'
      } else if (props.modelValue.indexOf('L') !== -1) { // 6最后
        monthData.type = '6'
        monthData.last = parseInt(props.modelValue.replace('L', ''))
      } else { // *
        monthData.type = '4'
        appoint.value = props.modelValue.split(',')
      }
    }

    watch(() => props.modelValue,
    () => updateVal());

    onBeforeMount(() => {
      updateVal();
    })

    return {
      ...toRefs(monthData),
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
