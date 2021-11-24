<template>
  <el-container direction="horizontal" style="border: 1px solid #eee">
    <el-main v-loading="loading">
      <div v-if="user != undefined">
        <h1>{{ user.name }}</h1>
      </div>
      <div v-else>
          Unable to FInd User
      </div>
    </el-main>
  </el-container>
</template>

<script lang="ts">
import { RedditUser, reviewUser, User } from "@/backend/api/User";
import { defineComponent, ref } from "vue";
import { useCookie } from "vue-cookie-next";
export default defineComponent({
  setup() {
    let cookie = useCookie();
    const loading = ref(true);
    const user = ref<RedditUser | undefined>(undefined);
    const getRedditUser = async () => {
      try {
        const value = await reviewUser(cookie.getCookie("token"));
        loading.value = false;
        if (value == undefined) {
          return;
        }
        user.value = value as RedditUser;
      } catch (e) {
        console.error(e);
      }
    };
    getRedditUser();
    return { loading, user, cookie };
  },
});
</script>
<style scoped></style>