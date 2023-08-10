use std::{
    num::NonZeroUsize,
    sync:: Mutex,
};
use lazy_static::lazy_static;
use lru::LruCache;

struct Cache {
    lru: LruCache<String, String>,
} 

impl Cache {
    fn new(capacity: usize) -> Cache {
        Cache {
            lru: LruCache::new(NonZeroUsize::new(capacity).unwrap()),
        }
    }

    fn put(&mut self, path: String, active_key: String) {
        self.lru.put(path, active_key);
    }

    fn get(&mut self, path: String) -> Option<&String> {
        self.lru.get(&path)
    }
}

lazy_static! {
    static ref CACHE: Mutex<Cache> = Mutex::new(Cache::new(20));
}

pub mod cmd {
    use std::collections::HashMap;
    use super::*;
    use tauri::command;
    

    #[command]
    pub fn save_route_active_keys(active_key_map: HashMap<String, String>) {
        let mut binding = CACHE.lock().unwrap();
        for (k, v) in active_key_map {
            binding.put(k, v);
        }
    }

    #[command]
    pub fn save_route_active_key(path: String, active_key: String) {
        let mut binding = CACHE.lock().unwrap();
        binding.put(path, active_key);
    }

    #[command]
    pub fn get_route_active_key(path: String) -> String {
        let mut binding = CACHE.lock().unwrap();
        let default = "".to_string();
        binding.get(path).unwrap_or(&default).to_string()
    }
}
