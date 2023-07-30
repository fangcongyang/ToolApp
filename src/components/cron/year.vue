<template>
  <div :val="value_">
    <el-radio-group v-model="type" class="cron-item">
      <div>
          <el-radio label="1">每年</el-radio>
      </div>
      <div class="item">
          <el-radio label="5">不指定</el-radio>
      </div>
      <div class="item">
          <el-radio label="2">周期</el-radio>
          <span style="margin-right: 5px;">从</span>
          <el-input-number @change="type = '2'" v-model="cycle.start" :min="2000"
            controls-position="right"
            style="width: 100px;"></el-input-number>
          <span style="margin-left: 5px; margin-right: 5px;">至</span>
          <el-input-number @change="type = '2'" v-model="cycle.end" :min="2001"
            controls-position="right"
            style="width: 100px;"></el-input-number>
          <span style="margin-left: 5px;">年</span>
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
  name: "year",
  props: {
    modelValue: {
      type: String,
      default: "*",
    },
  },
  setup(props, { emit }) {
    let year = new Date().getFullYear()
    const data = reactive({
      type: '1', // 类型
      cycle: { // 周期
        start: year,
        end: year + 1,
      },
      work: 0,
      last: 0,
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
        case '4': // 指定
          result.push(appoint.value.join(','))
          break
        case '6': // 最后
          result.push(`${data.last === 0 ? '' : data.last}L`)
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
      } else if (props.modelValue.indexOf('*') !== -1) { // 1每
        data.type = '1'
      } else if (props.modelValue.indexOf('L') !== -1) { // 6最后
        data.type = '6'
        data.last = parseInt(props.modelValue.replace('L', ''))
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
    }
  }
});
</script>

<style lang="css">
.el-checkbox+.el-checkbox {
    margin-left: 10px;
}
</style>