use std::collections::HashMap;
use super::Locale;

pub fn option_name_for_key(key: &str) -> &'static str {
    match key {
        "hc" => "hour-cycle",
        _ => unimplemented!(),
    }
}

pub fn option_key_for_name(key: &str) -> &'static str {
    match key {
        "hour-cycle" => "hc",
        _ => unimplemented!(),
    }
}


pub fn apply_options(loc: &mut Locale, opts: HashMap<&str, &str>) {
    for (key, value) in opts {
        match key {
            "language" => loc.language = Some(value.to_owned()),
            "script" => loc.script = Some(value.to_owned()),
            "region" => loc.region = Some(value.to_owned()),

            "hour-cycle" => {
                if let Some(ref mut exts) = loc.extensions {
                    if let Some(uext) = exts.get_mut("unicode") {
                        uext.insert("hour-cycle".to_owned(), value.to_owned());
                    }
                }
            }
            _ => {}
        }
    }
}
