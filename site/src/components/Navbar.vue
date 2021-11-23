<template>
  <el-menu :default-active="activeIndex" class="el-menu-demo" mode="horizontal">
    <el-menu-item index="Home" @click="router.push('/')">Index</el-menu-item>
  </el-menu>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import { useRouter } from "vue-router";
import { AuthToken, BasicResponse, User } from "@/backend/Response";
import http from "@/http-common";
export default defineComponent({
  props: {
    user: {
      required: true,
      type: Object as () => User,
    },
  },
  setup() {
    const router = useRouter();
    const activeIndex = ref(router.currentRoute.value.name);
    return { activeIndex, router };
  },
});
</script>