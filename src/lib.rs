#![cfg_attr(test, feature(test))]

extern crate test;

pub mod locale;
pub mod negotiate;

pub use locale::Locale;
pub use negotiate::negotiate_languages;

#[cfg(test)]
mod tests {
    use test::Bencher;
    use locale::Locale;
    use negotiate::negotiate_languages;

    #[bench]
    fn bench_locale(b: &mut Bencher) {
        let locales = ["en-US", "fr", "de", "en-GB", "it", "pl", "ru", "sr-Cyrl", "sr-Latn",
                       "zh-Hant", "zh-Hans", "ja-JP", "he-IL", "de-DE", "de-IT"];

        b.iter(|| for locale in locales.iter() {
                   Locale::new(locale, None).unwrap();
               });
    }

    #[bench]
    fn bench_negotiate(b: &mut Bencher) {
        let requested = vec!["de", "it", "ru"];
        let available = vec!["en-US", "fr", "de", "en-GB", "it", "pl", "ru", "sr-Cyrl", "sr-Latn",
                             "zh-Hant", "zh-Hans", "ja-JP", "he-IL", "de-DE", "de-IT"];

        b.iter(|| { negotiate_languages(requested.clone(), available.clone()); });
    }
}
