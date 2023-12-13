use icu_locid::{
    langid,
    subtags::{language, region, Language, Region},
    LanguageIdentifier,
};

static REGION_MATCHING_KEYS: &[(Language, Region)] = &[
    (language!("az"), region!("AZ")),
    (language!("bg"), region!("BG")),
    (language!("cs"), region!("CS")),
    (language!("de"), region!("DE")),
    (language!("es"), region!("ES")),
    (language!("fi"), region!("FI")),
    (language!("fr"), region!("FR")),
    (language!("it"), region!("IT")),
    (language!("lt"), region!("LT")),
    (language!("lv"), region!("LV")),
    (language!("nl"), region!("NL")),
    (language!("nu"), region!("NU")),
    (language!("pl"), region!("PL")),
    (language!("ro"), region!("RO")),
    (language!("ru"), region!("RU")),
];

#[derive(PartialEq, Eq, Debug)]
pub enum TransformResult {
    Modified,
    Unmodified,
}

pub struct LocaleExpander;

impl LocaleExpander {
    pub fn new() -> Self {
        Self
    }

    pub fn maximize(&self, input: &mut LanguageIdentifier) -> TransformResult {
        let extended = match &input {
            b if *b == &langid!("en") => langid!("en-Latn-US"),
            b if *b == &langid!("fr") => langid!("fr-Latn-FR"),
            b if *b == &langid!("sr") => langid!("sr-Cyrl-SR"),
            b if *b == &langid!("sr-RU") => langid!("sr-Latn-SR"),
            b if *b == &langid!("az-IR") => langid!("az-Arab-IR"),
            b if *b == &langid!("zh-GB") => langid!("zh-Hant-GB"),
            b if *b == &langid!("zh-US") => langid!("zh-Hant-US"),
            _ => {
                let lang = &input.language;

                if let Ok(idx) = REGION_MATCHING_KEYS.binary_search_by(|(l, _)| l.cmp(lang)) {
                    let subtag = REGION_MATCHING_KEYS[idx].1;
                    input.region = Some(subtag);
                    return TransformResult::Modified;
                }
                return TransformResult::Unmodified;
            }
        };
        let (language, script, region) = (extended.language, extended.script, extended.region);
        input.language = language;
        input.script = script;
        input.region = region;
        TransformResult::Modified
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region_matching_sort() {
        for v in REGION_MATCHING_KEYS.windows(2) {
            let (v1, v2) = (v[0], v[1]);
            assert!(
                v1.0 < v2.0,
                "Language \"{}\" is placed after \"{}\"",
                v1.0,
                v2.0
            );
        }
    }
}
