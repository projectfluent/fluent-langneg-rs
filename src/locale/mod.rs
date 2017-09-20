use std::fmt;
use std::collections::HashMap;

mod parser;
mod options;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Locale {
    pub language: Option<String>,
    extlangs: Option<Vec<String>>,
    pub script: Option<String>,
    pub region: Option<String>,
    pub variants: Option<Vec<String>>,
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
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut subtags = vec![];

        if let Some(ref language) = self.language {
            subtags.push(language.clone());
        } else {
            subtags.push("und".to_owned());
        }

        if let Some(ref extensions) = self.extensions {
            for (name, ext) in extensions {
                let mut ext_keys = vec![parser::ext_key_for_name(name).to_owned()];

                for (key, value) in ext {
                    ext_keys.push(format!("{}-{}", options::option_key_for_name(key), value));
                }

                subtags.push(ext_keys.join("-"));
            }
        }

        write!(f, "{}", subtags.join("-"))
    }
}
