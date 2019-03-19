use super::{Locale, TinyStr4};
use std::collections::BTreeMap;

pub fn option_name_for_key(key: &str) -> &'static str {
    match key {
        "hc" => "hour-cycle",
        "ca" => "calendar",
        _ => unimplemented!(),
    }
}

pub fn option_key_for_name(key: &str) -> &'static str {
    match key {
        "hour-cycle" => "hc",
        "calendar" => "ca",
        _ => unimplemented!(),
    }
}

pub fn apply_options(loc: &mut Locale, opts: BTreeMap<&str, &str>) {
    for (key, value) in opts {
        match key {
            // TODO: should we do something other than store None on strings
            // that fail representation?
            "language" => loc.language = TinyStr4::new(value).ok(),
            "script" => loc.script = TinyStr4::new(value).ok(),
            "region" => loc.region = TinyStr4::new(value).ok(),

            _ => if let Some(ref mut exts) = loc.extensions {
                let uext = exts
                    .entry("unicode".to_owned())
                    .or_insert_with(BTreeMap::new);
                uext.insert(key.to_owned(), value.to_owned());
            } else {
                let mut exts = BTreeMap::new();
                let mut uext = BTreeMap::new();
                uext.insert(key.to_owned(), value.to_owned());
                exts.insert("unicode".to_owned(), uext);
                loc.extensions = Some(exts);
            },
        }
    }
}
