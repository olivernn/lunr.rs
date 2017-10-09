use document::Document;
use field_ref::FieldRef;
use inverted_index::{InvertedIndex, Posting};
use vector::Vector;

use token::{Term, Tokens};
use std::collections::{HashMap, HashSet};

pub fn create() -> Builder {
    Builder::default()
}

#[derive(Default)]
pub struct Builder {
    pub inverted_index: InvertedIndex,
    pub field_vectors: HashMap<FieldRef, Vector>,
    pub fields: HashSet<String>,

    term_frequencies: HashMap<FieldRef, HashMap<Term, u32>>,
    field_lengths: HashMap<FieldRef, usize>,
    field_refs: Vec<FieldRef>,
}

impl Builder {
    pub fn add<'a, T: Document<'a>>(&mut self, document: T) {
        for field in document.fields() {
            let field_ref = FieldRef::new(document.id(), field.name.to_owned());
            let tokens: Tokens = field.text.into();
            let field_length = tokens.len();

            self.fields.insert(field.name);

            *self.field_lengths.entry(field_ref.clone()).or_insert(0) += field_length;

            for token in tokens {
                *self.term_frequencies
                     .entry(field_ref.clone())
                     .or_insert_with(HashMap::new)
                     .entry(token.term.to_owned())
                     .or_insert(0) += 1;

                self.inverted_index.add(field_ref.clone(), token);
            }

            self.field_refs.push(field_ref);
        }
        ()
    }

    pub fn build(&mut self) {
        for field_ref in &self.field_refs {
            let mut vector: Vector = Default::default();
            let term_frequencies =
                self.term_frequencies.get(field_ref).expect("token frequencies missing");

            for term in term_frequencies.keys() {
                let tf = f64::from(*term_frequencies.get(term).expect("token frequency missing"));
                let posting = self.inverted_index.posting(term).expect("posting missing");
                let idf = self.idf(posting);
                let score = tf * idf;

                vector.insert(posting.index as u32, score);
            }

            self.field_vectors.insert(field_ref.clone(), vector);
        }
    }

    fn idf(&self, posting: &Posting) -> f64 {
        let total_fields = self.field_lengths.len();
        let posting_fields = posting.len();

        let x = (total_fields / (1 + posting_fields)) as f64;

        (1.0f64 + x.abs()).ln()
    }
}
