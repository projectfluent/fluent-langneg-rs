use std::fmt;
use std::collections::HashMap;

mod parser;
mod options;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Locale {
    language: Option<String>,
    extlangs: Option<Vec<String>>,
    script: Option<String>,
    region: Option<String>,
    variants: Option<Vec<String>>,
    pub extensions: Option<HashMap<String, HashMap<String, String>>>,
    privateuse: Vec<String>,
}

impl Locale {
    pub fn new(loc_str: &str, opts: Option<HashMap<&str, &str>>) -> Result<Locale, parser::Error> {
        let mut locale = parser::parse_language_tag(loc_str)?;

        if let Some(opts) = opts {
            options::apply_options(&mut locale, opts);
        }

        Ok(locale)
    }

    pub fn set_language(&mut self, value: Option<String>) {
        self.language = value;
    }

    pub fn get_language(&self) -> Option<&String> {
        self.language.as_ref()
    }

    pub fn set_script(&mut self, value: Option<String>) {
        self.script = value;
    }

    pub fn get_script(&self) -> Option<&String> {
        self.script.as_ref()
    }

    pub fn set_region(&mut self, value: Option<String>) {
        self.region = value;
    }

    pub fn get_region(&self) -> Option<&String> {
        self.region.as_ref()
    }

    pub fn add_variant(&mut self, value: String) {}

    pub fn remove_variant(&self) {}

    pub fn get_variants(&self) -> Option<&Vec<String>> {
        self.variants.as_ref()
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

        subtags.push(self.get_language().as_ref().map_or("und", |l| &l));

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
