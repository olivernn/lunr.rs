use serde::ser::{Serialize, Serializer};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct FieldRef {
    pub document_ref: String,
    pub field_name: String,
}

impl FieldRef {
    pub fn new<S: Into<String>>(document_ref: S, field_name: S) -> FieldRef {
        FieldRef {
            document_ref: document_ref.into(),
            field_name: field_name.into(),
        }
    }
}


impl Serialize for FieldRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&format!("{}/{}", self.field_name, self.document_ref))
    }
}
