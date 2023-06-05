use std::sync::{Arc, Mutex};
use std::collections::HashMap;
#[allow(dead_code)]
pub mod database {
    use super::*;

    pub struct Database {
        pub data: Arc<Mutex<HashMap<String, String>>>
    }

    impl Database {
        pub fn new() -> Database {
            Database {
                data: Arc::new(Mutex::new(HashMap::new()))
            }
        }

        pub async fn insert(&self, key: String, value: String) {
            let mut data = self.data.lock().unwrap();
            data.insert(key, value);
        }

        pub async fn delete(&self, key: String) {
            let mut data = self.data.lock().unwrap();
            data.remove(&key);
        }

        pub async fn update(&self, key: String, value: String) {
            let mut data = self.data.lock().unwrap();
            if let Some(entry) = data.get_mut(&key) {
                *entry = value;
            }
        }
    }
}
