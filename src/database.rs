use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Debug)]
pub struct RedisDatabase {
    content: HashMap<String,String>
}

impl RedisDatabase {
    pub fn new() -> Self {
        return RedisDatabase { content: HashMap::new() }
    }


    pub fn insert (&mut self, key: &str, value: &str) -> Option<String> {
        self.content.insert(key.to_owned(), value.to_owned())
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.content.get(key)
    }
}