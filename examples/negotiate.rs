extern crate fluent_locale;

use fluent_locale::negotiate_languages;

fn main() {
    let supported = negotiate_languages(
        vec!["it", "pl", "ru"],
        vec!["fr", "en-GB", "en-US", "ru", "pl"],
        None,
    );

    println!("{:?}", supported);
}
