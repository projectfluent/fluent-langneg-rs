use std::convert::TryFrom;
use unic_langid::LanguageIdentifier;

static REGION_MATCHING_KEYS: &[&str] = &[
    "az", "bg", "cs", "de", "es", "fi", "fr", "hu", "it", "lt", "lv", "nl", "pl", "ro", "ru",
];

pub fn add(langid: &LanguageIdentifier) -> Option<LanguageIdentifier> {
    let extended = match langid.to_string().as_str() {
        "en" => LanguageIdentifier::try_from("en-Latn-US"),
        "fr" => LanguageIdentifier::try_from("fr-Latn-FR"),
        "sr" => LanguageIdentifier::try_from("sr-Cyrl-SR"),
        "sr-RU" => LanguageIdentifier::try_from("sr-Latn-SR"),
        "az-IR" => LanguageIdentifier::try_from("az-Aram-IR"),
        "zh-GB" => LanguageIdentifier::try_from("zh-Hant-GB"),
        "zh-US" => LanguageIdentifier::try_from("zh-Hant-US"),
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

    Some(extended.expect("Failed to parse langid.").to_owned())
}
