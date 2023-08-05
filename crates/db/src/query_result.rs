use std::{collections::HashMap, str::FromStr};

// query.rs
pub struct QueryResult {
    data: HashMap<String, String>,
}

impl QueryResult {
    pub fn create() -> Self {
        return QueryResult { data: HashMap::new() };
    }

    pub fn set(&mut self, k: &str, v: &str) {
        self.data.insert(
            k.to_string(),
            v.to_string(),
        );
    }

    pub fn get(&self, k: &str) -> Option<String> {
        if (self.data.contains_key(
            &k.to_string()
        )) {
            return Some(self.data.get(&k.to_string()).unwrap().clone());
        }

        return None;
    }
}
