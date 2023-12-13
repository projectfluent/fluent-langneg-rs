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
//! use fluent_langneg::negotiate_languages;
//! use fluent_langneg::NegotiationStrategy;
//! use fluent_langneg::convert_vec_str_to_langids_lossy;
//! use icu_locid::LanguageIdentifier;
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

use icu_locid::LanguageIdentifier;

#[cfg(not(feature = "cldr"))]
mod likely_subtags;
#[cfg(feature = "cldr")]
use icu_locid_transform::{LocaleExpander, TransformResult};
#[cfg(not(feature = "cldr"))]
use likely_subtags::{LocaleExpander, TransformResult};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum NegotiationStrategy {
    Filtering,
    Matching,
    Lookup,
}

fn subtag_matches<P: PartialEq>(
    subtag1: &Option<P>,
    subtag2: &Option<P>,
    as_range1: bool,
    as_range2: bool,
) -> bool {
    (as_range1 && subtag1.is_none()) || (as_range2 && subtag2.is_none()) || subtag1 == subtag2
}

#[inline]
fn matches(
    lid1: &LanguageIdentifier,
    lid2: &LanguageIdentifier,
    range1: bool,
    range2: bool,
) -> bool {
    ((range1 && lid1.language.is_empty())
        || (range2 && lid2.language.is_empty())
        || lid1.language == lid2.language)
        && subtag_matches(&lid1.script, &lid2.script, range1, range2)
        && subtag_matches(&lid1.region, &lid2.region, range1, range2)
        && ((range1 && lid1.variants.is_empty())
            || (range2 && lid2.variants.is_empty())
            || lid1.variants == lid2.variants)
}

pub fn filter_matches<'a, R: 'a + AsRef<LanguageIdentifier>, A: 'a + AsRef<LanguageIdentifier>>(
    requested: &[R],
    available: &'a [A],
    strategy: NegotiationStrategy,
) -> Vec<&'a A> {
    let mut lc: Option<LocaleExpander> = None;

    let mut supported_locales = vec![];

    let mut available_locales: Vec<&A> = available.iter().collect();

    macro_rules! test_strategy {
        ($req:ident, $self_as_range:expr, $other_as_range:expr) => {{
            let mut match_found = false;
            available_locales.retain(|locale| {
                if strategy != NegotiationStrategy::Filtering && match_found {
                    return true;
                }

                if matches(locale.as_ref(), &$req, $self_as_range, $other_as_range) {
                    match_found = true;
                    supported_locales.push(*locale);
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
        }};
    }

    for req in requested {
        let req = req.as_ref();

        // 1) Try to find a simple (case-insensitive) string match for the request.
        test_strategy!(req, false, false);

        // 2) Try to match against the available locales treated as ranges.
        test_strategy!(req, true, false);

        // Per Unicode TR35, 4.4 Locale Matching, we don't add likely subtags to
        // requested locales, so we'll skip it from the rest of the steps.
        if req.language.is_empty() {
            continue;
        }

        let mut req = req.to_owned();
        // 3) Try to match against a maximized version of the requested locale
        let lc = lc.get_or_insert_with(LocaleExpander::new);
        if lc.maximize(&mut req) == TransformResult::Modified {
            test_strategy!(req, true, false);
        }

        // 4) Try to match against a variant as a range
        req.variants.clear();
        test_strategy!(req, true, true);

        // 5) Try to match against the likely subtag without region
        req.region = None;
        if lc.maximize(&mut req) == TransformResult::Modified {
            test_strategy!(req, true, false);
        }

        // 6) Try to match against a region as a range
        req.region = None;
        test_strategy!(req, true, true);
    }

    supported_locales
}

pub fn negotiate_languages<
    'a,
    R: 'a + AsRef<LanguageIdentifier>,
    A: 'a + AsRef<LanguageIdentifier> + PartialEq,
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
        } else if !supported.contains(&default) {
            supported.push(default);
        }
    }
    supported
}
