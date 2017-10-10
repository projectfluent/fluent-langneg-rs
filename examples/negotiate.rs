extern crate fluent_locale;

use fluent_locale::negotiate_languages;
use fluent_locale::negotiate::NegotiationStrategy;

fn main() {
    let supported = negotiate_languages(
        &["it", "pl", "ru"],
        &["fr", "en-GB", "en-US", "ru", "pl"],
        None,
        NegotiationStrategy::Filtering,
    );

    println!("{:?}", supported);
}
