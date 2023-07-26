<template lang="html">
  <div class="cron" :val="value_">
    <ta-tabs>
      <ta-tab-pane tab="秒" key="s">
        <second-and-minute v-model="sVal" lable="秒"></second-and-minute >
      </ta-tab-pane>
      <ta-tab-pane tab="分" key="m">
        <second-and-minute v-model="mVal" lable="分"></second-and-minute >
      </ta-tab-pane>
      <ta-tab-pane tab="时" key="h">
        <hour v-model="hVal" lable="时"></hour>
      </ta-tab-pane>
      <ta-tab-pane tab="日" key="d">
        <day v-model="dVal" lable="日"></day>
      </ta-tab-pane>
      <ta-tab-pane tab="月" key="month">
        <month v-model="monthVal" lable="月"></month>
      </ta-tab-pane>
      <ta-tab-pane tab="周" key="week">
        <week v-model="weekVal" lable="周"></week>
      </ta-tab-pane>
      <ta-tab-pane tab="年" key="year">
        <year v-model="yearVal" lable="年"></year>
      </ta-tab-pane>
    </ta-tabs>
    <div>
        <span style="font-weight: 800;">最近运行时间: </span>
        <div style="height: 100px; margin-top: 10px;">
            <span style="display: block;" v-for="recentExecuteTime in recentExecuteTimes">{{recentExecuteTime}}</span>
        </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, watch, computed, ref, reactive, toRefs } from "vue";
import SecondAndMinute from './cron/secondAndMinute.vue'
import hour from './cron/hour.vue'
import day from './cron/day.vue'
import month from './cron/month.vue'
import week from './cron/week.vue'
import year from './cron/year.vue'
import { ElMessage } from 'element-plus';

export default defineComponent({
  name: "cron",
  components: {
    SecondAndMinute, 
    hour, 
    day, 
    month, 
    week, 
    year
  },
  props: {
    value: {
      type: String,
      default: "",
    },
  },
  setup(props, {emit}) {
    const cronData = reactive({
      activeName: 's',
      sVal: '*',
      mVal: '*',
      hVal: '*',
      dVal: '*',
      monthVal: '*',
      weekVal: '?',
      yearVal: '',
    })
    const recentExecuteTimes = ref<Array<String>>([]);
    const value_ = computed(() => {
      if (!cronData.dVal && !cronData.weekVal) {
        return ''
      }
      if (cronData.dVal === '?' && cronData.weekVal === '?') {
        ElMessage.error('日期与星期不可以同时为“不指定”')
      }
      if (cronData.dVal !== '?' && cronData.weekVal !== '?') {
        ElMessage.error('日期与星期必须有一个为“不指定”')
      }
      let v = `${cronData.sVal} ${cronData.mVal} ${cronData.hVal} ${cronData.dVal} ${cronData.monthVal} ${cronData.weekVal} ${cronData.yearVal}`
      if (v !== props.value) {
        emit('input', v)
      }
      return v
    })
    
    return {
      ...toRefs(cronData),
      recentExecuteTimes,
    }
  },
});
// export default {
//   watch: {
//     'value' (a, b) {
//       this.updateVal()
//     }
//   },
//   methods: {
//     updateVal () {
//       if (!this.value) {
//         return
//       }
//       let arrays = this.value.split(' ')
//       this.sVal = arrays[0]
//       this.mVal = arrays[1]
//       this.hVal = arrays[2]
//       this.dVal = arrays[3]
//       this.monthVal = arrays[4]
//       this.weekVal = arrays[5]
//       this.yearVal = arrays[6]
//       Base.submit(null, { url: '/quartzJob/nextTriggerTime', data: {scheduleType: "CRON", scheduleConf: this.value} }, {
//           successCallback: (data) => {
//               this.recentExecuteTimes = data.data.result;
//           },
//       })
//     }
//   },
//   created () {
//     this.updateVal()
//   },
// }
</script>

<style lang="css">
.cron {
  text-align: left;
  padding: 10px;
  background: #fff;
  border: 1px solid #dcdfe6;
  box-shadow: 0 2px 4px 0 rgba(0,0,0,.12), 0 0 6px 0 rgba(0,0,0,.04);
}
</style>