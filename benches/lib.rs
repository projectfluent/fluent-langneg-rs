#![feature(test)]

extern crate test;
extern crate fluent_locale;

use test::Bencher;
use fluent_locale::Locale;
use fluent_locale::negotiate_languages;

#[bench]
fn bench_locale(b: &mut Bencher) {
    let locales = [
        "en-US",
        "fr",
        "de",
        "en-GB",
        "it",
        "pl",
        "ru",
        "sr-Cyrl",
        "sr-Latn",
        "zh-Hant",
        "zh-Hans",
        "ja-JP",
        "he-IL",
        "de-DE",
        "de-IT",
    ];

    b.iter(|| for locale in &locales {
        let _ = Locale::new(*locale, None);
    });
}

#[bench]
fn bench_negotiate(b: &mut Bencher) {
    let requested = vec!["de", "it", "ru"];
    let available = vec![
        "en-US",
        "fr",
        "de",
        "en-GB",
        "it",
        "pl",
        "ru",
        "sr-Cyrl",
        "sr-Latn",
        "zh-Hant",
        "zh-Hans",
        "ja-JP",
        "he-IL",
        "de-DE",
        "de-IT",
    ];

    b.iter(|| {
        negotiate_languages(
            &requested,
            &available,
            None,
            fluent_locale::NegotiationStrategy::Filtering,
        );
    });
}
