#![feature(vec_remove_item)]
#![feature(ascii_ctype)]

//! fluent-locale is an API for operating on locales and language tags.
//! It's part of Project Fluent, a localization framework designed to unleash
//! the expressive power of the natural language.
//!
//! The primary use of fluent-locale is to parse/modify/serialize language tags
//! and to perform language negotiation.
//!
//! fluent-locale operates on a subset of [BCP47](http://tools.ietf.org/html/bcp47).
//! It can parse full BCP47 language tags, and will serialize them back,
//! but currently only allows for operations on primary subtags and
//! unicode extension keys.
//!
//! In result fluent-locale is not suited to replace full implementations of
//! BCP47 like [rust-language-tags](https://github.com/pyfisch/rust-language-tags),
//! but is arguably a better option for use cases involving operations on
//! language tags and for language negotiation.

pub mod locale;
pub mod negotiate;
pub mod accepted_languages;

pub use locale::Locale;
pub use negotiate::negotiate_languages;
pub use negotiate::NegotiationStrategy;
pub use accepted_languages::parse as parse_accepted_languages;
