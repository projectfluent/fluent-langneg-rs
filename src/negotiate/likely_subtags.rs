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
    (language!("nu"), region!("NU")),
    (language!("it"), region!("IT")),
    (language!("lt"), region!("LT")),
    (language!("lv"), region!("LV")),
    (language!("nl"), region!("NL")),
    (language!("pl"), region!("PL")),
    (language!("ro"), region!("RO")),
    (language!("ru"), region!("RU")),
];

pub trait MockLikelySubtags {
    fn maximize(&mut self) -> bool;
}

impl MockLikelySubtags for LanguageIdentifier {
    fn maximize(&mut self) -> bool {
        let extended = match &self {
            b if *b == &langid!("en") => langid!("en-Latn-US"),
            b if *b == &langid!("fr") => langid!("fr-Latn-FR"),
            b if *b == &langid!("sr") => langid!("sr-Cyrl-SR"),
            b if *b == &langid!("sr-RU") => langid!("sr-Latn-SR"),
            b if *b == &langid!("az-IR") => langid!("az-Arab-IR"),
            b if *b == &langid!("zh-GB") => langid!("zh-Hant-GB"),
            b if *b == &langid!("zh-US") => langid!("zh-Hant-US"),
            _ => {
                let lang = &self.language;

                if let Ok(idx) = REGION_MATCHING_KEYS.binary_search_by(|(l, _)| l.cmp(lang)) {
                    let subtag = REGION_MATCHING_KEYS[idx].1;
                    self.region = Some(subtag);
                    return true;
                }
                return false;
            }
        };
        let (language, script, region) = (extended.language, extended.script, extended.region);
        self.language = language;
        self.script = script;
        self.region = region;
        true
    }
}
