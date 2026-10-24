use std::collections::HashMap;

use crate::record::Record;

#[derive(Debug)]
pub struct Data(pub HashMap<u32, Vec<u32>>);

impl Data {
    pub fn new() -> Self {
        Data(HashMap::new())
    }

    pub fn insert_record(&mut self, record: &Record) {
        let key = record.links_from();
        let value = record.links_to();

        if let Some(value_vec) = self.0.get_mut(&key) {
            value_vec.push(value);
        } else {
            self.0.insert(key, vec![value]);
        }
    }

    #[inline]
    pub fn get_by_key(&self, key: u32) -> Option<&Vec<u32>> {
        self.0.get(&key)
    }

    pub fn get_random(&self) -> Option<u32> {
        let keys: Vec<&u32> = self.0.keys().collect();
        if keys.len() == 0 {
            return None;
        }

        let random_index = rand::random_range(0..keys.len());
        let random_key = keys[random_index];

        Some(*random_key)
    }
}
