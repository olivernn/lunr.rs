#![feature(offset_to)]
extern crate erased_serde;
extern crate serde;
extern crate serde_json;
extern crate unicode_segmentation;

mod field_ref;
mod token;
pub mod builder;
pub mod document;
mod inverted_index;
mod vector;
pub mod index;
mod tokenizer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
