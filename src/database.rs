use std::collections::HashMap;
use std::ops::Add;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct RedisDatabaseEntry {
    value: String,
    expire: Option<Instant>,
}

#[derive(Debug)]
pub struct RedisDatabase {
    content: HashMap<String, RedisDatabaseEntry>,
}

impl RedisDatabase {
    pub fn new() -> Self {
        return RedisDatabase {
            content: HashMap::new(),
        };
    }

    pub fn insert(&mut self, key: &str, value: &str, expiration: Option<u64>) -> Option<String> {
        let expiration_time = expiration.map(|t| Instant::now().add(Duration::from_millis(t)));

        let entry = RedisDatabaseEntry {
            value: value.to_owned(),
            expire: expiration_time,
        };
        self.content.insert(key.to_owned(), entry).map(|e| e.value)
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        let entry = self.content.get(key).and_then(|entry| {
            let now = Instant::now();

            if entry.expire.is_some_and(|expire| expire < now) {
                None
            } else {
                Some(&entry.value)
            }
        });

        return entry;
    }
}
