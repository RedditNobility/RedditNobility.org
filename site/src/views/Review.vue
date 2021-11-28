<template>
  <el-container direction="horizontal" style="border: 1px solid #eee">
    <el-main v-loading="loading">
      <div v-if="user != undefined">
        <h1>{{ user.name }}</h1>
        <el-button type="danger" @click="denied()">Deny</el-button>
        <el-button type="primary" @click="approve()">Approve</el-button>
        <el-tabs id="user-info" type="border-card">
          <el-tab-pane label="User">
            <div>
              <el-descriptions class="margin-top" title="User" border>
                <el-descriptions-item>
                  <template #label>
                    <el-icon><user /></el-icon>
                    Username
                  </template>
                  {{ user.name }}
                </el-descriptions-item>
                <el-descriptions-item>
                  <template #label>
                    <el-icon><user /></el-icon>
                    Discoverer
                  </template>
                  {{ user.user.discoverer }}
                </el-descriptions-item>
                <el-descriptions-item>
                  <template #label>
                    <el-icon><user /></el-icon>
                    Founded On
                  </template>
                  {{ dates.founded }}
                </el-descriptions-item>
                <el-descriptions-item v-if="titles.length > 1">
                  <template #label> Title </template>
                  <el-select v-model="title" placeholder="">
                    <el-option
                      v-for="title in titles"
                      :key="title.value"
                      :label="title.value"
                      :value="title.value"
                    >
                    </el-option>
                  </el-select>
                </el-descriptions-item>
                <el-descriptions-item>
                  <template #label>
                    <el-icon><user /></el-icon>
                    Total Karma
                  </template>
                  {{ user.total_karma }}
                </el-descriptions-item>
                <el-descriptions-item>
                  <template #label> Comment Karma </template>
                  {{ user.comment_karma }}
                </el-descriptions-item>
                <el-descriptions-item>
                  <template #label> Reddit Created on </template>
                  {{ dates.reddit_created }}
                </el-descriptions-item>
              </el-descriptions>
            </div>
          </el-tab-pane>
          <el-tab-pane label="Posts">
            <div>
              <div class="post" v-for="post in user.top_posts" :key="post.id">
                <h3>{{ post.title }}</h3>
                <el-descriptions class="margin-top" title="User" border>
                  <el-descriptions-item>
                    <template #label> Title </template>
                    {{ post.title }}
                  </el-descriptions-item>
                  <el-descriptions-item>
                    <template #label> Score </template>
                    {{ post.score }}
                  </el-descriptions-item>
                  <el-descriptions-item>
                    <template #label> Subreddit </template>
                    {{ post.subreddit }}
                  </el-descriptions-item>
                  <el-descriptions-item>
                    <template #label> ID </template>
                    <a :href="post.url"> {{ post.id }}</a>
                  </el-descriptions-item>
                  <el-descriptions-item>
                    <template #label> 18 + </template>
                    {{ post.content.over_18 }}
                  </el-descriptions-item>
                </el-descriptions>
                <div>
                  <div v-if="post.content.url != undefined">
                    {{ post.content.url }}
                  </div>
                  <div v-if="post.content.content != undefined">
                    {{ post.content.content }}
                  </div>
                </div>
              </div>
            </div>
          </el-tab-pane>
          <el-tab-pane label="Comments">
            <div>
              <div
                class="comment"
                v-for="comment in user.top_comments"
                :key="comment.id"
              >
                <el-descriptions
                  class="margin-top"
                  :title="comment.og_post_title"
                  border
                >
                  <el-descriptions-item>
                    <template #label> Score </template>
                    {{ comment.score }}
                  </el-descriptions-item>
                  <el-descriptions-item>
                    <template #label> Subreddit </template>
                    {{ comment.subreddit }}
                  </el-descriptions-item>
                  <el-descriptions-item>
                    <template #label> ID </template>
                    <a :href="comment.url"> {{ comment.id }}</a>
                  </el-descriptions-item>
                </el-descriptions>
                <div>
                  <div>
                    {{ comment.content }}
                  </div>
                </div>
              </div>
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
import { getTitles } from "@/backend/Generic";
export default defineComponent({
  setup() {
    const titles = ref<string[]>([]);
    let cookie = useCookie();
    const loading = ref(true);
    const tab = ref("User");
    const user = ref<RedditUser | undefined>(undefined);
    const dates = ref({
      founded: "",
      reddit_created: "",
    });
    const title = ref<string | undefined>(undefined);
    const route = useRoute();

    let username = route.params.username as string;

    const load = async () => {
     let value =  await getTitles();
        for (const title of value) {
          titles.value.push({ value: title });
        }
      
    };

    const getRedditUser = async () => {
      loading.value = true;
      user.value = undefined;
      try {
        let value;

        if (username != undefined && username !== "") {
          value = await reviewUserByName(cookie.getCookie("token"), username);
        } else {
          value = await reviewUser(cookie.getCookie("token"));
        }
        loading.value = false;
        if (value == undefined) {
          return;
        }
        let u = value as RedditUser;
        let date = new Date(0);
        date.setUTCSeconds(u.created);
        dates.value.reddit_created = date.toLocaleDateString("en-US");
        dates.value.founded = new Date(u.user.created).toLocaleDateString(
          "en-US"
        );
        title.value = u.user.title;
        user.value = u;
      } catch (e) {
        console.error(e);
      }
    };

    getRedditUser();
    load();
    return { loading, user, cookie, tab, dates, title, titles, getRedditUser };
  },
  methods: {
    async approve() {
      if ((this.title === "") | (this.title == undefined)) {
        this.$notify({
          title: "Please Specifiy A Title",
          type: "warn",
        });
        return;
      }
      const res = await http
        .post(
          "/api/moderator/review/" +
            this.user.user.username +
            "/Approved?title=" +
            this.title,
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
          this.$notify({
            title: "User Approved",
          });
          this.updateUser();
        })
        .catch((error) => {
          if (error.response) {
            this.$notify({
              title: "Unkown Error Occured",
              type: "warn",
            });
          }
        });
    },
    async denied() {
      const res = await http
        .post(
          "/api/moderator/review/" +
            this.user.user.username +
            "/Denied?title=" +
            this.title,
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
          this.updateUser();

          this.$notify({
            title: "User Denied",
          });
        })
        .catch((error) => {
          if (error.response) {
            this.$notify({
              title: "Unkown Error Occured",
              type: "warn",
            });
          }
        });
    },
    async updateUser() {
      this.getRedditUser();
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