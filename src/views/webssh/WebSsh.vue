<template>
  <div class="main-content">
    <el-menu
      :default-active="current"
      mode="horizontal"
      :ellipsis="false"
      @select="handleClick"
    >
      <el-menu-item index="_logo">LOGO</el-menu-item>
      <div class="flex-grow" />
      <el-sub-menu index="_site-menu">
        <template #title>站点</template>
        <el-menu-item-group title="站点列表">
          <el-menu-item v-for="serverInfo in serverInfoDtoList" :index="serverInfo.id">{{ serverInfo.hostAddr }}</el-menu-item>
        </el-menu-item-group>
      </el-sub-menu>
      <el-menu-item index="site">站点管理</el-menu-item>
    </el-menu>
    <div class="menu-content">
      <div class="part">
        <div class="part-content">
          <el-tabs
            v-model="activeKey"
            closable
            @tab-remove="removeTab"
            @change="onChange"
          >
            <el-tab-pane
              v-for="tab in tabs"
              :key="tab.key"
              :label="tab.title"
              :name="tab.key"
              :closable="tab.closable"
            >
              <div class="pane-item">
                <KeepAlive>
                  <component :is="tab.component" :serverInfo="currServerInfo" :key="tab.key" ref="sshRef"/>
                </KeepAlive>
              </div>
            </el-tab-pane>
          </el-tabs>
        </div>
      </div>
    </div>
    <el-dialog
      width="10rem"
      title="站点管理"
      v-model="siteShow"
      :afterClose="selectSshMainDto"
    >
      <el-card header="新增站点">
        <el-form :labelCol="{span: 7,}" :model="serverInfoForm">
          <el-row :gutter="24">
            <el-col :span="8">
              <el-form-item label="IP地址" required>
                <el-input v-model:value="serverInfoForm.ipAddr" />
              </el-form-item>
            </el-col>
            <el-col :span="8">
              <el-form-item label="端口" required>
                <el-input v-model:value="serverInfoForm.port" />
              </el-form-item>
            </el-col>
            <el-col :span="8">
              <el-form-item label="用户名" required>
                <el-input v-model:value="serverInfoForm.username" />
              </el-form-item>
            </el-col>
            <el-col :span="8">
              <el-form-item label="密码" required>
                <el-input v-model:value="serverInfoForm.password" />
              </el-form-item>
            </el-col>
            <el-col :span="16">
              <el-form-item >
                <el-button type="primary" @click="onSubmit">提交</el-button>
              </el-form-item>
            </el-col>
          </el-row>
        </el-form>
      </el-card>
      <el-card header="站点列表">
        <el-table :data="serverInfoList" :columns="columns" :pagination="false">
          <el-table-column prop="ipAddr" label="IP地址" column-key="ipAddr"/>
          <el-table-column prop="port" label="端口" column-key="port"/>
          <el-table-column prop="username" label="用户名" column-key="username"/>
          <el-table-column prop="password" label="密码" column-key="password"/>
          <el-table-column label="操作">
            <template #default="scope">
              <span>
                <a @click="editSshMain(scope.record)">编辑</a>
                <el-divider direction="vertical" />
                <a @click="delSelectSshMain(scope.record.id)">删除</a>
              </span>
            </template>
          </el-table-column>
        </el-table>
      </el-card>
    </el-dialog>
  </div>
</template>
  
<script lang="ts">
  import { defineComponent, toRefs, ref, reactive, onMounted, UnwrapRef, toRaw, onBeforeMount  } from 'vue';
  import { useRouter } from 'vue-router';
  import { ElMessage } from "element-plus";
  import { invoke } from "@tauri-apps/api/tauri";
  import Ssh from "./Ssh.vue";
  import { Tab, ServerInfo, ServerInfoDto } from './webssh';
  import { useCoreStore } from "../../store";
  import { storeToRefs } from 'pinia';

  const columns = [
    {
      title: '操作',
      key: 'action',
    },
  ];
  export default defineComponent({
    components: {
      Ssh
    },
    setup() {
      const router = useRouter();
      const coreStore = useCoreStore();
      const { initWebssh } = coreStore;
      const { init } = storeToRefs(coreStore);

      const serverInfoForm: UnwrapRef<ServerInfo> = reactive({
        id: "",
        ipAddr: "",
        port: "",
        username: "",
        password: "",
        authModel: 1
      });

      const state = reactive({
        activeKey: "",
        siteShow: false,
      });
      
      const tabs = ref<Array<Tab>>([]);
      const serverInfoDtoList = ref<Array<ServerInfoDto>>([]);
      const serverInfoList = ref<Array<ServerInfo>>([]);
      const serverMap = ref<Map<string, ServerInfoDto>>(new Map());
      const currServerInfo = ref<ServerInfoDto | null>(null);
      const sshRef = ref<Array<InstanceType<typeof Ssh>>>([]);
      const current = ref<String>("");
      
      async function selectSshMainDto() {
        let serverInfoListStr:string = await invoke("select_ssh_main_dto", {});
        serverInfoDtoList.value = JSON.parse(serverInfoListStr);
        for (let serverInfo of serverInfoDtoList.value) {
          serverMap.value.set(serverInfo.id, serverInfo);
        }
      }

      async function selectSshMain() {
        let serverInfoListStr:string = await invoke("select_ssh_main", {});
        serverInfoList.value = JSON.parse(serverInfoListStr);
      }

      const addTab = (key: string) => {
        if (!init.value.websshInit) {
          ElMessage.warning('webssh后端未启动，请重启服务');
          return
        }
        let serverInfo = serverMap.value.get(key);
        currServerInfo.value = serverInfo!;
        const tabKey = crypto.randomUUID();
        tabs.value.push({
          key: tabKey,
          title: serverInfo!.hostAddr,
          closable: true,
          component: `ssh`, // 绑定新的组件名
        });
        state.activeKey = tabKey;
      };
      
      const handleClick = (key: string, keyPath: string[]) => {
        switch (key) {
          case "site":
            selectSshMain();
            state.siteShow = true;
            break;
          case "/":
            router.push({ path: key, });
            break;
          case "_logo":
          case "_site-menu":
            break;
          default:
            addTab(key);
        }
      };

      const removeTab = (targetName: string) => {
        let lastIndex = 0;
        tabs.value.forEach((tab, i) => {
          if (tab.key === targetName) {
            lastIndex = i - 1;
          }
        });
        const newTabs = tabs.value.filter((tab) => tab.key !== targetName);
        tabs.value = newTabs;
        if (newTabs.length && state.activeKey === targetName) {
          if (lastIndex >= 0) {
            state.activeKey = newTabs[lastIndex].key;
          } else {
            state.activeKey = newTabs[0].key;
          }
        }
      };

      const onChange = (key: string) => {
        state.activeKey = key;
        tabs.value.forEach((tab, i) => {
          if (tab.key === key) {
            let arr:Array<InstanceType<typeof Ssh>> = sshRef.value!;
            if (arr[i]) {
              arr[i].onResize();
              arr[i].resizeRemoteTerminal();
            }
          }
        });
      };
      
      onBeforeMount(() => {
        initWebssh();
      })

      onMounted(() => {
        selectSshMainDto();
      })
      
      async function saveSelectSshMain(serverInfo: ServerInfo) {
        await invoke("save_ssh", { sshMainJson: JSON.stringify(serverInfo)});
        selectSshMain();
        Object.keys(serverInfoForm).forEach(key => {
          if (key !== "authModel") {
            serverInfoForm[key] = "";
          } else {
            serverInfoForm[key] = 1;
          }
        });
      }

      const onSubmit = () => {
        saveSelectSshMain(toRaw(serverInfoForm))
      };

      async function delSelectSshMain(id: string) {
        await invoke("del_ssh", { id: id});
        selectSshMain();
      }

      const editSshMain = (record: ServerInfo) => {
        Object.keys(serverInfoForm).forEach(key =>  serverInfoForm[key] = record[key]);
      }

      return {
        ...toRefs(state),
        removeTab,
        onChange,
        current,
        handleClick,
        tabs,
        serverInfoDtoList,
        serverInfoList,
        currServerInfo,
        sshRef,
        columns,
        serverInfoForm,
        onSubmit,
        delSelectSshMain,
        editSshMain,
        selectSshMainDto,
      }
    }
  });
  </script>
  
  
  