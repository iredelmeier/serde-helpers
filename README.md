# serde-helpers

### Wrappers for serializing/deserializing Rust objects to and from different data formats

[![Build Status](https://travis-ci.org/iredelmeier/serde-helpers.svg?branch=master)](https://travis-ci.org/iredelmeier/serde-helpers)
[![Docs](https://docs.rs/serde-helpers/badge.svg)](https://docs.rs/serde-helpers)
[![Crates.io](https://img.shields.io/crates/v/serde-helpers.svg)](https://crates.io/crates/serde-helpers)

`serde-helpers` provides wrappers for serializing and deserializing data structures
to and from certain data formats.

The library is primarily intended to keep crates that need to serialize or deserialize
data from having to be aware of `serde` itself, if they do not otherwise have a need
to consume it.

Additionally, `serde-helpers` provides some additional consistency across data formats,
e.g., by providing a single, consolidated `Error` struct that wraps the format-specific
error.
