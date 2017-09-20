use super::locale::Locale;

pub fn filter_matches(requested: Vec<&str>, available: Vec<&str>) -> Vec<String> {

    let available_locales: Vec<Locale> = available.iter().map(|loc| Locale::from(*loc)).collect();

    let mut supported_locales = vec![];

    for req_loc_str in requested {
        let requested_locale = Locale::from(req_loc_str);

        if requested_locale.get_language().is_empty() {
            continue;
        }

        for available_locale in &available_locales {
            if requested_locale.get_language() == available_locale.get_language() {
                supported_locales.push(available_locale.to_string());
            }
        }
    }

    supported_locales
}

pub fn negotiate_languages(requested: Vec<&str>, available: Vec<&str>) -> Vec<String> {
    let supported = filter_matches(requested, available);
    supported
}
