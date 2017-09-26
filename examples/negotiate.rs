extern crate fluent_locale;

use fluent_locale::negotiate_languages;
use fluent_locale::negotiate::NegotiationStrategy;

fn main() {
    let supported = negotiate_languages(
        vec!["it", "pl", "ru"],
        vec!["fr", "en-GB", "en-US", "ru", "pl"],
        None,
        NegotiationStrategy::Filtering,
    );

    println!("{:?}", supported);
}
