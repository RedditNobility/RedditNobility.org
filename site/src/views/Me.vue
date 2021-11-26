<template>
  <el-container style="border: 1px solid #eee">
    <el-main v-loading="user == undefined">
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
                    <el-option label="Approved" value="Approved"> </el-option>
                    <el-option label="Denied" value="Denied"> </el-option>
                  </el-select>
                </el-descriptions-item>
                <el-descriptions-item>
                  <template #label> Title </template>
                  <el-select
                    disabled
                    v-model="user.properties.title"
                    placeholder=""
                  >
                    <el-option
                      :key="user.properties.title"
                      :label="user.properties.title"
                      :value="user.properties.title"
                    >
                    </el-option>
                  </el-select>
                </el-descriptions-item>
              </el-descriptions>
            </div>
          </el-tab-pane>
          <el-tab-pane label="Permissions">
            <div>
              <el-form disabled>
                <el-form-item label="Admin">
                  <el-switch v-model="user.permissions.admin" />
                </el-form-item>
                <el-form-item label="Moderator">
                  <el-switch v-model="user.permissions.moderator" />
                </el-form-item>
                <el-form-item label="Login">
                  <el-switch v-model="user.permissions.login" />
                </el-form-item>
                <el-form-item label="Recruit Users">
                  <el-switch v-model="user.permissions.review_user" />
                </el-form-item>
                <el-form-item label="Submit Users">
                  <el-switch v-model="user.permissions.submit" />
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
          <el-tab-pane label="Change Password">
            <el-form
              v-on:submit="updatePassword"
              label-position="top"
              :model="password"
              label-width="120px"
            >
              <el-form-item>
                <el-form-item label="Password">
                  <el-input
                    v-model="password.password"
                    placeholder="Please input password"
                    show-password
                    autocomplete="off"
                  />
                </el-form-item>
                <el-form-item label="Confirm Password">
                  <el-input
                    autocomplete="off"
                    v-model="password.confirm_password"
                    placeholder="Please input password"
                    show-password
                  />
                </el-form-item>
                <!--Yeah, I know. But please don't judge -->
                <el-button
                  :disabled="
                    password.password.length == 0 ||
                    password.password != password.confirm_password
                  "
                  type="primary"
                  native-type="submit"
                  @click="updatePassword"
                  >Update Passwords</el-button
                >
              </el-form-item>
            </el-form>
          </el-tab-pane>
        </el-tabs>
      </div>
      <div v-else>Unable to FInd User</div>
    </el-main>
  </el-container>
</template>
<script lang="ts">
import {
  getUser,
  getUserByName,
  getUserStats,
  User,
  UserStats,
} from "@/backend/api/User";
import { getTitles } from "@/backend/Generic";
import { defineComponent, onBeforeMount, ref } from "vue";
import { useCookie } from "vue-cookie-next";
import { useRoute } from "vue-router";
import userStore from "@/store/user";
import http from "@/http-common";
import useStore from "element-plus/es/components/table/src/store";
export default defineComponent({
  name: "Me",
  setup() {
    const loadingStats = ref(true);
    const loading = ref(true);
    const user = ref<User | undefined>(undefined);
    let value: UserStats = {};
    const stats = ref<UserStats>(value);
    const password = ref({
      password: "",
      confirm_password: "",
    });
    const route = useRoute();
    const cookie = useCookie();
    const date = ref("");
    const status_changed = ref("");
    const loadUser = async () => {
      loading.value = true;
      user.value = undefined;
      try {
        let value = await getUser(cookie.getCookie("token"));
        if (value == undefined) {
          return;
        }
        let u = value as User;
        date.value = new Date(u.created).toLocaleDateString("en-US");
        status_changed.value = new Date(u.status_changed).toLocaleDateString(
          "en-US"
        );
        user.value = u;
        loadStats();
      } catch (e) {
        console.error(e);
      }
    };
    const loadStats = async () => {
      loadingStats.value = true;
      try {
        let value = await getUserStats(
          user.value.username,
          cookie.getCookie("token")
        );
        loadingStats.value = false;
        if (loadingStats == undefined) {
          return;
        }
        stats.value = value as UserStats;
      } catch (e) {
        console.error(e);
      }
    };
    loadUser();
    return {
      userStore,
      user,
      stats,
      loadStats,
      loadingStats,
      date,
      status_changed,
      password,
    };
  },
  methods: {
    async updatePassword(e: any) {
      e.preventDefault();
      if (this.password.password !== this.password.confirm_password) {
        this.$notify({
          title: "Passwords Do Not Match",
          type: "warn",
        });
        this.password.password = "";
        this.password.confirm_password = "";
        return;
      }
      let body = {
        value: this.password.password,
      };

      await http
        .post("api/me/password/change", body, {
          headers: {
            Authorization: "Bearer " + this.$cookie.getCookie("token"),
          },
        })
        .then((res) => {
          console.log(typeof res);
          if (res.status != 200) {
            console.error(res.data);
            this.$notify({
              title: "Unable to Update Password",
              type: "warn",
            });
          } else {
            this.password.password = "";
            this.password.confirm_password = "";
            this.$notify({
              title: "Password Updated",
              type: "info",
            });
          }
        })
        .catch((error) => {
          console.error(error);
          this.$notify({
            title: "Unable to Update Password",
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