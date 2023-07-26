<template lang="html">
  <span>
    <el-popover placement="right">
      <cron @input="changeValue" v-model="execTime" />
      <el-input
        slot="reference"
        field-decorator-id="scheduleConf"
        v-model="execTime"
      />
    </el-popover>
  </span>
</template>
<script lang="ts">
import { defineComponent, watch, ref } from "vue";
import cron from "./cron.vue";

export default defineComponent({
  name: "cronForm",
  components: {
    cron,
  },
  props: {
    value: {
      type: String,
      default: "",
    },
  },
  setup(props, { emit }) {
    const execTime = ref<String>("");

    //触发整体修改,触发change事件
    const changeValue = (value: string) => {
      //提供 onChange 事件或 trigger 的值同名的事件。
      emit("change", value);
    };

    watch(
      () => props.value,
      () => {
        execTime.value = props.value;
      }
    );

    return {
      execTime,
      changeValue,
    };
  },
});
</script>
