import "@/assets/index.css";
import Main from "@/Main.vue";
import i18n from "@/plugin/i18n";
import router, { setupRouterGuards } from "@/plugin/router";
import { useSettingsStore } from "@/stores/settings";
import Aura from "@primevue/themes/aura";
import { defaultWindowIcon } from "@tauri-apps/api/app";
import { Menu } from "@tauri-apps/api/menu";
import { TrayIcon, TrayIconEvent } from "@tauri-apps/api/tray";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { exit } from "@tauri-apps/plugin-process";
import { createPinia } from "pinia";
import PrimeVue from "primevue/config";
import ConfirmationService from "primevue/confirmationservice";
import { createApp, Ref, ref } from "vue";

export const window = await getCurrentWindow();
const icon = (await defaultWindowIcon()) || "src-tauri\\icons\\icon.ico";
const menu = await Menu.new({
  items: [
    {
      id: "open",
      text: "Open",
      action: () => {
        window.show();
        window.unminimize();
        window.setFocus();
      },
    },
    {
      id: "quit",
      text: "Quit",
      action: () => {
        exit(0);
      },
    },
  ],
});
const options = {
  icon,
  menu,
  menuOnLeftClick: false,
  action: (event: TrayIconEvent) => {
    switch (event.type) {
      case "Click":
        if (event.button == "Left" && event.buttonState == "Down") {
          window.show();
          window.unminimize();
          window.setFocus();
        }
        break;
    }
  },
};

await window.onCloseRequested(async () => {
  window.hide();
});

const pinia = createPinia();
const app = createApp(Main);

async function loadSvg(name: string): Promise<string> {
  const path = `/src/assets/icons/${name}.svg`;
  try {
    const response = await fetch(path);
    return await response.text();
  } catch (error) {
    console.error("Failed to load SVG:", error);
    return "";
  }
}

app
  .use(pinia) // First initialize pinia
  .use(router) // Then initialize router
  .use(PrimeVue, {
    theme: {
      preset: Aura,
      options: {
        prefix: "p",
        darkModeSelector: "system",
        cssLayer: false,
      },
    },
  })
  .use(ConfirmationService); // Add this line

setupRouterGuards(router); // Finally setup router guards

app.use(i18n).directive("svg", {
  async mounted(el, binding) {
    const svgContent = await loadSvg(binding.value);
    el.innerHTML = svgContent;
  },
  async updated(el, binding) {
    const svgContent = await loadSvg(binding.value);
    el.innerHTML = svgContent;
  },
});

export const error: Ref<string[]> = ref([]);

const settingsStore = useSettingsStore();
await settingsStore.loadSettings();
if (settingsStore.minimize_to_tray_on_close) {
  try {
    await TrayIcon.new(options);
  } catch (err) {
    console.error("Failed to create tray icon:", err);
    error.value.push(`Failed to create tray icon: ${err}`);
  }
}

app.mount("#app");
