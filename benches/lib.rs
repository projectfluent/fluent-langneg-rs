#![feature(test)]

extern crate test;

use fluent_locale::convert_vec_str_to_langids_lossy;
use fluent_locale::negotiate_languages;
use test::Bencher;
use unic_langid::LanguageIdentifier;

#[bench]
fn bench_locale(b: &mut Bencher) {
    let langids = [
        "en-US", "fr", "de", "en-GB", "it", "pl", "ru", "sr-Cyrl", "sr-Latn", "zh-Hant", "zh-Hans",
        "ja-JP", "he-IL", "de-DE", "de-IT",
    ];

    b.iter(|| {
        for locale in &langids {
            let _: LanguageIdentifier = locale.parse().unwrap();
        }
    });
}

#[bench]
fn bench_negotiate(b: &mut Bencher) {
    let requested = vec!["de", "it", "ru"];
    let available = vec![
        "en-US", "fr", "de", "en-GB", "it", "pl", "ru", "sr-Cyrl", "sr-Latn", "zh-Hant", "zh-Hans",
        "ja-JP", "he-IL", "de-DE", "de-IT",
    ];

    let requested = convert_vec_str_to_langids_lossy(&requested);
    let available = convert_vec_str_to_langids_lossy(&available);

    b.iter(|| {
        negotiate_languages(
            &requested,
            &available,
            None,
            fluent_locale::NegotiationStrategy::Filtering,
        );
    });
}
