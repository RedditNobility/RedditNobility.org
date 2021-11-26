<template>
  <el-container style="border: 1px solid #eee">
    <el-main v-loading="loading">
      <div v-if="user != undefined">
        <h1>{{ user.name }}</h1>
        <el-tabs id="user-info" type="border-card">
          <el-tab-pane label="User">
            <div>
              <el-descriptions class="margin-top" title="User" border>
                <el-descriptions-item>
                  <template #label>
                    <el-icon><user /></el-icon>
                    Username
                  </template>
                  {{ user.username }}
                </el-descriptions-item>
                <el-descriptions-item>
                  <template #label>
                    <el-icon><user /></el-icon>
                    Discoverer
                  </template>
                  {{ user.discoverer }}
                </el-descriptions-item>
                <el-descriptions-item>
                  <template #label>
                    <el-icon><user /></el-icon>
                    Created On
                  </template>
                  {{ date }}
                </el-descriptions-item>
                <el-descriptions-item>
                  <template #label> Status Changed </template>
                  {{ status_changed }}
                </el-descriptions-item>
                <el-descriptions-item>
                  <template #label> Moderator</template>
                  {{ user.reviewer }}
                </el-descriptions-item>
                <el-descriptions-item>
                  <template #label> Status</template>
                  <el-select disabled v-model="user.status" placeholder="">
                    <el-option
                      @click="updateStatus"
                      label="Approved"
                      value="Approved"
                    >
                    </el-option>
                    <el-option
                      @click="updateStatus"
                      label="Denied"
                      value="Denied"
                    >
                    </el-option>
                  </el-select>
                </el-descriptions-item>
                <el-descriptions-item v-if="titles.length > 1">
                  <template #label> Title </template>
                  <el-select
                    disabled
                    v-model="user.properties.title"
                    placeholder=""
                  >
                    <el-option
                      v-for="title in titles"
                      @click="updateTitle"
                      :key="title.value"
                      :label="title.value"
                      :value="title.value"
                    >
                    </el-option>
                  </el-select>
                </el-descriptions-item>
              </el-descriptions>
            </div>
          </el-tab-pane>
          <el-tab-pane label="Permissions">
            <div>
              <el-form :disabled="!userStore.state.user.permissions.admin">
                <el-form-item label="Admin">
                  <el-switch
                    v-model="user.permissions.admin"
                    @change="updatePermission('admin')"
                  />
                </el-form-item>
                <el-form-item label="Moderator">
                  <el-switch
                    v-model="user.permissions.moderator"
                    @change="updatePermission('moderator')"
                  />
                </el-form-item>
                <el-form-item label="Login">
                  <el-switch
                    v-model="user.permissions.login"
                    @change="updatePermission('login')"
                  />
                </el-form-item>
                <el-form-item label="Recruit Users">
                  <el-switch
                    v-model="user.permissions.reviewer"
                    @change="updatePermission('reviewer')"
                  />
                </el-form-item>
                <el-form-item label="Submit Users">
                  <el-switch
                    v-model="user.permissions.submit"
                    @change="updatePermission('submit')"
                  />
                </el-form-item>
              </el-form>
            </div>
          </el-tab-pane>
          <el-tab-pane label="Statistics">
            <div v-loading="loadingStats">
              <el-descriptions title="User Statistics">
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
          </el-tab-pane>
        </el-tabs>
      </div>
      <div v-else>Unable to FInd User</div>
    </el-main>
  </el-container>
</template>
<script lang="ts">
import {
  getUserByName,
  getUserStats,
  User,
  UserStats,
} from "@/backend/api/User";
import { getTitles } from "@/backend/Generic";
import { defineComponent, ref } from "vue";
import { useCookie } from "vue-cookie-next";
import { useRoute } from "vue-router";
import userStore from "@/store/user";
import { BasicResponse } from "../Response";
import http from "@/http-common";
export default defineComponent({
  setup() {
    const titles = ref<string[]>([]);
    const loading = ref(true);
    const loadingStats = ref(true);
    const date = ref("");
    const status_changed = ref("");
    const user = ref<User | undefined>(undefined);
    let value: UserStats = {};
    const stats = ref<UserStats>(value);

    const load = async () => {
      let value = await getTitles();
      for (const title of value) {
        titles.value.push({ value: title });
      }
    };
    const route = useRoute();
    let username = route.params.username as string;
    const cookie = useCookie();
    const loadUser = async () => {
      loading.value = true;
      user.value = undefined;
      try {
        let value = await getUserByName(username, cookie.getCookie("token"));
        loading.value = false;
        if (value == undefined) {
          return;
        }
        let u = value as User;
        date.value = new Date(u.created).toLocaleDateString("en-US");
        status_changed.value = new Date(u.status_changed).toLocaleDateString(
          "en-US"
        );
        user.value = u;
      } catch (e) {
        console.error(e);
      }
    };
    const loadStats = async () => {
      loadingStats.value = true;
      try {
        let value = await getUserStats(username, cookie.getCookie("token"));
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
    loadUser();
    load();
    return {
      userStore,
      user,
      stats,
      loadStats,
      loading,
      loadingStats,
      titles,
      date,
      status_changed,
    };
  },
  methods: {
    async updateTitle() {
      console.log("HEY");
    },
    async updateStatus() {},
    async updatePermission(permission: string) {
      let user = this.user as User;
      let value = user.permissions[permission];
      let url =
        "/api/admin/user/" +
        user.id +
        "/permission/" +
        permission +
        "/" +
        value;
      let response = await http
        .post(
          url,
          {},
          {
            headers: {
              Authorization: "Bearer " + this.$cookie.getCookie("token"),
            },
          }
        )
        .then((res) => {
          console.log(typeof res);
          if (res.status == 200) {
            this.$notify({
              title: "Updated Permission: "+permission + ": "+value,
              type: "info",
            });
          } else {
            this.$notify({
              title: "Unable to update ermission",
              type: "warn",
            });
          }
        })
        .catch((error) => {
          console.error(error);
            this.$notify({
              title: "Unable to update ermission",
              type: "warn",
            });
        });
    },
  },
});
</script>
<style scoped>
@media only screen and (min-width: 1200px) {
  #user-info {
    width: 50%;
    margin: auto;
  }
}
.post {
  border-style: solid;
}
.comment {
  border-style: solid;
}
</style>