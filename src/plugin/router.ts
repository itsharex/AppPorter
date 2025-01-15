import { useInstallationConfigStore } from "@/stores/installation_config";
import { useConfirm } from "primevue/useconfirm";
import type { Router } from "vue-router";
import { createMemoryHistory, createRouter } from "vue-router";

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
  router.beforeEach(async (to, from) => {
    // Skip guard if both routes are the same
    if (to.path === from.path) {
      return true;
    }

    const installationConfig = useInstallationConfigStore();

    // Only show confirmation when leaving installation option page to a different page
    if (
      from.name === "installation-option" &&
      to.name !== "installation-option"
    ) {
      const confirm = useConfirm();
      try {
        await new Promise((resolve, reject) => {
          confirm.require({
            message:
              "Are you sure you want to leave? All changes will be lost.",
            header: "Confirm",
            icon: "material-symbols-rounded text-2xl warning", // Changed from text-xl to text-2xl
            acceptIcon: "material-symbols-rounded text-lg logout", // Changed from text-base to text-lg
            rejectIcon: "material-symbols-rounded text-lg close", // Changed from text-base to text-lg
            acceptLabel: "Leave",
            rejectLabel: "Cancel",
            rejectClass: "p-button-outlined p-button-secondary",
            accept: () => resolve(true),
            reject: () => reject(),
          });
        });
      } catch {
        return false;
      }
    }

    // Clear data based on route
    if (to.name === "installation") {
      installationConfig.$reset();
    } else if (to.name === "installation-option") {
      const zipPath = installationConfig.zip_path;
      installationConfig.$reset();
      installationConfig.zip_path = zipPath;
    } else if (to.name === "installation-progress") {
      return true;
    } else if (to.name === "settings") {
      return true;
    }
    return true;
  });
}

export function goTo(path: string) {
  router.push(path);
}

export default router;
