use document::Document;
use field_ref::FieldRef;
use inverted_index::{InvertedIndex, Posting};
use vector::Vector;

use token::{Token, Tokens};
use std::collections::HashMap;

pub struct Builder {
    inverted_index: InvertedIndex,
    token_frequencies: HashMap<FieldRef, HashMap<Token, u32>>,
    field_lengths: HashMap<FieldRef, usize>,
    field_refs: Vec<FieldRef>,
    field_vectors: HashMap<FieldRef, Vector>
}

impl Builder {
    fn add<'a, T: Document<'a>>(&mut self, document: T) {
        for field in document.fields() {
            let field_ref = FieldRef::new(document.id(), &field.name);
            let tokens: Tokens = field.text.into();
            let field_length = tokens.len();

            *self.field_lengths.entry(field_ref.clone()).or_insert(0) += field_length;

            for token in tokens {
                self.inverted_index.add(token.clone(), field_ref.clone());

                *self.token_frequencies
                    .entry(field_ref.clone())
                    .or_insert(HashMap::new())
                    .entry(token)
                    .or_insert(0) += 1;
            }

            self.field_refs.push(field_ref);

        }
        return ();
    }

    fn build(&mut self) {
        for field_ref in &self.field_refs {
            let mut vector = Vector::new();
            let field_length = self.field_lengths.get(field_ref).expect("field ref missing");
            let token_frequencies = self.token_frequencies.get(field_ref).expect("token frequencies missing");

            for token in token_frequencies.keys() {
                let tf = token_frequencies.get(token).expect("token frequency missing");
                let posting = self.inverted_index.posting(token).expect("posting missing");
                let idf = self.idf(posting);
                let score = *tf as f64 * idf;

                vector.insert(posting.index as u32, score);
            }

            self.field_vectors.insert(field_ref.clone(), vector);
        }
    }

    fn idf(&self, posting: &Posting) -> f64 {
        let total_fields = self.field_lengths.len();
        let posting_fields = posting.len();

        let x = (total_fields - posting_fields / total_fields) as f64;

        (1.0f64 + x.abs()).ln()
    }
}