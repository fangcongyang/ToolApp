<template>
  <div>
    <el-input v-model="execTime" placeholder="cron表达式" />
    <cron v-model="execTime" />
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
