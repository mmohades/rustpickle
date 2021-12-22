use std::collections::HashMap;

pub struct RustPickle {
    map: HashMap<String, String>,
    file_path: String,
}

impl RustPickle {
    /**
     * Create a new RustPickle instance.
     */
    pub fn new(file_path: String) -> RustPickle {
        RustPickle {
            map: HashMap::new(),
            file_path,
        }
    }

    pub fn add(&mut self, key: String, value: String) -> Option<String> {
        self.map.insert(key, value)
    }

    pub fn get(self, key: String) -> Option<String> {
        Some(String::from(self.map.get(&key)?))
    }

    pub fn read(&mut self) -> Result<String, String> {
        todo!()
    }

    pub fn dump(&mut self) -> Result<String, String> {
        todo!()
    }
}

pub fn testing() {
    println!("Testing init");
}
