extern crate serde;
extern crate serde_json;

mod field_ref;
mod token;
pub mod builder;
pub mod document;
mod inverted_index;
mod vector;
pub mod index;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
