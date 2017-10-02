use token::Token;
use field_ref::FieldRef;

use std::collections::{HashMap, HashSet};

pub struct InvertedIndex {
    index: HashMap<Token, Posting>
}

impl InvertedIndex {
    fn new() -> InvertedIndex {
        InvertedIndex { index: HashMap::new() }
    }

    pub fn add(&mut self, token: Token, field_ref: FieldRef) {
        let index = self.index.len();

        let mut posting = self.index
            .entry(token)
            .or_insert(Posting::new(index));

        posting.insert(field_ref);
    }

    pub fn posting(&self, token: &Token) -> Option<&Posting> {
        self.index.get(token)
    }
}

pub struct Posting {
    pub index: usize,
    field_refs: HashSet<FieldRef>
}

impl Posting {
    fn new(index: usize) -> Posting {
        Posting { index: index, field_refs: HashSet::new() }
    }

    pub fn insert(&mut self, field_ref: FieldRef) {
        self.field_refs.insert(field_ref);
    }

    pub fn len(&self) -> usize {
        self.field_refs.len()
    }
}