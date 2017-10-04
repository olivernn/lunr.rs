# Lunr.rs

A Lunr backend implemented in Rust.

## Status

**Very** early stages.

The current implementation is able to generate an index that is readable and searchable by lunr.js, but that is about it. Currently there is no [pipeline](https://lunrjs.com/docs/lunr.Pipeline.html) and no ability to associate metadata with a token.

- [ ] Implement text processing pipeline
- [ ] Add stemmer?
- [ ] Implement token metadata (collecting and serialising)
- [ ] Capture token positions during tokenisation
- [ ] Improve the interface for defining documents and indexes

There is almost certainly a bunch more things that need to be done, this is very definitly still at the proof of concept stage.

## Example

There is a simple example that will spit out a serialised index on stdout. This can be used to create a lunr index in JavaScript, the following _should_ work in any JavaScript environment that has Lunr:

```javascript
let idx = lunr.Index.load(JSON.parse('BIG GLOB OF JSON HERE'))
idx.search('life')
```