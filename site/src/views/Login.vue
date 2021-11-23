<template>
  <el-container
    id="container"
    direction="horizontal"
    style="border: 1px solid #eee"
  >
    <el-main>
      <el-alert
        v-if="form.error.length != 0"
        :title="form.error"
        type="error"
      />
      <el-form
        label-position="top"
        :model="form"
        label-width="120px"
        v-on:submit="onSubmit"
      >
        <el-form-item label="Username">
          <el-input
            :disabled="type == 'otp'"
            v-model="form.username"
          ></el-input>
        </el-form-item>
        <el-form-item label="Password">
          <el-input
            v-model="this.form.password"
            placeholder="Please input password"
            show-password
          />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" native-type="submit">Log In</el-button>
        </el-form-item>
      </el-form>
      <el-button type="primary" @click="generateOTP">Generate OTP</el-button>
    </el-main>
  </el-container>
</template>

<script lang="ts">
import axios from "axios";
import { AuthToken, BasicResponse } from "@/backend/Response";
import http from "@/http-common";
import { defineComponent, ref } from "vue";
import { useRoute } from "vue-router";
import router from "@/router";
export default defineComponent({
  setup() {
    let form = ref({
      username: "",
      password: "",
      error: "",
    });
    const route = useRoute();
    let type = route.params.type as string;
    let username = route.params.username as string;
    if (type == undefined || type === "") {
      type = "classic";
    }
    if (username != undefined || username !== "") {
      form.value.username = username;
    }
    return { form, type };
  },
  methods: {
    async generateOTP() {
      let newUser = {
        username: this.form.username,
      };
      let body = JSON.stringify(newUser);
      console.log(body);
      await http
        .post("api/login/otp/create", body)
        .then((res) => {
          console.log(typeof res);
          if (res.status != 201) {
            this.$notify({
              title: "Unable to create OTP",
              type: "warn",
            });
          } else {
            router.replace({
              name: "Login",
              params: { type: "otp", username: newUser.username },
            });
          }
        })
        .catch((error) => {
          console.error(error);
          this.$notify({
            title: "Unable to create OTP",
            type: "warn",
          });
        });
    },
    async loginPassword() {
      let newUser = {
        username: this.form.username,
        password: this.form.password,
      };
      let body = JSON.stringify(newUser);
      console.log(body);
      const res = await http
        .post("api/login", body)
        .then((res) => {
          console.log(typeof res);
          if (res.status != 200) {
            return;
          }
          const result = res.data;
          let value = JSON.stringify(result);
          let response: BasicResponse<unknown> = JSON.parse(value);
          if (response.success) {
            let loginRequest = response as BasicResponse<AuthToken>;
            let date = new Date(loginRequest.data.expiration * 1000);
            this.$cookie.setCookie("token", loginRequest.data.token, {
              expire: date,
              sameSite: "lax",
            });
            location.replace("/");
          } else {
            this.form.password = "";
            this.$notify({
              title: "Invalid Username or Password",
              type: "warn",
            });
          }
        })
        .catch((error) => {
          if (error.response) {
            if (error.response.status == 401) {
              this.form.password = "";
              this.$notify({
                title: "Invalid Username or Password",
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
    async loginOTP() {
      let newUser = {
        username: this.form.username,
        otp: this.form.password,
      };
      let body = JSON.stringify(newUser);
      console.log(body);
      const res = await http
        .post("api/login/otp", body)
        .then((res) => {
          console.log(typeof res);
          if (res.status != 200) {
            return;
          }
          const result = res.data;
          let value = JSON.stringify(result);
          let response: BasicResponse<unknown> = JSON.parse(value);
          if (response.success) {
            let loginRequest = response as BasicResponse<AuthToken>;
            let date = new Date(loginRequest.data.expiration * 1000);
            this.$cookie.setCookie("token", loginRequest.data.token, {
              expire: date,
              sameSite: "lax",
            });
            location.replace("/");
          } else {
            this.form.password = "";
            this.$notify({
              title: "Invalid OTP",
              type: "warn",
            });
          }
        })
        .catch((error) => {
          if (error.response) {
            if (error.response.status == 401) {
              this.form.password = "";
              this.$notify({
                title: "Invalid Username or Password",
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
    async onSubmit(e: any) {
      e.preventDefault();
      if (this.type === "otp") {
        await this.loginOTP();
      } else {
        await this.loginPassword();
      }
    },
  },
});
</script>
<style scoped>
#container {
  margin: auto;
  width: 50%;
}
</style>