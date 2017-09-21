//! Language Negotiation is a process in which locales from different
//! sources are filtered and sorted in an effort to produce the best
//! possible selection of them.
//!
//! There are multiple language negotiation strategies, most popular is
//! described in [RFC4647](https://www.ietf.org/rfc/rfc4647.txt).
//!
//! The algorithm is based on the BCP4647 3.3.2 Extended Filtering algorithm,
//! with several modifications.
//!
//! # Example:
//!
//! ```
//! let supported = negotiate_languages(
//!   vec!["pl", "fr", "en-US"],                // requested
//!   vec!["it", "de", "fr", "en-GB", "en-US"]  // available
//! );
//! assert_eq!(supported, vec!["fr", "en-US"]);
//! ```
//!
//! # The exact algorithm is custom, and consists of a 6 level strategy:
//!
//! ### 1) Attempt to find an exact match for each requested locale in available locales.
//!
//! Example:
//!
//! ```
//! // [requested] * [available] = [supported]
//!
//! ["en-US"] * ["en-US"] = ["en-US"]
//! ```
//!
//! ### 2) Attempt to match a requested locale to an available locale treated as a locale range.
//!
//! Example:
//!
//! ```
//! // [requested] * [available] = [supported]
//!
//! ["en-US"] * ["en"] = ["en"]
//!               ^^
//!                |-- becomes "en-*-*-*"
//! ```
//!
//! ### 3) Maximize the requested locale to find the best match in available locales.
//!
//! This part uses ICU's likelySubtags or similar database.
//!
//! Example:
//!
//! ```
//! // [requested] * [available] = [supported]
//!
//! ["en"] * ["en-GB", "en-US"] = ["en-US"]
//!   ^^       ^^^^^    ^^^^^
//!    |           |        |
//!    |           |----------- become "en-*-GB-*" and "en-*-US-*"
//!    |
//!    |-- ICU likelySubtags expands it to "en-Latn-US"
//! ```
//!
//! ### 4) Attempt to look up for a different variant of the same locale.
//!
//! Example:
//!
//! ```
//! // [requested] * [available] = [supported]
//!
//! ["ja-JP-win"] * ["ja-JP-mac"] = ["ja-JP-mac"]
//!   ^^^^^^^^^       ^^^^^^^^^
//!           |               |-- become "ja-*-JP-mac"
//!           |
//!           |----------- replace variant with range: "ja-JP-*"
//! ```
//!
//! ### 5) Look up for a maximized version of the requested locale, stripped of the region code.
//!
//! Example:
//!
//! ```
//! // [requested] * [available] = [supported]
//!
//! ["en-CA"] * ["en-ZA", "en-US"] = ["en-US", "en-ZA"]
//!   ^^^^^
//!       |       ^^^^^    ^^^^^
//!       |           |        |
//!       |           |----------- become "en-*-ZA-*" and "en-*-US-*"
//!       |
//!       |----------- strip region produces "en", then lookup likelySubtag: "en-Latn-US"
//! ```
//!
//!
//! ### 6) Attempt to look up for a different region of the same locale.
//!
//! Example:
//!
//! ```
//! // [requested] * [available] = [supported]
//!
//! ["en-GB"] * ["en-AU"] = ["en-AU"]
//!   ^^^^^       ^^^^^
//!       |           |-- become "en-*-AU-*"
//!       |
//!       |----- replace region with range: "en-*"
//! ```
//!

use super::locale::Locale;

fn filter_matches(requested: Vec<&str>, available: Vec<&str>) -> Vec<String> {

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
