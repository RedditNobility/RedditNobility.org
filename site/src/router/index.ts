import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
import Home from "../views/Home.vue";
import Install from "../views/Install.vue";
import Login from "../views/Login.vue";
import Admin from "../views/Admin.vue";
import Review from "../views/Review.vue";
import ModifyUser from "../views/ModifyUser.vue";
import Moderator from "../views/Moderator.vue";
const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "Home",
    component: Home,
  },
  {
    path: "/about",
    name: "About",
    // route level code-splitting
    // this generates a separate chunk (about.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    component: () =>
      import(/* webpackChunkName: "about" */ "../views/About.vue"),
  }, {
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

  },  {
    path: "/moderator",
    name: "Moderator",
    component: Moderator,

  }, {
    path: "/review",
    name: "Review",
    component: Review,

  },
];

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
});

export default router;
