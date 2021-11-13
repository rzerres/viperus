use std::collections::hash_map::Drain;
use std::collections::HashMap;

pub mod map_value;

/// Viperus values
pub use map_value::ViperusValue;

#[derive(Debug)]
/// Hashmap that assignes a string to a viperus value.
pub struct Map {
    data: HashMap<String, ViperusValue>,
}

impl Default for Map {
    fn default() -> Self {
        Map::new()
    }
}

impl Map {
    /// Creates a new viperius hashmap instance.
    pub fn new() -> Self {
        Map {
            data: HashMap::new(),
        }
    }

    /// Drains the values inside a viperius data structure.
    pub fn drain<'a>(&'a mut self) -> Drain<'a, String, ViperusValue> {
        self.data.drain()
    }

    /// Add a value to the viperius data structure.
    pub fn add<T>(&mut self, key: &str, value: T) -> Option<T>
    where
        ViperusValue: From<T>,
        ViperusValue: Into<T>,
    {
        self.data.insert(key.to_string(), ViperusValue::from(value)).map(|mv| mv.into())
    }

    /// Add a value to a viperius data structure key.
    pub fn add_value(&mut self, key: &str, value: ViperusValue) -> Option<ViperusValue> {
        self.data.insert(key.to_string(), value)
        //     let path: Vec<&str>=key.to_lowercase().split(".").collect();
        //     let pathLen = path.len();
        //    for pi  in 0..pathLen-1 {
        //        let v = self.data.get(path[pi]);
        //        if let None = v {
        //            let node=

        //        }

        //    }

        //     todo!("imlp add a key to the map")
    }

    /// Get a value from a viperius data structure key.
    pub fn get_value(&self, key: &str) -> Option<&ViperusValue> {
        self.data.get(key)
    }

    /// Get values from a viperius data structure.
    pub fn get<'a, 'b, 'c, T>(&'a self, key: &'a str) -> Option<T>
    where
        ViperusValue: From<T>,
        &'c ViperusValue: Into<T>,
        ViperusValue: Into<T>,
    {
        self.data.get(key).map(|mv| (*mv).clone().into())
    }

    /// Merge a value into a viperius data structure key.
    pub fn merge(&mut self, src: &Map) {
        for (k, v) in &src.data {
            self.add_value(k, v.clone());
        }
    }
}
