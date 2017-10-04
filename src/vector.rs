use std::collections::BTreeMap;

use serde::ser::{Serialize, Serializer, SerializeSeq};

#[derive(Debug)]
pub struct Vector {
    elements: BTreeMap<u32, f64>,
}

impl Vector {
    pub fn new() -> Vector {
        Vector { elements: BTreeMap::new() }
    }

    pub fn insert(&mut self, index: u32, score: f64) {
        self.elements.insert(index, score);
    }
}

impl Serialize for Vector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut seq = serializer.serialize_seq(Some(self.elements.len() * 2))?;

        for (index, score) in self.elements.iter() {
            seq.serialize_element(index)?;
            seq.serialize_element(score)?;
        }

        seq.end()
    }
}
