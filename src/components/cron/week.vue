<template>
  <div :val="value_">
    <el-radio-group v-model="type" class="cron-item">
        <div>
            <el-radio label="1">每周</el-radio>
        </div>
        <div class="item">
            <el-radio label="5">不指定</el-radio>
        </div>
        <div class="item">
            <el-radio label="2">周期</el-radio>
            <span style="margin-right: 5px;">从星期</span>
            <el-select  @change="type = '2'" v-model="cycle.start" placeholder="星期"
              style="width: 100px;">
              <el-option
                v-for="item in weekSelect"
                :key="item.value"
                :label="item.label"
                :value="item.value"
              />
            </el-select>
            <span style="margin-left: 5px; margin-right: 5px;">至星期</span>
            <el-select  @change="type = '2'" v-model="cycle.end" placeholder="星期"
              style="width: 100px;">
              <el-option
                v-for="item in weekSelect"
                :key="item.value"
                :label="item.label"
                :value="item.value"
              />
            </el-select>
        </div>
        <div class="item">
            <el-radio label="3">循环</el-radio>
            <span style="margin-right: 5px;">从星期</span>
            <el-select  @change="type = '3'" v-model="loop.start" placeholder="星期"
              style="width: 100px;">
              <el-option
                v-for="item in weekSelect"
                :key="item.value"
                :label="item.label"
                :value="item.value"
              />
            </el-select>
            <span style="margin-left: 5px; margin-right: 5px;">开始，每</span>
            <el-input-number @change="type = '3'" v-model="loop.end" :min="1" :max="7"
              controls-position="right"
              style="width: 100px;">
            </el-input-number>
            <span style="margin-left: 5px;">天执行一次</span>
        </div>
        <div class="item">
            <el-radio label="7">指定周</el-radio>
            <span style="margin-right: 5px;">本月第</span>
            <el-input-number @change="type = '7'" v-model="week.start" :min="1" :max="4"
              controls-position="right"
              style="width: 100px;">
            </el-input-number>
            <span style="margin-left: 5px; margin-right: 5px;">周，星期</span>
            <el-select  @change="type = '7'" v-model="week.end" placeholder="星期"
              style="width: 100px;">
              <el-option
                v-for="item in weekSelect"
                :key="item.value"
                :label="item.label"
                :value="item.value"
              />
            </el-select>
        </div>
        <div class="item">
            <el-radio label="6">本月最后一个</el-radio>
            <el-select  @change="type = '6'" v-model="last" placeholder="星期"
              style="width: 100px;">
              <el-option
                v-for="item in weekSelect"
                :key="item.value"
                :label="item.label"
                :value="item.value"
              />
            </el-select>
        </div>
        <div style="display:flex;margin-top: 10px;">
            <el-radio label="4">指定</el-radio>
            <el-checkbox-group @change="(value: string[]) => {type ='4';}" v-model="appoint" style="width: 600px;">
                <el-row>
                    <el-col :span="2" v-for="j in 7" :key="j">
                        <el-checkbox :key="j" :label="j">{{j}}</el-checkbox>
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
const weekSelect = [
  {label: "星期一", value: 2,},
  {label: "星期二", value: 3,},
  {label: "星期三", value: 4,},
  {label: "星期四", value: 5,},
  {label: "星期五", value: 6,},
  {label: "星期六", value: 7,},
  {label: "星期天", value: 1,}
]
export default defineComponent({
  name: "week",
  props: {
    modelValue: {
      type: String,
      default: "?",
    },
  },
  setup(props, { emit }) {
    const data = reactive({
      type: "1", // 类型
      cycle: {
        // 周期
        start: 2,
        end: 3,
      },
      loop: {
        // 循环
        start: 2,
        end: 1,
      },
      week: { // 指定周
        start: 1,
        end: 1
      },
      last: 2,
    })
    const appoint = ref<Array<String>>([]);
    const value_ = computed(() => {
      let result = []
      switch (data.type) {
        case '1': // 每秒
          result.push('*')
          break
        case '2': // 年期
          result.push(`${data.cycle.start}-${data.cycle.end}`)
          break
        case '3': // 循环
          result.push(`${data.loop.start}/${data.loop.end}`)
          break
        case '4': // 指定
          result.push(appoint.value.join(','))
          break
        case '6': // 最后
          result.push(`${data.last === 0 ? '' : data.last}L`)
          break
        case '7': // 指定周
          result.push(`${data.week.end}#${data.week.start}`)
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
        data.type = '5'
      } else if (props.modelValue.indexOf('-') !== -1) { // 2周期
        arr = props.modelValue.split('-')
        if (arr.length === 2) {
          data.type = '2'
          data.cycle.start = parseInt(arr[0])
          data.cycle.end = parseInt(arr[1])
        }
      } else if (props.modelValue.indexOf('/') !== -1) { // 3循环
        arr = props.modelValue.split('/')
        if (arr.length === 2) {
          data.type = '3'
          data.loop.start = parseInt(arr[0])
          data.loop.end = parseInt(arr[1])
        }
      } else if (props.modelValue.indexOf('*') !== -1) { // 1每
        data.type = '1'
      } else if (props.modelValue.indexOf('L') !== -1) { // 6最后
        data.type = '6'
        data.last = parseInt(props.modelValue.replace('L', ''))
      } else if (props.modelValue.indexOf('#') !== -1) { // 7指定周
        arr = props.modelValue.split('#')
        if (arr.length === 2) {
          data.type = '7'
          data.week.start = parseInt(arr[0])
          data.week.end = parseInt(arr[1])
        }
      } else { // *
        data.type = '4'
        appoint.value = props.modelValue.split(',')
      }
    }
    
    watch(() => props.modelValue,
    () => updateVal());

    onBeforeMount(() => {
      updateVal();
    })

    return {
      ...toRefs(data),
      appoint,
      value_,
      weekSelect,
    }
  },
});
</script>

<style lang="css">
.el-checkbox+.el-checkbox {
    margin-left: 10px;
}
</style>