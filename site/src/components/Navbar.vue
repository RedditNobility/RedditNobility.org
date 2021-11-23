<template>
  <el-menu :default-active="activeIndex" class="el-menu-demo" mode="horizontal">
    <el-menu-item index="Home" @click="router.push('/')">Index</el-menu-item>

    <el-menu-item
      v-if="user.id == 0"
      index="Login"
      @click="router.push('/login')"
      >Login</el-menu-item
    >
    <el-menu-item
      v-if="user.permissions.submit"
      index="Submit"
      @click="router.push('/')"
      >Submit</el-menu-item
    >
    <el-menu-item
      v-if="user.permissions.approve_user"
      index="Review Users"
      @click="router.push('/')"
      >Review Users</el-menu-item
    >
    <el-menu-item
      v-if="user.permissions.modify_user"
      index="Modify User"
      @click="router.push('/')"
      >Modify User</el-menu-item
    >    
    <el-menu-item
      v-if="user.permissions.admin"
      index="Admin"
      @click="router.push('/')"
      >Admin</el-menu-item
    >
  </el-menu>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import { useRouter } from "vue-router";
import { AuthToken, BasicResponse } from "@/backend/Response";
import http from "@/http-common";
import { User } from "@/backend/api/User";
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