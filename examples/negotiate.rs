use fluent_locale::negotiate::NegotiationStrategy;
use fluent_locale::{convert_vec_str_to_langids_lossy, negotiate_languages};

fn main() {
    let requested = convert_vec_str_to_langids_lossy(&["it", "pl", "ru"]);
    let available = convert_vec_str_to_langids_lossy(&["fr", "en-GB", "en-US", "ru", "pl"]);
    let supported =
        negotiate_languages(&requested, &available, None, NegotiationStrategy::Filtering);

    println!("{:?}", supported);
}
