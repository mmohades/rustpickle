use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::sync::mpsc::{channel, Receiver};
use threadpool::ThreadPool;

#[derive(Serialize, Deserialize, Debug)]
struct MapData {
    map: HashMap<String, String>,
}

#[derive(Debug)]
pub struct RustPickle {
    map_data: MapData,
    file_path: String,
    pool: ThreadPool,
}

impl MapData {
    pub fn new() -> MapData {
        MapData {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: String) -> Option<String> {
        self.map.insert(key, value)
    }

    pub fn get(&self, key: String) -> Option<String> {
        Some(String::from(self.map.get(&key)?))
    }

    pub fn serialize(&self) -> Result<Vec<u8>, String> {
        bincode::serialize(&self.map)
            .ok()
            .ok_or_else(|| "Failed to serialize the DB".to_string())
    }

    pub fn deserialize(&mut self, buffer: Vec<u8>) -> Result<(), String> {
        let deserialized: HashMap<String, String> = bincode::deserialize(&buffer)
            .ok()
            .ok_or("Failed to deserialize the DB")?;
        self.map = deserialized;
        Ok(())
    }
}

impl RustPickle {
    /**
     * Create a new RustPickle instance.
     */
    pub fn new(file_path: String) -> RustPickle {
        //using one thread to make writes in order.
        let pool = ThreadPool::new(1);
        RustPickle {
            map_data: MapData::new(),
            file_path: file_path + ".rspl",
            pool,
        }
    }

    pub fn add(&mut self, key: String, value: String) -> Option<String> {
        self.map_data.insert(key, value)
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.map_data.get(key)
    }

    pub fn read(&mut self) -> Result<(), String> {
        let filepath = self.file_path.clone();
        let mut file = File::open(filepath)
            .ok()
            .ok_or("Failed to open the file.")?;

        // read bytes
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .ok()
            .ok_or("Failed to read the content of the file.")?;

        // deserialize
        self.map_data.deserialize(buffer)?;

        Ok(())
    }

    pub fn dump(&self) -> Result<Receiver<()>, String> {
        let serialized = self.map_data.serialize()?;
        let filepath = self.file_path.clone();
        let (tx, rx) = channel();

        //add the task of writing to file to the pool to be run async
        self.pool.execute(move || {
            let mut file = File::create(filepath).unwrap();
            file.write_all(&serialized).unwrap();
            //signal the other thread if waited to complete writing.
            tx.send(())
                .expect("Could not send write completed signal on channel.");
        });

        Ok(rx)
    }
}
