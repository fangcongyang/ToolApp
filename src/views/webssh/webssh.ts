export interface Tab {
    key: string;
    title: string;
    closable: boolean;
    component: string;
  }

export interface ServerInfoDto {
    id: string;
    hostAddr: string;
    username: string,
    password: string,
    authModel: 1
}

export interface ServerInfo {
  id: string;
  ipAddr: string;
  port: string;
  username: string,
  password: string,
  authModel: 1
  [key: string]: string | number;
}