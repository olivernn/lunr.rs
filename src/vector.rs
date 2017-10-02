use std::collections::BTreeMap;

pub struct Vector {
    elements: BTreeMap<u32, f64>
}

impl Vector {
    pub fn new() -> Vector {
        Vector { elements: BTreeMap::new() }
    }

    pub fn insert(&mut self, index: u32, score: f64) {
        self.elements.insert(index, score);
    }
}