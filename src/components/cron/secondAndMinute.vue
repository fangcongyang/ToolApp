<!-- 秒,分钟 -->
<template lang="html">
  <div :val="value_">
    <ta-radio-group v-model="type">
        <div>
            <ta-radio value="1" border>每{{lable}}</ta-radio>
        </div>
        <div style="margin-top: 10px;">
            <ta-radio value="2">周期</ta-radio>
            <span style="margin-left: 10px; margin-right: 5px;">从</span>
            <ta-input-number @change="type = '2'" v-model="cycle.start" :min="1" :max="59" style="width: 100px;"></ta-input-number>
            <span style="margin-left: 5px; margin-right: 5px;">至</span>
            <ta-input-number @change="type = '2'" v-model="cycle.end" :min="2" :max="59" style="width: 100px;"></ta-input-number>
            {{lable}}
        </div>
        <div style="margin-top: 10px;">
            <ta-radio value="3">循环</ta-radio>
            <span style="margin-left: 10px; margin-right: 5px;">从</span>
            <ta-input-number @change="type = '3'" :min="0" :max="59" v-model="loop.start" style="width: 100px;"></ta-input-number>
            <span style="margin-left: 5px; margin-right: 5px;">{{lable}}开始，每</span>
            <ta-input-number @change="type = '3'" :min="1" :max="59" v-model="loop.end" style="width: 100px;"></ta-input-number>
            {{lable}}执行一次
        </div>
        <div style="display:flex;margin-top: 10px;">
            <ta-radio value="4">指定</ta-radio>
            <ta-checkbox-group @change="(value) => {type ='4'; appoint = value}" :value="appoint" style="width: 600px;">
                <ta-row v-for="i in 6" :key="i" >
                    <ta-col :span="2" v-for="j in 10" :key="j">
                        <ta-checkbox :value="(i - 1) * 10 + (j - 1) + ''">{{(i - 1) + '' + (j - 1)}}</ta-checkbox>
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
    },
    lable: {
      type: String
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
        start: 0,
        end: 1
      },
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