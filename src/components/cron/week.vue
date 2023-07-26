<template lang="html">
  <div :val="value_">
    <ta-radio-group v-model="type">
        <div>
            <ta-radio value="1">每周</ta-radio>
        </div>
        <div style="margin-top: 10px;">
            <ta-radio value="5">不指定</ta-radio>
        </div>
        <div style="margin-top: 10px;">
            <ta-radio value="2">周期</ta-radio>
            <span style="margin-left: 10px; margin-right: 5px;">从星期</span>
            <ta-input-number @change="type = '2'" v-model="cycle.start" :min="1" :max="7" style="width: 100px;"></ta-input-number>
            <span style="margin-left: 5px; margin-right: 5px;">至星期</span>
            <ta-input-number @change="type = '2'" v-model="cycle.end" :min="2" :max="7" style="width: 100px;"></ta-input-number>
        </div>
        <div style="margin-top: 10px;">
            <ta-radio value="3">循环</ta-radio>
            <span style="margin-left: 10px; margin-right: 5px;">从星期</span>
            <ta-input-number @change="type = '3'" v-model="loop.start" :min="1" :max="7" style="width: 100px;"></ta-input-number>
            <span style="margin-left: 5px; margin-right: 5px;">开始，每</span>
            <ta-input-number @change="type = '3'" v-model="loop.end" :min="1" :max="7" style="width: 100px;"></ta-input-number>
            天执行一次
        </div>
        <div style="margin-top: 10px;">
            <ta-radio value="7">指定周</ta-radio>
            <span style="margin-left: 10px; margin-right: 5px;">本月第</span>
            <ta-input-number @change="type = '7'" v-model="week.start" :min="1" :max="4" style="width: 100px;"></ta-input-number>
            <span style="margin-left: 5px; margin-right: 5px;">周，星期</span>
            <ta-input-number @change="type = '7'" v-model="week.end" :min="1" :max="7" style="width: 100px;"></ta-input-number>
        </div>
        <div style="margin-top: 10px;">
            <ta-radio value="6">本月最后一个</ta-radio>
            <span style="margin-left: 10px; margin-right: 5px;">星期</span>
            <ta-input-number @change="type = '6'" v-model="last" :min="1" :max="7" style="width: 100px;"></ta-input-number>
        </div>
        <div style="display:flex;margin-top: 10px;">
            <ta-radio value="4">指定</ta-radio>
            <ta-checkbox-group @change="(value) => {type ='4'; appoint = value}" :value="appoint" style="width: 600px;">
                <ta-row>
                    <ta-col :span="2" v-for="j in 7" :key="j">
                        <ta-checkbox :value="j">{{j}}</ta-checkbox>
                    </ta-col>
                </ta-row>
            </ta-checkbox-group>
        </div>
    </ta-radio-group>
  </div>
</template>

<script>
export default {
  props: {
    value: {
      type: String,
      default: '*'
    }
  },
  data () {
    return {
      type: '1', // 类型
      cycle: { // 周期
        start: 1,
        end: 2
      },
      loop: { // 循环
        start: 1,
        end: 1
      },
      week: { // 指定周
        start: 1,
        end: 1
      },
      last: 1,
      appoint: [] // 指定
    }
  },
  computed: {
    value_ () {
      let result = []
      switch (this.type) {
        case '1': // 每秒
          result.push('*')
          break
        case '2': // 年期
          result.push(`${this.cycle.start}-${this.cycle.end}`)
          break
        case '3': // 循环
          result.push(`${this.loop.start}/${this.loop.end}`)
          break
        case '4': // 指定
          result.push(this.appoint.join(','))
          break
        case '6': // 最后
          result.push(`${this.last === 0 ? '' : this.last}L`)
          break
        case '7': // 指定周
          result.push(`${this.week.start}#${this.week.end}`)
          break
        default: // 不指定
          result.push('?')
          break
      };
      this.$emit('input', result.join(''))
      return result.join('')
    }
  },
  watch: {
    'value' (a, b) {
      this.updateVal()
    }
  },
  methods: {
    updateVal () {
      if (!this.value) {
        return
      }
      if (this.value === '?') {
        this.type = '5'
      } else if (this.value.indexOf('-') !== -1) { // 2周期
        if (this.value.split('-').length === 2) {
          this.type = '2'
          this.cycle.start = this.value.split('-')[0]
          this.cycle.end = this.value.split('-')[1]
        }
      } else if (this.value.indexOf('/') !== -1) { // 3循环
        if (this.value.split('/').length === 2) {
          this.type = '3'
          this.loop.start = this.value.split('/')[0]
          this.loop.end = this.value.split('/')[1]
        }
      } else if (this.value.indexOf('*') !== -1) { // 1每
        this.type = '1'
      } else if (this.value.indexOf('L') !== -1) { // 6最后
        this.type = '6'
        this.last = this.value.replace('L', '')
      } else if (this.value.indexOf('#') !== -1) { // 7指定周
        if (this.value.split('#').length === 2) {
          this.type = '7'
          this.week.start = this.value.split('#')[0]
          this.week.end = this.value.split('#')[1]
        }
      } else { // *
        this.type = '4'
        this.appoint = this.value.split(',')
      }
    }
  },
  created () {
    this.updateVal()
  }
}
</script>

<style lang="css">
.el-checkbox+.el-checkbox {
    margin-left: 10px;
}
</style>