use std::fmt;
use std::collections::BTreeMap;

mod parser;
mod options;

/// A Locale object.
///
/// Locale object stores information encoded in a language tag and provides
/// methods allowing for parsing, serializing and manipulating locale fields.
///
/// All data is validated and canonicalized on input, which means that
/// the output is always canonicalized.
///
/// # Currently supported subtags are:
/// * `language` (e.g. "en")
/// * `script` (e.g. "Latn")
/// * `region` (e.g. "US")
/// * `variants` (e.g. "windows")
/// * `extensions` (e.g. "-u-ca-gregorian-hc-h12")
///
/// The API parses correctly the remaining fields of the BCP47 language tag, but
/// at the moment does not provide any API for operating on them.
///
/// # Examples
///
/// ## Parsing
/// Locale supports a `From` trait from `String` and `&str`:
///
/// ```
/// use fluent_locale::Locale;
///
/// let loc = Locale::from("en-latn-us");
///
/// assert_eq!(loc.to_string(), "en-Latn-US");
/// ```
///
/// Locale can also accept options, similarly to ECMA402 Intl.Locale:
///
/// ```
/// use fluent_locale::Locale;
/// use std::collections::BTreeMap;
///
/// let mut opts = BTreeMap::new();
/// opts.insert("hour-cycle", "h12");
/// let loc = Locale::new("en", Some(opts)).unwrap();
///
/// assert_eq!(loc.to_string(), "en-u-hc-h12");
/// ```
///
/// ## Serializing
/// Locale supports `Display` trait allowing for:
///
/// ```
/// use fluent_locale::Locale;
/// use std::collections::BTreeMap;
///
/// let mut opts = BTreeMap::new();
/// opts.insert("hour-cycle", "h12");
/// let loc = Locale::new("en-Latn-US-u-hc-h23", Some(opts)).unwrap();
///
/// assert_eq!(loc.to_string(), "en-Latn-US-u-hc-h12");
/// ```
///
/// ## Manipulating
/// During the lifetime of `Locale`, its fields can be modified via getter/setter
/// methods:
///
/// ```
/// use fluent_locale::Locale;
///
/// let mut loc = Locale::from("en-Latn-US");
/// loc.set_region("GB");
///
/// assert_eq!(loc.to_string(), "en-Latn-GB");
/// ```
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Locale {
    language: Option<String>,
    extlangs: Option<Vec<String>>,
    script: Option<String>,
    region: Option<String>,
    variants: Option<Vec<String>>,
    extensions: Option<BTreeMap<String, BTreeMap<String, String>>>,
    privateuse: Vec<String>,
}

impl Locale {
    pub fn new(loc_str: &str, opts: Option<BTreeMap<&str, &str>>) -> Result<Locale, parser::Error> {
        let mut locale = parser::parse_language_tag(loc_str)?;

        if let Some(opts) = opts {
            options::apply_options(&mut locale, opts);
        }

        Ok(locale)
    }

    pub fn set_language(&mut self, value: &str) -> parser::Result<()> {
        if !value.is_empty() {
            self.language = Some(parser::parse_language_subtag(value)?);
        } else {
            self.language = None;
        }
        Ok(())
    }

    pub fn get_language(&self) -> &str {
        if let Some(ref language) = self.language {
            return language.as_str();
        }
        return "";
    }

    pub fn set_script(&mut self, value: &str) -> parser::Result<()> {
        if !value.is_empty() {
            self.script = Some(parser::parse_script_subtag(value)?);
        } else {
            self.script = None;
        }
        Ok(())
    }

    pub fn get_script(&self) -> &str {
        if let Some(ref script) = self.script {
            return script.as_str();
        }
        return "";
    }

    pub fn set_region(&mut self, value: &str) -> parser::Result<()> {
        if !value.is_empty() {
            self.region = Some(parser::parse_region_subtag(value)?);
        } else {
            self.region = None;
        }
        Ok(())
    }

    pub fn get_region(&self) -> &str {
        if let Some(ref region) = self.region {
            return region.as_str();
        }
        return "";
    }

    pub fn add_variant(&mut self, value: String) {
        if let Some(ref mut variants) = self.variants {
            variants.push(value);
        } else {
            self.variants = Some(vec![value]);
        }
    }

    pub fn remove_variant(&mut self, value: String) {
        if let Some(ref mut variants) = self.variants {
            variants.remove_item(&value);
        }
    }

    pub fn get_variants(&self) -> Vec<&String> {
        self.variants
            .as_ref()
            .map_or(Vec::new(), |v| v.iter().map(|elem| elem).collect())
    }

    pub fn get_extensions(&self) -> BTreeMap<String, &BTreeMap<String, String>> {
        self.extensions
            .as_ref()
            .map_or(BTreeMap::new(), |map| {
                map.iter()
                    .map(|(key, value)| (key.clone(), value))
                    .collect()
            })
    }

    pub fn add_extension(&mut self, ext_name: String, key: String, value: String) {
        if let Some(ref mut extensions) = self.extensions {
            let ext = extensions.entry(ext_name).or_insert(BTreeMap::new());
            ext.insert(key, value);
        } else {
            let mut exts = BTreeMap::new();
            let mut ext = BTreeMap::new();
            ext.insert(key, value);
            exts.insert(ext_name, ext);
            self.extensions = Some(exts);
        }
    }

    pub fn matches(&self, other: &Locale, range: bool) -> bool {
        let language = self.get_language();
        if (range && self.language.is_none()) || (language != other.get_language()) {
            return false;
        }

        let script = self.get_script();
        if (range && !script.is_empty()) || (script != other.get_script()) {
            return false;
        }

        let region = self.get_region();
        if (range && !region.is_empty()) || (region != other.get_region()) {
            return false;
        }

        let variants = self.get_variants();
        if (range && variants.len() != 0) || (variants != other.get_variants()) {
            return false;
        }

        return true;
    }
}

impl From<String> for Locale {
    fn from(s: String) -> Self {
        Locale::new(s.as_str(), None).unwrap()
    }
}

impl<'a> From<&'a str> for Locale {
    fn from(s: &'a str) -> Self {
        Locale::new(s, None).unwrap()
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut subtags = vec![];

        subtags.push(self.get_language());

        if let Some(ref script) = self.script {
            subtags.push(&script);
        }

        if let Some(ref region) = self.region {
            subtags.push(&region);
        }

        if let Some(ref variants) = self.variants {
            for variant in variants {
                subtags.push(&variant);
            }
        }

        if let Some(ref extensions) = self.extensions {
            for (name, ext) in extensions {
                subtags.push(parser::ext_key_for_name(name));

                for (key, value) in ext {
                    subtags.push(options::option_key_for_name(key));
                    subtags.push(value);
                }
            }
        }

        write!(f, "{}", subtags.join("-"))
    }
}
