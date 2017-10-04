extern crate lunr;
extern crate serde_json;

use lunr::builder;
use lunr::index::Index;
use lunr::document::{Document, Field};

struct Quote {
    id: String,
    text: String
}

impl<'a> Document<'a> for Quote {
    fn id(&self) -> String {
        self.id.to_owned()
    }

    fn fields(&self) -> Vec<Field> {
        vec![Field { name: String::from("text"), text: self.text.to_owned() }]
    }
}

fn main() {
    let lennon = Quote {
        id: String::from("lennon"),
        text: String::from("life is what happens while you are busy making other plans"),
    };

    let wilde = Quote {
        id: String::from("wilde"),
        text: String::from("work is the curse of the drinking classes"),
    };

    let mut builder = builder::create();

    builder.add(lennon);
    builder.add(wilde);

    let index: Index = builder.into();

    let json = serde_json::to_string(&index).expect("json serialization failed");

    println!("{}", json);
}