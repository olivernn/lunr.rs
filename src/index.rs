use inverted_index::InvertedIndex;
use field_ref::FieldRef;
use vector::Vector;
use builder::Builder;

use serde::ser::{Serialize, Serializer, SerializeStruct};

use std::collections::HashSet;
use std::convert::From;

type FieldVector = (FieldRef, Vector);

pub struct Index {
    version: String,
    inverted_index: InvertedIndex,
    field_vectors: Vec<FieldVector>,
    fields: HashSet<String>,
    pipeline: Vec<String>,
}

impl From<Builder> for Index {
    fn from(mut builder: Builder) -> Index {
        builder.build();

        Index {
            version: String::from("2.1.3"),
            pipeline: vec![],
            inverted_index: builder.inverted_index,
            field_vectors: builder.field_vectors
                .into_iter()
                .map(|(k, v)| (k, v))
                .collect(),
            fields: builder.fields,
        }
    }
}

impl Serialize for Index {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut index = serializer.serialize_struct("Index", 3)?;

        index.serialize_field("version", &self.version)?;
        index.serialize_field("pipeline", &self.pipeline)?;
        index.serialize_field("fields", &self.fields)?;
        index.serialize_field("fieldVectors", &self.field_vectors)?;
        index.serialize_field("invertedIndex", &self.inverted_index)?;

        index.end()
    }
}
