use token::Token;
use field_ref::FieldRef;

use serde::ser::{Serialize, Serializer, SerializeSeq, SerializeMap};

use std::collections::{BTreeMap, HashSet, HashMap};

#[derive(Default, Debug)]
pub struct InvertedIndex {
    index: BTreeMap<Token, Posting>,
}

impl InvertedIndex {
    pub fn add(&mut self, token: Token, field_ref: FieldRef) {
        let index = self.index.len();

        let posting = self.index.entry(token).or_insert_with(|| Posting::new(index));

        posting.insert(field_ref);
    }

    pub fn posting(&self, token: &Token) -> Option<&Posting> {
        self.index.get(token)
    }

    fn len(&self) -> usize {
        self.index.len()
    }
}

impl Serialize for InvertedIndex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;

        let pairs: Vec<(&Token, &Posting)> = self.index
            .iter()
            .map(|pair| pair)
            .collect();

        for pair in &pairs {
            seq.serialize_element(pair)?;
        }

        seq.end()
    }
}

#[derive(Debug)]
pub struct Posting {
    pub index: usize,
    field_postings: HashMap<String, FieldPosting>,
}

impl Posting {
    fn new(index: usize) -> Posting {
        Posting {
            index: index,
            field_postings: HashMap::new(),
        }
    }

    pub fn insert(&mut self, field_ref: FieldRef) {
        self.field_postings
            .entry(field_ref.field_name)
            .or_insert_with(FieldPosting::default)
            .insert(field_ref.document_ref)
    }

    pub fn len(&self) -> usize {
        self.field_postings.len()
    }
}

impl Serialize for Posting {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut posting = serializer.serialize_map(Some(self.len()))?;

        posting.serialize_entry("_index", &self.index)?;

        for (field_name, field_posting) in &self.field_postings {
            posting.serialize_entry(&field_name, &field_posting)?;
        }

        posting.end()
    }
}

#[derive(Default, Debug)]
struct FieldPosting {
    documents: HashSet<String>,
}

impl FieldPosting {
    fn insert(&mut self, document_ref: String) {
        self.documents.insert(document_ref);
    }
}

impl Serialize for FieldPosting {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut field_posting = serializer.serialize_map(Some(self.documents.len()))?;

        // empty for now...
        let metadata: HashMap<String, String> = HashMap::new();

        for document_ref in &self.documents {
            field_posting.serialize_entry(&document_ref, &metadata)?;
        }

        field_posting.end()
    }
}
