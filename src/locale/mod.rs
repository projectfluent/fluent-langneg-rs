use std::collections::BTreeMap;
use std::fmt;

mod options;
mod parser;

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
/// loc.set_region("GB").unwrap();
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
        if !value.is_empty() && value != "und" {
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
        ""
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
        ""
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
        ""
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
            if let Some(position) = variants.iter().position(|x| *x == *value) {
                variants.remove(position);
            }
        }
    }

    pub fn get_variants(&self) -> Vec<&String> {
        self.variants
            .as_ref()
            .map_or(Vec::new(), |v| v.iter().collect())
    }

    pub fn clear_variants(&mut self) {
        self.variants = None;
    }

    pub fn has_privateuse(&self) -> bool {
        !self.privateuse.is_empty()
    }

    pub fn get_extensions(&self) -> BTreeMap<String, &BTreeMap<String, String>> {
        self.extensions.as_ref().map_or(BTreeMap::new(), |map| {
            map.iter()
                .map(|(key, value)| (key.clone(), value))
                .collect()
        })
    }

    pub fn add_extension(&mut self, ext_name: String, key: String, value: String) {
        if let Some(ref mut extensions) = self.extensions {
            let ext = extensions.entry(ext_name).or_insert_with(BTreeMap::new);
            ext.insert(key, value);
        } else {
            let mut exts = BTreeMap::new();
            let mut ext = BTreeMap::new();
            ext.insert(key, value);
            exts.insert(ext_name, ext);
            self.extensions = Some(exts);
        }
    }

    pub fn matches(&self, other: &Locale, available_range: bool, requested_range: bool) -> bool {
        if !self.privateuse.is_empty() || other.has_privateuse() {
            return false;
        }

        if (!available_range || !self.language.is_none())
            && (!requested_range || !other.get_language().is_empty())
            && self.get_language() != other.get_language()
        {
            return false;
        }

        if (!available_range || !self.script.is_none())
            && (!requested_range || !other.get_script().is_empty())
            && self.get_script() != other.get_script()
        {
            return false;
        }

        if (!available_range || !self.region.is_none())
            && (!requested_range || !other.get_region().is_empty())
            && self.get_region() != other.get_region()
        {
            return false;
        }

        if (!available_range || !self.variants.is_none())
            && (!requested_range || !other.get_variants().is_empty())
            && self.get_variants() != other.get_variants()
        {
            return false;
        }

        true
    }
}

impl From<String> for Locale {
    fn from(s: String) -> Self {
        Locale::new(s.as_str(), None).unwrap_or_else(|_| Locale::new("", None).unwrap())
    }
}

impl<'a> From<&'a str> for Locale {
    fn from(s: &'a str) -> Self {
        Locale::new(s, None).unwrap_or_else(|_| Locale::new("", None).unwrap())
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut subtags = vec![];

        subtags.push(self.get_language());

        if let Some(ref script) = self.script {
            subtags.push(script);
        }

        if let Some(ref region) = self.region {
            subtags.push(region);
        }

        if let Some(ref variants) = self.variants {
            for variant in variants {
                subtags.push(variant);
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
