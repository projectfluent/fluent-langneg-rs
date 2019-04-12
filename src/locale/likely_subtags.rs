#[path = "tables.rs"]
mod tables;

use super::Locale;

/// Get little-endian numeric representation of string
fn str_to_le(s: &str) -> u32 {
    if s.len() > 4 {
        !0
    } else {
        let mut buf = [0; 4];
        buf[0..s.len()].copy_from_slice(s.as_bytes());
        u32::from_le_bytes(buf)
    }
}

fn lookup_binary<K: Ord, T: Copy>(key: K, data: &[(K, T)]) -> Option<T> {
    data.binary_search_by(|(k, _)| k.cmp(&key)).ok().map(|i| data[i].1)
}

/// Apply a lookup result to the locale.
///
/// Assumes lookup is of form "lang-Scrp-REG".
fn apply_lookup(locale: &mut Locale, lookup: &str) {
    let bytes = lookup.as_bytes();
    let end_lang = bytes.iter().position(|&b| b == b'-').unwrap();
    let start_script = end_lang + 1;
    let end_script = start_script + bytes[start_script..].iter().position(|&b| b == b'-').unwrap();
    let start_region = end_script + 1;
    if locale.get_language().is_empty() {
        let _ = locale.set_language(&lookup[..end_lang]);
    }
    if locale.get_script().is_empty() {
        let _ = locale.set_script(&lookup[start_script..end_script]);
    }
    if locale.get_region().is_empty() {
        let _ = locale.set_region(&lookup[start_region..]);
    }
}

const SCRIPT_ZZZZ_TAG: u32 = 0x7a7a7a5a; // "Zzzz" in little-endian
const REGION_ZZ_TAG: u32 = 0x5a5a; // "ZZ" in little-endian

/// Add likely subtags to locale.
///
/// Returns `true` when the lookup succeeded.
///
/// See <http://www.unicode.org/reports/tr35/#Likely_Subtags>
impl Locale {
    pub fn add_likely_subtags(&mut self) -> bool {
        // Canonicalize.
        // TODO: replace deprecated subtags.
        // TODO: bail on grandfathered tag.
        let lang = str_to_le(self.get_language());
        let script = str_to_le(self.get_script());
        let region = str_to_le(self.get_region());
        if script == SCRIPT_ZZZZ_TAG {
            let _ = self.set_script("");
        }
        if region == REGION_ZZ_TAG {
            let _ = self.set_region("");
        }
        // Lookup.
        if lang == 0 {
            let key = (region as u64) << 32 | (script as u64);
            if let Some(lookup) = lookup_binary(key, &tables::SCRIPT_REGION) {
                apply_lookup(self, lookup);
                return true;
            }
        }
        let key = (region as u64) << 32 | (lang as u64);
        if let Some(lookup) = lookup_binary(key, &tables::LANG_REGION) {
            apply_lookup(self, lookup);
            return true;
        }
        let key = (script as u64) << 32 | (lang as u64);
        if let Some(lookup) = lookup_binary(key, &tables::LANG_SCRIPT) {
            apply_lookup(self, lookup);
            return true;
        }
        if let Some(lookup) = lookup_binary(lang, &tables::LANG_ONLY) {
            apply_lookup(self, lookup);
            return true;
        }
        let key = (script as u64) << 32; // und-script
        if let Some(lookup) = lookup_binary(key, &tables::LANG_SCRIPT) {
            apply_lookup(self, lookup);
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::Locale;

    fn a(fr: &str, to: &str) {
        let mut loc = Locale::from(fr);
        loc.add_likely_subtags();
        assert_eq!(loc.to_string(), to);
    }

    #[test]
    fn add_likely() {
        a("en-Zzzz-US", "en-Latn-US");
        a("en-ZZ", "en-Latn-US");
        a("und-Arab-CC", "ms-Arab-CC");
        a("und-Hebr-GB", "yi-Hebr-GB");
        a("yi-GB", "yi-Hebr-GB");
        a("az", "az-Latn-AZ");
        a("az-IS", "az-Latn-IS");
        a("az-IQ", "az-Arab-IQ");
        a("az-RU", "az-Cyrl-RU");
        a("az-Arab", "az-Arab-IR");
        a("zh-CN", "zh-Hans-CN");
        a("zh-HK", "zh-Hant-HK");
        a("und-Adlm", "ff-Adlm-GN");
        a("und-Adlm-IS", "is-Adlm-IS");
        a("und-Adlm-IO", "ff-Adlm-IO");
        a("und-CN", "zh-Hans-CN");
        a("en-Shaw", "en-Shaw-GB");
        a("ZH-ZZZZ-SG", "zh-Hans-SG"); // example from spec
    }

    #[test]
    fn add_likely_consistent_with_shortcuts() {
        // Make sure this logic is consistent with the shortcuts in the negotiate module.
        a("az", "az-Latn-AZ");
        a("bg", "bg-Cyrl-BG");
        a("cs", "cs-Latn-CZ");
        a("de", "de-Latn-DE");
        a("en", "en-Latn-US");
        a("es", "es-Latn-ES");
        a("fi", "fi-Latn-FI");
        a("fr", "fr-Latn-FR");
        a("hu", "hu-Latn-HU");
        a("it", "it-Latn-IT");
        a("lt", "lt-Latn-LT");
        a("lv", "lv-Latn-LV");
        a("nl", "nl-Latn-NL");
        a("pl", "pl-Latn-PL");
        a("ro", "ro-Latn-RO");
        a("ru", "ru-Cyrl-RU");
        a("sr", "sr-Cyrl-RS");
        a("sr-RU", "sr-Latn-RU");
        a("az-IR", "az-Arab-IR");
        a("zh-GB", "zh-Hant-GB");
        a("zh-US", "zh-Hant-US");
    }
}
