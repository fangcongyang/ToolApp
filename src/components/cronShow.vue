<template>
  <div>
    <span>cron表达式:</span><el-input v-model="execTime" style="width: 300Px;margin-left: 10Px;" placeholder="cron表达式" />
    <cron v-model="execTime" style="margin-top: 10Px;" />
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, watch } from "vue";
import cron from "./cron.vue";

export default defineComponent({
  name: "cronShow",
  components: {
    cron,
  },
  
  setup(props, { emit }) {
    const execTime = ref<string>("");

    return {
      execTime,
    };
  },
});
</script>
