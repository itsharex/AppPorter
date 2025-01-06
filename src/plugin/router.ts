import { createMemoryHistory, createRouter } from "vue-router";
import { useInstallationConfigStore } from "@/stores/installation_config";
import type { Router } from "vue-router";

import Installation from "@/Installation.vue";
import Installation_Option from "@/Installation/Option.vue";
import Installation_Progress from "@/Installation/Progress.vue";
import Settings from "@/Settings.vue";

const routes = [
  { path: "/", redirect: "/Installation", name: "root" }, // Add root path redirect
  { path: "/Installation", component: Installation, name: "installation" },
  {
    path: "/Installation/Option",
    component: Installation_Option,
    name: "installation-option",
  },
  {
    path: "/Installation/Progress",
    component: Installation_Progress,
    name: "installation-progress",
  },
  { path: "/Settings", component: Settings, name: "settings" },
] as const;

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

// Move the navigation guard setup to a separate function
export function setupRouterGuards(router: Router) {
  router.beforeEach((to) => {
    const installationConfig = useInstallationConfigStore();

    switch (to.path) {
      case "/Installation":
        installationConfig.$reset();
        break;
      case "/Installation/Option":
        // Keep zip_path but reset other fields
        break;
      case "/Installation/Progress":
        // Keep installation config
        break;
      case "/Settings":
        // No need to clear anything
        break;
    }
    return true;
  });
}

export function goTo(path: string) {
  router.push(path);
}

export default router;
