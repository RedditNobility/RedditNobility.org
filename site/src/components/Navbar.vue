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
      @click="dialogVisible = true"
      >Submit</el-menu-item
    >
    <el-menu-item
      v-if="user.permissions.review_user"
      index="Review Users"
      @click="router.push('/review')"
      >Review Users</el-menu-item
    >
    <el-menu-item
      v-if="user.permissions.moderator"
      index="Moderator"
      @click="router.push('/moderator')"
      >Moderator</el-menu-item
    >
    <el-menu-item
      v-if="user.permissions.admin"
      index="Admin"
      @click="router.push('/admin')"
      >Admin</el-menu-item
    >
    <el-menu-item
      v-if="user.permissions.login"
      index="Me"
      @click="router.push('/me')"
      >Me</el-menu-item
    >
  </el-menu>
  <el-dialog v-model="dialogVisible" title="Submit a New User">
    <el-form
      :model="form"
      label-position="top"
      label-width="120px"
      v-on:submit="onSubmit"
    >
      <el-form-item label="Username">
        <el-input v-model="form.username"></el-input>
      </el-form-item>
      <el-form-item>
        <el-button block native-type="submit" type="primary"
          >Submit User</el-button
        >
      </el-form-item>
    </el-form>
  </el-dialog>
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
    let form = ref({
      username: "",
    });
    const activeIndex = ref(router.currentRoute.value.name);
    const dialogVisible = ref(false);
    return { activeIndex, router, dialogVisible, form };
  },
  methods: {
    async onSubmit(e: any) {
      e.preventDefault();
      const res = await http
        .post(
          "api/submit/" + this.form.username,
          {},
          {
            headers: {
              Authorization: "Bearer " + this.$cookie.getCookie("token"),
            },
          }
        )
        .then((res) => {
          if (res.status != 201 && res.status != 200) {
            return;
          }
          this.form.username = "";
          this.$notify({
            title: "User Submitted",
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
    },
  },
});
</script>