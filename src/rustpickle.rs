use bincode;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

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
        let filepath = self.file_path.clone();
        let mut file = File::open(filepath).unwrap();

        // read bytes
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        // deserialize
        let deserialized: HashMap<String, String> = bincode::deserialize(&buffer).unwrap();

        self.map = deserialized;
        Ok(String::from("Success"))
    }

    pub fn dump(&mut self) -> Result<String, String> {
        let serialized = bincode::serialize(&self.map).unwrap();

        let filepath = self.file_path.clone();
        let mut file = File::create(filepath).unwrap();

        file.write_all(&serialized).unwrap();

        Ok(String::from("success"))
    }
}

pub fn testing() {
    println!("Testing init");
}
