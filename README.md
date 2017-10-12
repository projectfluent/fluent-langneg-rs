# Fluent Locale

**Fluent Locale is a library for language tags manipulations and negotiation.**

[![crates.io](http://meritbadge.herokuapp.com/fluent-locale)](https://crates.io/crates/fluent-locale)
[![Build Status](https://travis-ci.org/projectfluent/fluent-locale-rs.svg?branch=master)](https://travis-ci.org/projectfluent/fluent-locale-rs)
[![Coverage Status](https://coveralls.io/github/projectfluent/fluent-locale-rs/badge.svg)](https://coveralls.io/github/projectfluent/fluent-locale-rs)

Introduction
------------

This is a Rust implementation of fluent-locale library which is a part of Project Fluent.

The library allows for parsing language tags into `Locale` objects, operating on them
and serializing the result back to language tag strings.

On top of that, it allows for simple operations like comparing `Locale` objects and
negotiating between lists of language tags.


Installation
------------

```toml
[dependencies]
fluent_locale = "0.2.0"
```

Usage
-----

```rust
extern crate fluent_locale;

use fluent_locale::Locale;
use fluent_locale::negotiate_languages;

let loc = Locale::from("fr-CA");
println!("Language: {}", loc.get_language());
println!("Script: {}", loc.get_script());
println!("Region: {}", loc.get_region());
println!("Variants: {}", loc.get_variants());
println!("Extensions: {}", loc.get_extensions());

let loc2 = Locale::new("fr-FR");

loc2.set_region("ca")?;

// The second and third parameters allow for range matching 
if loc.matches(loc2, false, false) {
  println!("Locales are matching!");
}

let supported = negotiate_languages(
  &["de-DE", "fr-FR", "en-US"],
  &["de-DE", "de-AT", "fr-CA", "fr", "en-GB", "en", "en-US", "it"],
  "en-US",
  fluent_locale::NegotiationStrategy::Filtering
);
```

See [docs.rs][] for more examples.

[docs.rs]: https://docs.rs/fluent-locale/

Status
------

The implementation is in early stage, but is complete according to fluent-locale
corpus of tests, which means that it parses, serializes and negotiates as expected.

The remaining work is on the path to 1.0 is to gain in-field experience of using it,
add more tests and ensure that bad input is correctly handled.

Compatibility
-------------

The API is based on [BCP47][] definition of Language Tags and is aiming to
parse and serialize all language tags according to that definition.

Parsed language tags are stored as `Locale` objects compatible with
ECMA402's [Intl.Locale][] and allow for operations on language tag subtags and
unicode extension keys as defined by [RFC6067][] and Unicode [UTS35][]

Language negotiation algorithms are custom Project Fluent solutions,
based on [RFC4647][].

The current API only allows for operations on basic language subtags (language, script, region, variants)
and unicode extension keys. Other subtags will be parsed and serialized, but there is no
API access to them when operating on the `Locale` object.

The language negotiation strategies aim to replicate the best-effort matches with
the most limited amount of data. The algorithm returns reasonable
results without any database, but the results can be improved with either limited
or full [CLDR likely-subtags] database.

The result is a balance chosen for Project Fluent and may differ from other
implementations of language negotiation algorithms which may choose different
tradeoffs.

[BCP47]: https://tools.ietf.org/html/bcp47
[Intl.Locale]: https://github.com/tc39/proposal-intl-locale
[RFC6067]: https://www.ietf.org/rfc/rfc6067.txt
[UTS35]: http://www.unicode.org/reports/tr35/#Locale_Extension_Key_and_Type_Data
[RFC4647]: https://tools.ietf.org/html/rfc4647
[CLDR likely-subtags]: http://www.unicode.org/cldr/charts/latest/supplemental/likely_subtags.html

Alternatives
------------

Although Fluent Locale aims to stay close to W3C Accepted Languages, it does not aim
to implement the full behavior and some aspects of the language negotiation strategy
recommended by W3C, such as weights, are not a target right now.

For such purposes, [rust-language-tags][] crate seems to be a better choice.

[rust-language-tags]: https://github.com/pyfisch/rust-language-tags

Performance
-----------

There has not been a significant performance work being done on the library yet,
so we expect there are some low hanging fruit waiting for someone to find them.

At the moment performance is comparable to previously mentioned `language-tags` crate
for parsing a sample list of language tags based on this crate's benchmark code:


    running 2 tests
    test bench_locale(fluent-locale)  ... bench:       1,773 ns/iter (+/- 48)
    test bench_locale(language-tags) ... bench:       1,982 ns/iter (+/- 280)


Develop
-------

    cargo build
    cargo test
    cargo bench

