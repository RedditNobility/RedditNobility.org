<template>
  <el-container direction="horizontal" style="border: 1px solid #eee">
    <el-main>
      <el-space wrap>
        <el-card>
          <template #header>
            <div class="card-header">
              <span>System Stats</span>
            </div>
          </template>
          <div v-loading="loadingStats">
            <el-descriptions title="System Stats Statistics" :column="2" border>
              <el-descriptions-item label="Users in Queue">{{
                stats.users_discovered - stats.users_reviewed
              }}</el-descriptions-item>
              <br />
              <el-descriptions-item label="Users Discovered">{{
                stats.users_discovered
              }}</el-descriptions-item>
              <el-descriptions-item label="Users Discovered This Month">{{
                stats.users_discovered_this_month
              }}</el-descriptions-item>
              <el-descriptions-item label="Users Reviewed">
                {{ stats.users_reviewed }}
              </el-descriptions-item>
              <el-descriptions-item label="Users Reviewed This Month">
                {{ stats.users_reviewed_this_month }}
              </el-descriptions-item>
            </el-descriptions>
          </div>
        </el-card>
      </el-space>
      <el-space wrap>
        <el-card>
          <template #header>
            <div class="card-header">
              <span>User Page</span>
            </div>
            <el-form
              label-position="top"
              label-width="120px"
              v-on:submit="onSubmit"
            >
              <el-form-item label="Username">
                <el-input v-model="user_page"></el-input>
              </el-form-item>
              <el-form-item>
                <el-button block native-type="submit" type="primary"
                  >Go To User</el-button
                >
              </el-form-item>
            </el-form>
          </template>
        </el-card>
      </el-space>
    </el-main>
  </el-container>
</template>

<script lang="ts">
import {
  RedditUser,
  reviewUser,
  reviewUserByName,
  User,
} from "@/backend/api/User";
import { defineComponent, ref } from "vue";
import { useCookie } from "vue-cookie-next";
import { BasicResponse } from "../Response";
import http from "@/http-common";
import { useRoute } from "vue-router";
import { getSystemStats, getTitles } from "@/backend/Generic";
export default defineComponent({
  setup() {
    const loadingStats = ref(false);
    const cookie = useCookie();
    let user_page = ref("");
    let value: UserStats = {};
    const stats = ref<UserStats>(value);

    const loadStats = async () => {
      loadingStats.value = true;
      try {
        let value = await getSystemStats(cookie.getCookie("token"));
        loadingStats.value = false;
        if (loadingStats == undefined) {
          return;
        }
        stats.value = value as UserStats;
      } catch (e) {
        console.error(e);
      }
    };
    loadStats();
    return { loadingStats, user_page, stats };
  },
  methods: {
    onSubmit() {
      this.$router.replace({
        name: "User",
        params: { username: this.user_page },
      });
    },
  },
});
</script>
<style scoped>
.post {
  border-style: solid;
}
.comment {
  border-style: solid;
}
@media only screen and (min-width: 600px) {
  #user-info {
    width: 50%;
    margin: auto;
  }
}
</style>