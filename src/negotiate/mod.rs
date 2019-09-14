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
//! use fluent_locale::negotiate_languages;
//! use fluent_locale::NegotiationStrategy;
//! use fluent_locale::convert_vec_str_to_langids_lossy;
//! use unic_langid::LanguageIdentifier;
//!
//! let requested = convert_vec_str_to_langids_lossy(&["pl", "fr", "en-US"]);
//! let available = convert_vec_str_to_langids_lossy(&["it", "de", "fr", "en-GB", "en_US"]);
//! let default: LanguageIdentifier = "en-US".parse().expect("Parsing langid failed.");
//!
//! let supported = negotiate_languages(
//!   &requested,
//!   &available,
//!   Some(&default),
//!   NegotiationStrategy::Filtering
//! );
//!
//! let expected = convert_vec_str_to_langids_lossy(&["fr", "en-US", "en-GB"]);
//! assert_eq!(supported,
//!            expected.iter().map(|t| t.as_ref()).collect::<Vec<&LanguageIdentifier>>());
//! ```
//!
//! # The exact algorithm is custom, and consists of a 6 level strategy:
//!
//! ### 1) Attempt to find an exact match for each requested locale in available locales.
//!
//! Example:
//!
//! ```text
//! // [requested] * [available] = [supported]
//!
//! ["en-US"] * ["en-US"] = ["en-US"]
//! ```
//!
//! ### 2) Attempt to match a requested locale to an available locale treated as a locale range.
//!
//! Example:
//!
//! ```text
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
//! ```text
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
//! ```text
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
//! ```text
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
//! ```text
//! // [requested] * [available] = [supported]
//!
//! ["en-GB"] * ["en-AU"] = ["en-AU"]
//!   ^^^^^       ^^^^^
//!       |           |-- become "en-*-AU-*"
//!       |
//!       |----- replace region with range: "en-*"
//! ```
//!

use std::collections::HashMap;
use unic_langid::LanguageIdentifier;
mod likely_subtags;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum NegotiationStrategy {
    Filtering,
    Matching,
    Lookup,
}

pub fn filter_matches<'a, R: 'a + AsRef<LanguageIdentifier>, A: 'a + AsRef<LanguageIdentifier>>(
    requested: &[R],
    available: &'a [A],
    strategy: NegotiationStrategy,
) -> Vec<&'a A> {
    let mut supported_locales = vec![];

    let mut av_map: HashMap<&'a LanguageIdentifier, &'a A> = HashMap::new();

    for av in available.into_iter() {
        av_map.insert(av.as_ref(), av);
    }

    for req in requested {
        let req = req.as_ref();
        if req.get_language() == "und" {
            continue;
        }

        let mut match_found = false;

        // 1) Try to find a simple (case-insensitive) string match for the request.
        av_map.retain(|key, value| {
            if strategy != NegotiationStrategy::Filtering && match_found {
                return true;
            }

            if key.matches(&req, false, false) {
                match_found = true;
                supported_locales.push(*value);
                return false;
            }
            true
        });

        if match_found {
            match strategy {
                NegotiationStrategy::Filtering => {}
                NegotiationStrategy::Matching => continue,
                NegotiationStrategy::Lookup => break,
            }
        }

        match_found = false;

        // 2) Try to match against the available locales treated as ranges.
        av_map.retain(|key, value| {
            if strategy != NegotiationStrategy::Filtering && match_found {
                return true;
            }

            if key.matches(&req, true, false) {
                match_found = true;
                supported_locales.push(*value);
                return false;
            }
            true
        });

        if match_found {
            match strategy {
                NegotiationStrategy::Filtering => {}
                NegotiationStrategy::Matching => continue,
                NegotiationStrategy::Lookup => break,
            };
        }

        match_found = false;

        // 3) Try to match against a maximized version of the requested locale
        let mut req = if let Some(req) = likely_subtags::add(req) {
            av_map.retain(|key, value| {
                if strategy != NegotiationStrategy::Filtering && match_found {
                    return true;
                }

                if key.matches(&req, true, false) {
                    match_found = true;
                    supported_locales.push(*value);
                    return false;
                }
                true
            });

            if match_found {
                match strategy {
                    NegotiationStrategy::Filtering => {}
                    NegotiationStrategy::Matching => continue,
                    NegotiationStrategy::Lookup => break,
                };
            }

            match_found = false;
            req
        } else {
            req.to_owned()
        };

        // 4) Try to match against a variant as a range
        req.set_variants(&[]).unwrap();
        av_map.retain(|key, value| {
            if strategy != NegotiationStrategy::Filtering && match_found {
                return true;
            }

            if key.matches(&req, true, true) {
                match_found = true;
                supported_locales.push(*value);
                return false;
            }
            true
        });

        if match_found {
            match strategy {
                NegotiationStrategy::Filtering => {}
                NegotiationStrategy::Matching => continue,
                NegotiationStrategy::Lookup => break,
            };
        }

        match_found = false;

        // 5) Try to match against the likely subtag without region
        req.set_region(None).unwrap();
        if let Some(req) = likely_subtags::add(&req) {
            av_map.retain(|key, value| {
                if strategy != NegotiationStrategy::Filtering && match_found {
                    return true;
                }

                if key.matches(&req, true, false) {
                    match_found = true;
                    supported_locales.push(*value);
                    return false;
                }
                true
            });

            if match_found {
                match strategy {
                    NegotiationStrategy::Filtering => {}
                    NegotiationStrategy::Matching => continue,
                    NegotiationStrategy::Lookup => break,
                };
            }

            match_found = false;
        }

        // 6) Try to match against a region as a range
        req.set_region(None).unwrap();
        av_map.retain(|key, value| {
            if strategy != NegotiationStrategy::Filtering && match_found {
                return true;
            }

            if key.matches(&req, true, true) {
                match_found = true;
                supported_locales.push(*value);
                return false;
            }
            true
        });

        if match_found {
            match strategy {
                NegotiationStrategy::Filtering => {}
                NegotiationStrategy::Matching => continue,
                NegotiationStrategy::Lookup => break,
            };
        }
    }

    supported_locales
}

pub fn negotiate_languages<
    'a,
    R: 'a + AsRef<LanguageIdentifier>,
    A: 'a + AsRef<LanguageIdentifier>,
>(
    requested: &[R],
    available: &'a [A],
    default: Option<&'a A>,
    strategy: NegotiationStrategy,
) -> Vec<&'a A> {
    let mut supported = filter_matches(requested, available, strategy);

    if let Some(default) = default {
        if strategy == NegotiationStrategy::Lookup {
            if supported.is_empty() {
                supported.push(default);
            }
        } else if !supported.iter().any(|s| s.as_ref() == default.as_ref()) {
            supported.push(default);
        }
    }
    supported
}
