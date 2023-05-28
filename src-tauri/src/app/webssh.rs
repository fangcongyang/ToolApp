use crate::utils;
use std::{
    num::NonZeroUsize,
    sync:: Mutex,
    process::Command,
    thread
};
use rusqlite::Connection;
use lazy_static::lazy_static;
use lru::LruCache;
use serde::{Serialize, Deserialize};

pub const DBNAME: &str = "tauri";

// 定义一个缓存结构体，包含一个lru缓存和一个容量
struct Cache {
    lru: LruCache<String, Connection>,
    capacity: usize,
} 

// 实现缓存结构体的方法
impl Cache {
    // 创建一个新的缓存，指定容量
    fn new(capacity: usize) -> Cache {
        Cache {
            lru: LruCache::new(NonZeroUsize::new(capacity).unwrap()),
            capacity,
        }
    }

    // 向缓存中插入一个数据库连接，如果缓存已满，淘汰最久未使用的对象
    fn put(&mut self, id: String, channel: Connection) {
        self.lru.put(id, channel);
    }

    // 从缓存中获取一个数据库连接，如果存在，返回Some，否则返回None，并更新最近使用时间
    fn get(&mut self, id: String) -> Option<&Connection> {
        self.lru.get(&id)
    }

    fn get_size(&mut self) -> usize {
        self.capacity
    }
}

lazy_static! {
    static ref CACHE: Mutex<Cache> = Mutex::new(Cache::new(1));
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SshMain {
    id: String,
    ipAddr: String,
    port: String,
    username: String,
    password: String,
    authModel: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct SshMainDto {
    id: String,
    hostAddr: String,
    username: String,
    password: String,
    authModel: i32,
}

pub fn init() {
    // 创建一个新线程, 启动后端
    thread::spawn(move || {
        // 创建一个 Command 对象，传入 exe 文件的路径和可选的参数
        let mut path = utils::app_install_root();
        path.pop();
        let server_close = Command::new(path.join("expand").join("ServerClose.exe"))
          .args(["webssh.exe"])
          .output()
          .expect("执行关闭应用线程命令!");
        // 检查子进程是否成功
        if server_close.status.success() {
            // 打印子进程的标准输出
            let s = String::from_utf8_lossy(&server_close.stdout);
            println!("关闭应用成功:{}", s);
        } else {
            // 打印子进程的标准错误
            let s = String::from_utf8_lossy(&server_close.stderr);
            println!("关闭应用失败:{}", s);
        }

        let output = Command::new(path.join("expand").join("webssh.exe"))
          .output()
          .expect("启动webssh后端失败!");
    
        // 检查子进程是否成功
        if output.status.success() {
            // 打印子进程的标准输出
            let s = String::from_utf8_lossy(&output.stdout);
            println!("启动webssh后端成功:{}", s);
        } else {
            // 打印子进程的标准错误
            let s = String::from_utf8_lossy(&output.stderr);
            println!("启动webssh后端失败:{}", s);
        }
      });
    let mut path = utils::app_install_root();
    path.pop();
    let output = path.join("data").join("tauri.db");
    let webssh_db = Connection::open(output).unwrap();
    CACHE.lock().unwrap().put(DBNAME.into(), webssh_db);
}


pub mod cmd {
    use super::*;
    use rusqlite::{params};
    use tauri::command;
    use uuid::Uuid;

    #[command]
    pub fn select_ssh_main() -> String {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        
        let mut stmt = conn.prepare("SELECT * FROM ssh_main").unwrap();
        let sshs = stmt.query_map([], |row| {
            Ok(SshMain {
                id: row.get(0)?,
                ipAddr: row.get(1)?,
                port: row.get(2)?,
                username: row.get(3)?,
                password: row.get(4)?,
                authModel: row.get(5)?
            })
        }).unwrap();
        
        let mut ssh_list = Vec::new();
        for ssh in sshs {
            let s = ssh.unwrap();
            ssh_list.push(s);
        }
        
        let json = serde_json::to_string(&ssh_list).unwrap();
        return json;
    }

    #[command]
    pub fn select_ssh_main_dto() -> String {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        
        let mut stmt = conn.prepare("SELECT * FROM ssh_main").unwrap();
        let sshs = stmt.query_map([], |row| {
            Ok(SshMain {
                id: row.get(0)?,
                ipAddr: row.get(1)?,
                port: row.get(2)?,
                username: row.get(3)?,
                password: row.get(4)?,
                authModel: row.get(5)?
            })
        }).unwrap();
        
        let mut ssh_list = Vec::new();
        for ssh in sshs {
            let s = ssh.unwrap();
            let mut hostAddr = s.ipAddr.to_string();
            hostAddr.push(':');
            hostAddr.push_str(&s.port);
            let ssh_dto = SshMainDto {
                id: s.id,
                hostAddr: hostAddr,
                username: s.username,
                password: s.password,
                authModel: s.authModel,
            };
            ssh_list.push(ssh_dto);
        }
        
        let json = serde_json::to_string(&ssh_list).unwrap();
        return json;
    }

    #[command]
    pub fn save_ssh(ssh_main_json: String) {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        let mut ssh_main: SshMain = serde_json::from_str(&ssh_main_json).unwrap();
        if ssh_main.id == "" {
            let id: String = Uuid::new_v4().to_string();
            ssh_main.id = id.replace("-", "");
            
            conn.execute(
                "INSERT INTO ssh_main (id, ip_addr, port, username, password) VALUES (?1, ?2, ?3, ?4, ?5)",
                (&ssh_main.id, &ssh_main.ipAddr, &ssh_main.port, &ssh_main.username, &ssh_main.password),
            ).unwrap();
        } else {
            conn.execute(
                "UPDATE ssh_main SET ip_addr = ?1, port = ?2, username = ?3, password = ?4 WHERE id = ?5",
                (&ssh_main.ipAddr, &ssh_main.port, &ssh_main.username, &ssh_main.password, &ssh_main.id, ),
            ).unwrap();
        }
    }

    #[command]
    pub fn del_ssh(id: String) {
        let mut binding = CACHE.lock().unwrap();
        let conn = binding.get(DBNAME.into()).unwrap();
        conn.execute("DELETE FROM ssh_main WHERE id = ?1", params![&id]).unwrap();
    }
}
