<template>
  <el-container direction="horizontal" style="border: 1px solid #eee">
    <el-main>
    </el-main>
  </el-container>
</template>

<script lang="ts">
import { getTeamMap, TeamMember, Level } from "@/backend/api/Team";
import { defineComponent, ref } from "vue";
import { useCookie } from "vue-cookie-next";
export default defineComponent({
  setup() {
    const loadingTeamMembers = ref(true);
    const moderators = ref<Array<TeamMember>>([]);
    const recruiters = ref<Array<TeamMember>>([]);
    const retired = ref<Array<TeamMember>>([]);
    const loadTeam = async () => {
      loadingTeamMembers.value = true;
      let teamValue = await getTeamMap();
      loadingTeamMembers.value = false;
      if (!teamValue) return;

      moderators.value = teamValue.Moderator;
      recruiters.value = teamValue.Recruiter;
      retired.value = teamValue.Retired;
    };
    loadTeam();
    return {
      loadingTeamMembers,
      loadTeam,
      moderators,
      recruiters,
      retired,
    };
  },
});
</script>
<style scoped>
.box-card {
  width: 480px;
  margin:auto;
}
</style>
