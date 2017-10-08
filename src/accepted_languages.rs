//! This function parses Accept-Language string into a list of language tags that
//! can be later passed to language negotiation functions.
//!
//! # Example:
//!
//! ```
//! use fluent_locale::negotiate_languages;
//! use fluent_locale::NegotiationStrategy;
//! use fluent_locale::parse_accepted_languages;
//!
//! let requested = parse_accepted_languages("de-AT;0.9,de-DE;0.8,de;0.7;en-US;0.5");
//!
//! let supported = negotiate_languages(
//!   requested,
//!   vec!["fr", "pl", "de", "en-US"],
//!   Some("en-US"),
//!   NegotiationStrategy::Filtering
//! );
//! assert_eq!(supported, vec!["de", "en-US"]);
//! ```
//!
//! This function ignores the weights associated with the locales, since Fluent Locale
//! language negotiation only uses the order of locales, not the weights.
//!
pub fn parse(s: &str) -> Vec<&str> {
    s.split(',')
        .map(|t| t.trim().split(';').nth(0).unwrap())
        .filter(|t| !t.is_empty())
        .collect()
}
