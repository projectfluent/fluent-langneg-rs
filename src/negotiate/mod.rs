use super::locale::Locale;

pub fn filter_matches(requested: Vec<&str>, available: Vec<&str>) -> Vec<String> {

    let available_locales: Vec<Locale> = available
        .iter()
        .map(|loc| Locale::new(loc, None).unwrap())
        .collect();

    let mut supported_locales = vec![];

    for req_loc_str in requested {
        let requested_locale = Locale::new(req_loc_str, None).unwrap();

        if requested_locale.language.is_none() {
            continue;
        }

        for available_locale in available_locales.iter() {
            if requested_locale.language == available_locale.language {
                supported_locales.push(available_locale.to_string());
            }
        }
    }

    return supported_locales;
}

pub fn negotiate_languages(requested: Vec<&str>, available: Vec<&str>) -> Vec<String> {
    let supported = filter_matches(requested, available);
    return supported;
}
