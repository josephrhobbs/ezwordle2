//! Main library for the EZWordle2 package.

// No compile warnings
#![deny(warnings)]

// Everything must be documented
#![deny(missing_docs)]

mod matchstring;
mod word;
mod wordlist;

pub use matchstring::{
    Match,
    MatchString,
};
pub use word::Word;
pub use wordlist::Wordlist;