use std::collections::HashMap;

use crate::record::Record;

pub struct Data(HashMap<u32, Vec<u32>>);

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
}
