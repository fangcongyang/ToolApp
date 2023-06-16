<template>
  <div style="display: flex; flex-direction: column;height: calc(100vh - 40px);">
    <a-menu style="height: 40px;" v-model:selectedKeys="current" mode="horizontal" @click="handleClick" >
      <a-sub-menu>
        <template #icon>
          <setting-outlined />
        </template>
        <template #title>站点</template>
        <a-menu-item-group title="站点列表">
          <a-menu-item v-for="serverInfo in serverInfoDtoList" :key="serverInfo.id">{{ serverInfo.hostAddr }}</a-menu-item>
        </a-menu-item-group>
      </a-sub-menu>
      <a-menu-item key="site">
        站点管理
      </a-menu-item>
    </a-menu>
    <a-tabs
      style="display: flex; flex-direction: column; flex: 1;margin-top: 0.1rem;"
      type="editable-card"
      v-model:value="activeKey"
      @edit="onEdit"
      @change="onChange"
    >
      <a-tab-pane
        v-for="tab in tabs"
        :key="tab.key"
        :tab="tab.title"
        :closable="tab.closable"
      >
        <KeepAlive>
          <component :is="tab.component" :serverInfo="currServerInfo" :key="tab.key" ref="sshRef"/>
        </KeepAlive>
      </a-tab-pane>
    </a-tabs>
    <a-modal
      width="11rem"
      title="站点管理"
      v-model:visible="siteShow"
      :centered="true"
      :footer="null"
      :afterClose="selectSshMainDto"
    >
      <a-card title="新增站点">
        <a-form :labelCol="{span: 7,}" :model="serverInfoForm">
          <a-row :gutter="24">
            <a-col :span="8">
              <a-form-item label="IP地址" required>
                <a-input v-model:value="serverInfoForm.ipAddr" />
              </a-form-item>
            </a-col>
            <a-col :span="8">
              <a-form-item label="端口" required>
                <a-input v-model:value="serverInfoForm.port" />
              </a-form-item>
            </a-col>
            <a-col :span="8">
              <a-form-item label="用户名" required>
                <a-input v-model:value="serverInfoForm.username" />
              </a-form-item>
            </a-col>
            <a-col :span="8">
              <a-form-item label="密码" required>
                <a-input v-model:value="serverInfoForm.password" />
              </a-form-item>
            </a-col>
            <a-col :span="16">
              <a-form-item >
                <a-button type="primary" @click="onSubmit">提交</a-button>
              </a-form-item>
            </a-col>
          </a-row>
        </a-form>
      </a-card>
      <a-card title="站点列表">
        <a-table :dataSource="serverInfoList" :columns="columns" :pagination="false">
          <template  #bodyCell="{ column, record }">
            <template v-if="column.key === 'action'">
              <span>
                <a @click="editSshMain(record)">编辑</a>
                <a-divider type="vertical" />
                <a @click="delSelectSshMain(record.id)">删除</a>
              </span>
            </template>
          </template>
        </a-table>
      </a-card>
    </a-modal>
  </div>
</template>
  
<script lang="ts">
  import { defineComponent, toRefs, ref, reactive, onMounted, UnwrapRef, toRaw  } from 'vue';
  import { useRouter } from 'vue-router';
  import type { MenuProps } from 'ant-design-vue';
  import { invoke } from "@tauri-apps/api/tauri";
  import Ssh from "./Ssh.vue";
  import { Tab, ServerInfo, ServerInfoDto } from './webssh';

  const columns = [
    {
      title: 'IP地址',
      dataIndex: 'ipAddr',
      key: 'ipAddr',
    },
    {
      title: '端口',
      dataIndex: 'port',
      key: 'port',
    },
    {
      title: '用户名',
      key: 'username',
      dataIndex: 'username',
    },
    {
      title: '密码',
      key: 'password',
      dataIndex: 'password',
    },
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
      const current = ref("");
      
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
        let serverInfo = serverMap.value.get(key);
        currServerInfo.value = serverInfo!;
        tabs.value.push({
          key: crypto.randomUUID(),
          title: serverInfo!.hostAddr,
          closable: true,
          component: `ssh`, // 绑定新的组件名
        });
        state.activeKey = serverInfo!.id;
      };
      
      const handleClick: MenuProps['onClick'] = menuInfo => {
        switch (menuInfo.key.toString()) {
          case "site":
            selectSshMain();
            state.siteShow = true;
            break;
          case "/":
            router.push({ path: menuInfo.key.toString(), });
            break;
          default:
            addTab(menuInfo.key.toString());
        }
      };

      const removeTab = (targetKey: string) => {
        let lastIndex = 0;
        tabs.value.forEach((tab, i) => {
          if (tab.key === targetKey) {
            lastIndex = i - 1;
          }
        });
        const newTabs = tabs.value.filter((tab) => tab.key !== targetKey);
        tabs.value = newTabs;
        if (newTabs.length && state.activeKey === targetKey) {
          if (lastIndex >= 0) {
            state.activeKey = newTabs[lastIndex].key;
          } else {
            state.activeKey = newTabs[0].key;
          }
        }
      };

      const onEdit = (targetKey: any, action: string) => {
        if (action === "remove") {
          removeTab(targetKey);
        }
      };

      const onChange = (key: string) => {
        state.activeKey = key;
        tabs.value.forEach((tab, i) => {
          if (tab.key === key) {
            let arr:Array<InstanceType<typeof Ssh>> = sshRef.value!;
            arr[i].onResize();
          }
        });
      };

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
        onEdit,
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
  //
  </script>
<style lang="css">
  .ant-tabs-content {
    height: 100% !important;
  }
</style>
  
  
  