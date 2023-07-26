import { defineStore } from 'pinia';
import { invoke } from "@tauri-apps/api/tauri";

export const useCoreStore = defineStore('core', {
    state: () => {
        return {
            init: {
                systemConfInit: false,
                websshInit: false,
            },
            systemConf: {
                theme: "theme-light",
                saveWindowState: false,
                mainWidth: 1080.0,
                mainHeight: 720.0,
            },
            homeConf: {
                activeMenu: "developer",
            }
        }
    },
    actions: {

        async refreshSystemConf() {
          let systemConfStr: string = await invoke("get_conf_by_name", { confName: "systemConf" });
          const systemConf = JSON.parse(systemConfStr);
          this.systemConf = systemConf;
        },
  
        async getSystemConf() {
          if (!this.init.systemConfInit) {
            await this.refreshSystemConf();
            this.init.systemConfInit = true;
          }
        },

        async initWebssh() {
            if (!this.init.websshInit) {
                await invoke("close_webssh", {});
                await invoke("init_webssh", {});
                this.init.websshInit = true;
            }
        }
    }
});