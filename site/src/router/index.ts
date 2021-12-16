import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
import Home from "../views/Home.vue";
import Install from "../views/Install.vue";
import Login from "../views/Login.vue";
import Admin from "../views/Admin.vue";
import Review from "../views/Review.vue";
import ModifyUser from "../views/ModifyUser.vue";
import Moderator from "../views/Moderator.vue";
import Me from "../views/Me.vue";
import About from "../views/About.vue";
const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "Home",
    component: Home,
  },
  {
    path: "/install",
    name: "Install",
    component: Install,

  },
  {
    path: "/login/:type?/:username?",
    name: "Login",
    component: Login,

  },
  {
    path: "/admin",
    name: "Admin",
    component: Admin,

  }, {
    path: "/user/:username",
    name: "User",
    component: ModifyUser,

  }, {
    path: "/moderator",
    name: "Moderator",
    component: Moderator,

  }, {
    path: "/review/:username?",
    name: "Review",
    component: Review,

  }, {
    path: "/me",
    name: "Me",
    component: Me,

  }, {
    path: "/about",
    name: "About",
    component: About,

  },
];

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
});

export default router;
