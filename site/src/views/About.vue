<template>
  <el-container>
    <el-main>
      <div id="intro">
        <h1>A Reddit Community</h1>
        <p>
          <b>Royalty of Reddit</b>: For too long our nobility has failed to
          garner the respect and dignity it should be afforded. Some of you may
          have been scorned, others banished, others forsaken, and some of you
          have no idea what the hell I'm talking about. A once proud kingdom and
          community turned on itself due to the vision of one man, and we wish
          to make this right. We have built a new Kingdom for all of the Royal
          court of Reddit. Because you are special. You are royalty. And we
          welcome you to the community with open arms, pictures of pets, and
          friendships you can find only here. So whether ye be Philosopher or
          Jester, General or Politician, Active or Lurker: You have your crown,
          your throne, and your community. 
          <br>
          Welcome to Reddit Nobility! Be that in the newly created Subreddit, our Discord, any other community of
          your making, or just in your hearts, we sincerely hope you find some
          value in Nobility. ~
          <i>Reddit Nobility mod team</i>
        </p>
      </div>
      <div id="titles">
        <h1>Welcomed Titles</h1>

        <div
          v-for="title in titles"
          :key="title.value"
          :label="title.properName"
          :value="title.value"
          class="title"
        >
          <h2 class="titleHeader">{{ title.properName }}</h2>
          <div class="titleContent">
            <p>
              {{ title.description }}
            </p>
          </div>
        </div>
      </div>
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
import http from "@/http-common";
import { useRoute } from "vue-router";
import { getTitles, TitleElement } from "@/backend/Generic";
export default defineComponent({
  setup() {
    const titles = ref<TitleElement[]>([]);
    let cookie = useCookie();
    const loadingTitles = ref(true);
    const route = useRoute();

    const load = async () => {
      let value = await getTitles();
      if (!value) {
        return;
      }
      value.titles.sort((a, b) => a.properName.localeCompare(b.properName));
      for (const title of value.titles) {
        titles.value.push(title);
      }
    };

    load();
    return { loadingTitles, cookie, titles };
  },
});
</script>
<style>
#intro {
  width: 50%;
  margin: auto;
  border-radius: 4px;
  border: 1px solid var(--el-border-color-base);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.12), 0 0 6px rgba(0, 0, 0, 0.04)
}
#titles {
  width: 50%;
  margin: auto;
}
.title {
  padding: 5px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.12), 0 0 6px rgba(0, 0, 0, 0.04)
}
.titleHeader {
  text-align: left;
  border-bottom: 1px solid var(--el-border-color-base);
}
.titleContent {
  border-bottom: 1px solid var(--el-border-color-base);
  border-left: 1px solid var(--el-border-color-base);
  border-right: 1px solid var(--el-border-color-base);
  
}
.el-card {
  width: 50%;
  height: 500px;
  margin: auto;
}
</style>