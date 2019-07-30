use unic_langid::LanguageIdentifier;

static REGION_MATCHING_KEYS: &[&str] = &[
    "az", "bg", "cs", "de", "es", "fi", "fr", "hu", "it", "lt", "lv", "nl", "pl", "ro", "ru",
];

pub fn add(langid: &LanguageIdentifier) -> Option<LanguageIdentifier> {
    let extended = match langid.to_string().as_str() {
        "en" => "en-Latn-US",
        "fr" => "fr-Latn-FR",
        "sr" => "sr-Cyrl-SR",
        "sr-RU" => "sr-Latn-SR",
        "az-IR" => "az-Arab-IR",
        "zh-GB" => "zh-Hant-GB",
        "zh-US" => "zh-Hant-US",
        _ => {
            let lang = langid.get_language();

            for subtag in REGION_MATCHING_KEYS {
                if lang == *subtag {
                    let mut new_lang = langid.clone();
                    new_lang.set_region(Some(subtag)).unwrap();
                    return Some(new_lang);
                }
            }
            return None;
        }
    };
    let langid: LanguageIdentifier = extended.parse().expect("Failed to parse langid.");

    Some(langid)
}
