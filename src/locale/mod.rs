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
    extensions: Option<HashMap<String, HashMap<String, String>>>,
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

    pub fn get_extensions(&self) -> HashMap<String, &HashMap<String, String>> {
        self.extensions
            .as_ref()
            .map_or(HashMap::new(), |map| {
                map.iter()
                    .map(|(key, value)| (key.clone(), value))
                    .collect()
            })
    }

    pub fn add_extension(&mut self, ext_name: String, key: String, value: String) {
        if let Some(ref mut extensions) = self.extensions {
            let ext = extensions.entry(ext_name).or_insert(HashMap::new());
            ext.insert(key, value);
        } else {
            let mut exts = HashMap::new();
            let mut ext = HashMap::new();
            ext.insert(key, value);
            exts.insert(ext_name, ext);
            self.extensions = Some(exts);
        }
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
