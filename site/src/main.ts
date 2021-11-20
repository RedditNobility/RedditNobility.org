/* eslint-disable */
import {createApp} from "vue";
import App from "./App.vue";
import router from "./router";
import {VueCookieNext} from "vue-cookie-next";
import Notifications from "@kyvg/vue3-notification";



const app = createApp(App);
app.use(VueCookieNext);
app.use(Notifications);
app.use(router);

app.mount("#app");

// set default config
VueCookieNext.config({ expire: "7d" });