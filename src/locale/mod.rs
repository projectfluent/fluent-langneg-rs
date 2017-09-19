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
