use std::collections::HashMap;
use std::fmt::{self, Display};
use std::error::Error as ErrorTrait;
use super::Locale;
use super::options;

fn is_alphabetic(s: &str) -> bool {
    s.chars()
        .all(|x| x >= 'A' && x <= 'Z' || x >= 'a' && x <= 'z')
}

fn is_numeric(s: &str) -> bool {
    s.chars().all(|x| x >= '0' && x <= '9')
}

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// The same extension subtag is only allowed once in a tag before the private use part.
    DuplicateExtension,
    /// If an extension subtag is present, it must not be empty.
    EmptyExtension,
    /// If the `x` subtag is present, it must not be empty.
    EmptyPrivateUse,
    /// The langtag contains a char that is not A-Z, a-z, 0-9 or the dash.
    ForbiddenChar,
    /// A subtag fails to parse, it does not match any other subtags.
    InvalidSubtag,
    /// The given language subtag is invalid.
    InvalidLanguage,
    /// A subtag may be eight characters in length at maximum.
    SubtagTooLong,
    /// At maximum three extlangs are allowed, but zero to one extlangs are preferred.
    TooManyExtlangs,
}

impl ErrorTrait for Error {
    fn description(&self) -> &str {
        match *self {
            Error::DuplicateExtension => "The same extension subtag is only allowed once in a tag",
            Error::EmptyExtension => "If an extension subtag is present, it must not be empty",
            Error::EmptyPrivateUse => "If the `x` subtag is present, it must not be empty",
            Error::ForbiddenChar => "The langtag contains a char not allowed",
            Error::InvalidSubtag => "A subtag fails to parse, it does not match any other subtags",
            Error::InvalidLanguage => "The given language subtag is invalid",
            Error::SubtagTooLong => "A subtag may be eight characters in length at maximum",
            Error::TooManyExtlangs => "At maximum three extlangs are allowed",
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

pub fn ext_name_for_key(key: &str) -> &'static str {
    match key {
        "u" => "unicode",
        _ => unimplemented!(),
    }
}

pub fn ext_key_for_name(key: &str) -> &'static str {
    match key {
        "unicode" => "u",
        _ => unimplemented!(),
    }
}

pub fn parse_language_subtag(t: &str) -> Result<String> {
    if t.len() < 2 || t.len() > 3 || !is_alphabetic(t) {
        return Err(Error::InvalidLanguage);
    }

    Ok(t.to_lowercase())
}

pub fn parse_script_subtag(t: &str) -> Result<String> {
    if t.len() != 4 || !is_alphabetic(t) {
        return Err(Error::InvalidLanguage);
    }

    let (first, rest) = t.split_at(1);

    let mut s = first.to_uppercase();
    s.push_str(rest);

    Ok(s)
}

pub fn parse_region_subtag(t: &str) -> Result<String> {
    if t.len() != 2 || !is_alphabetic(t) {
        return Err(Error::InvalidLanguage);
    }

    Ok(t.to_uppercase())
}

pub fn parse_language_tag(t: &str) -> Result<Locale> {
    let mut locale = Locale {
        language: None,
        extlangs: None,
        script: None,
        region: None,
        variants: None,
        extensions: None,
        privateuse: vec![],
    };

    if t.len() == 0 {
        return Ok(locale);
    }

    let mut position = 0;
    let mut current_extension: Option<&str> = None;
    let mut ext_key: Option<&str> = None;

    for subtag in t.split(|c| c == '-' || c == '_') {
        let slen = subtag.len();

        if subtag.len() > 8 {
            // All subtags have a maximum length of eight characters.
            return Err(Error::SubtagTooLong);
        }

        match current_extension {
            Some(ext) => {
                if ext == "x" {
                    locale.privateuse.push(subtag.to_owned());
                } else {
                    match ext_key {
                        Some(key) => {
                            if let Some(ref mut exts) = locale.extensions {
                                exts.get_mut(ext)
                                    .expect("no entry found for key")
                                    .insert(key.to_owned(), subtag.to_owned());
                                ext_key = None;
                            }
                        }
                        None => {
                            ext_key = Some(options::option_name_for_key(subtag));
                        }
                    }
                }
            }
            None => {
                if slen == 1 {
                    let ext_name = ext_name_for_key(subtag);
                    if let Some(ref mut exts) = locale.extensions {
                        if exts.contains_key(ext_name) {
                            return Err(Error::DuplicateExtension);
                        } else {
                            exts.insert(ext_name.to_owned(), HashMap::new());
                        }
                    } else {
                        let mut exts = HashMap::new();
                        exts.insert(ext_name.to_owned(), HashMap::new());
                        locale.extensions = Some(exts);
                    }
                    current_extension = Some(ext_name);
                } else if position == 0 {
                    // Primary language
                    if slen < 2 || slen > 3 || !is_alphabetic(subtag) {
                        return Err(Error::InvalidLanguage);
                    }
                    locale.set_language(subtag)?;
                    if slen < 4 {
                        // extlangs are only allowed for short language tags
                        position = 1;
                    } else {
                        position = 2;
                    }
                } else if position == 1 && slen == 3 && is_alphabetic(subtag) {
                    // extlangs
                    if let Some(ref mut extlangs) = locale.extlangs {
                        extlangs.push(subtag.to_owned());
                    } else {
                        locale.extlangs = Some(vec![subtag.to_owned()]);
                    }
                } else if position <= 2 && slen == 4 && is_alphabetic(subtag) {
                    // Script
                    locale.set_script(subtag)?;
                    position = 3;
                } else if position <= 3 &&
                          (slen == 2 && is_alphabetic(subtag) || slen == 3 && is_numeric(subtag)) {
                    locale.set_region(subtag)?;
                    position = 4;
                } else if position <= 4 &&
                          (slen >= 5 && is_alphabetic(&subtag[0..1]) ||
                           slen >= 4 && is_numeric(&subtag[0..1])) {
                    // Variant
                    if let Some(ref mut variants) = locale.variants {
                        variants.push(subtag.to_owned());
                    } else {
                        locale.variants = Some(vec![subtag.to_owned()]);
                    }
                    position = 4;
                } else {
                    return Err(Error::InvalidSubtag);
                }
            }
        }
    }
    Ok(locale)
}
