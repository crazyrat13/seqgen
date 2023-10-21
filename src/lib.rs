//! SeqGen is a sequence generation library. A library that generates sequences
//! based on given initial elements and a function that describes the transition
//! from the currently generated elements to next element to be generated in a lazy fashion
//! this means that the elements won't be generated unless explicitly requested or used.
//! seqgen does not only work with sequences of numbers, it works with any data type,
//! sequence elements can be strings, images, other sequences, etc...
//! The sequence implements the Iterator trait so that they can be manipulated
//! using the rust iterator API.

#![deny(missing_docs)]
#![doc(
    html_logo_url = "../seqgen_logo.svg",
    html_favicon_url = "../seqgen_logo.svg"
)]

pub mod error;
pub mod prelude;
pub mod sequence;
pub mod sequence_part;
