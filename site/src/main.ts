/* eslint-disable */
import {createApp} from "vue";
import App from "./App.vue";
import router from "./router";
import {VueCookieNext} from "vue-cookie-next";
import Notifications from "@kyvg/vue3-notification";
import naive from "naive-ui";



const app = createApp(App);
app.use(VueCookieNext);
app.use(Notifications);
app.use(router);
app.use(naive);

app.mount("#app");

// set default config
VueCookieNext.config({ expire: "7d" });