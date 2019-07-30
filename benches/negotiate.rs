use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use fluent_locale::convert_vec_str_to_langids_lossy;
use fluent_locale::negotiate_languages;

fn negotiate_bench(c: &mut Criterion) {
    let requested = vec!["de", "it", "ru"];
    let available = vec![
        "en-US", "fr", "de", "en-GB", "it", "pl", "ru", "sr-Cyrl", "sr-Latn", "zh-Hant", "zh-Hans",
        "ja-JP", "he-IL", "de-DE", "de-IT",
    ];

    let requested = convert_vec_str_to_langids_lossy(&requested);
    let available = convert_vec_str_to_langids_lossy(&available);

    c.bench_function("negotiate", move |b| {
        b.iter(|| {
            negotiate_languages(
                &requested,
                &available,
                None,
                fluent_locale::NegotiationStrategy::Filtering,
            );
        })
    });
}

criterion_group!(benches, negotiate_bench);
criterion_main!(benches);
