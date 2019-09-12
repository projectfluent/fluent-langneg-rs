use unic_langid::LangId;
use unic_langid::LanguageIdentifier;

static REGION_MATCHING_KEYS: &[&str] = &[
    "az", "bg", "cs", "de", "es", "fi", "fr", "hu", "it", "lt", "lv", "nl", "pl", "ro", "ru",
];

pub fn add<L: LangId>(langid: &L) -> Option<LanguageIdentifier> {
    if langid.get_script().is_some() {
        return None;
    }

    let extended = match (langid.get_language(), langid.get_region()) {
        ("en", None) => "en-Latn-US",
        ("fr", None) => "fr-Latn-FR",
        ("sr", None) => "sr-Cyrl-RS",
        ("sr", Some("RU")) => "sr-Latn-RU",
        ("az", Some("IR")) => "az-Arab-IR",
        ("zh", Some("GB")) => "zh-Hant-GB",
        ("zh", Some("US")) => "zh-Hant-US",
        _ => {
            let lang = langid.get_language();

            for subtag in REGION_MATCHING_KEYS {
                if lang == *subtag {
                    let new_lang = LanguageIdentifier::from_parts(
                        Some(langid.get_language()),
                        langid.get_script(),
                        Some(subtag),
                        &langid.get_variants(),
                    )
                    .expect("FAILED");
                    return Some(new_lang);
                }
            }
            return None;
        }
    };
    let langid: LanguageIdentifier = extended.parse().expect("Failed to parse langid.");

    Some(langid)
}
