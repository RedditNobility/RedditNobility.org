<template>
  <el-container direction="horizontal" style="border: 1px solid #eee">
    <el-main>
      <el-space wrap>
        <el-card>
          <template #header>
            <div class="card-header">
              <span>Team Members</span>
            </div>
          </template>
          <el-button type="primary" @click="newUser()" size="small"
            >New Team</el-button
          >

          <div v-loading="loadingTeamMembers">
            <ul class="infinite-list" style="overflow: auto">
              <li v-for="t in team" :key="t.id" class="infinite-list-item">
                <el-descriptions class="margin-top" :column="2" border>
                  <el-descriptions-item>
                    <template #label>
                      <el-icon><user /></el-icon>
                      Username
                    </template>
                    {{ t.user.username }}
                  </el-descriptions-item>
                  <el-descriptions-item>
                    <template #label> Update </template>
                    <el-button
                      type="primary"
                      @click="updateUser(t.id)"
                      size="small"
                      >Update Team Member</el-button
                    >
                    <el-button type="danger" disabled size="small"
                      >Delete Team Member</el-button
                    >
                  </el-descriptions-item>
                  <el-descriptions-item>
                    <template #label> Level </template>
                    {{ t.level }}
                  </el-descriptions-item>
                  <el-descriptions-item>
                    <template #label> Description </template>
                    {{ t.description }}
                  </el-descriptions-item>
                </el-descriptions>
              </li>
            </ul>
          </div>
        </el-card>
      </el-space>
      <el-dialog v-model="dialogVisible" title="Update Team Member">
        <el-form
          :model="teamInfo"
          label-position="top"
          label-width="120px"
          v-on:submit="teamUpdateNew"
        >
          <el-form-item label="Username">
            <el-input v-model="teamInfo.user"></el-input>
          </el-form-item>
          <el-form-item label="Level">
            <el-select v-model="teamInfo.level" placeholder="">
              <el-option label="Moderator" value="Moderator"> </el-option>
              <el-option label="Recruiter" value="Recruiter"> </el-option>
              <el-option label="Retired" value="Retired"> </el-option>
            </el-select>
          </el-form-item>
          <el-form-item label="Description">
            <el-input
              v-model="teamInfo.description"
              :rows="2"
              type="textarea"
              placeholder="Please input"
            />
          </el-form-item>
          <el-form-item>
            <el-button block native-type="submit" type="primary"
              >Submit User</el-button
            >
          </el-form-item>
        </el-form>
      </el-dialog>
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
import { BasicResponse } from "../backend/Response";
import {
  TeamUser,
  TeamInfo,
  TeamMember,
  getTeamList,
} from "../backend/api/Team";
import http from "@/http-common";
import { useRoute } from "vue-router";
import { getSystemStats, getTitles } from "@/backend/Generic";
export default defineComponent({
  setup() {
    const loadingTeamMembers = ref(true);
    const token = useCookie().getCookie("token");
    const team = ref<Array<TeamMember>>([]);
    const dialogVisible = ref(false);
    const teamInfo = ref<TeamInfo>({
      user: "",
      level: "",
      description: "",
    });

    const loadTeam = async () => {
      loadingTeamMembers.value = true;
      let teamValue = await getTeamList();
      loadingTeamMembers.value = false;
      if (!teamValue) return;
      team.value = teamValue;
    };
    loadTeam();
    return {
      loadingTeamMembers,
      loadTeam,
      team,
      token,
      dialogVisible,
      teamInfo,
    };
  },
  methods: {
    newUser() {
      this.teamInfo.user = "";
      this.teamInfo.description = "";
      this.teamInfo.level = "";
      this.dialogVisible = true;
    },
    updateUser(id: number) {
      for (const user of this.team) {
        if (user.id == id) {
          this.teamInfo.user = user.user.username;
          this.teamInfo.level = user.level;
          this.teamInfo.description = user.description;
          break;
        }
      }
      this.dialogVisible = true;
    },
    async teamUpdateNew(e: any) {
      e.preventDefault();
      const res = await http
        .put("/api/admin/team/add", this.teamInfo, {
          headers: {
            Authorization: "Bearer " + this.$cookie.getCookie("token"),
          },
        })
        .then((res) => {
          if (res.status != 201 && res.status != 200) {
            return;
          }
          this.dialogVisible = false;
          this.$notify({
            title: "Team Added",
          });
        })
        .catch((error) => {
          if (error.response) {
            if (error.response.status == 409) {
              this.$notify({
                title: "Username Already Exists",
                type: "warn",
              });
            } else {
              this.$notify({
                title: "Unkown Error Occured",
                type: "warn",
              });
            }
          }
        });
      this.loadTeam();
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
<style lang="scss">
.infinite-list {
  height: 300px;
  padding: 0;
  margin: 0;
  list-style: none;

  .infinite-list-item {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100px;
    margin: 10px;
    padding: 10px;
    width: 100%;
    color: var(--el-color-primary);
    & + .list-item {
      margin-top: 10px;
    }
  }
}
</style>