use std::convert::TryInto;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;

use fluent_locale::negotiate::negotiate_languages;
use fluent_locale::negotiate::NegotiationStrategy;
use fluent_locale::parse_accepted_languages;
use unic_langid::LanguageIdentifier;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum NegotiateTestInput {
    NoDefault(Vec<String>, Vec<String>),
    Default(Vec<String>, Vec<String>, String),
}

#[derive(Serialize, Deserialize)]
struct NegotiateTestSet {
    input: NegotiateTestInput,
    strategy: Option<String>,
    output: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct AcceptedLanguagesTestSet {
    input: String,
    output: Vec<String>,
}

fn read_negotiate_testsets<P: AsRef<Path>>(path: P) -> Result<Vec<NegotiateTestSet>, Box<Error>> {
    let file = File::open(path)?;
    let sets = serde_json::from_reader(file)?;
    Ok(sets)
}

fn test_negotiate_fixtures(path: &str) {
    let tests = read_negotiate_testsets(path).unwrap();

    for test in tests {
        let strategy = match test.strategy {
            Some(strategy) => match strategy.as_str() {
                "filtering" => NegotiationStrategy::Filtering,
                "matching" => NegotiationStrategy::Matching,
                "lookup" => NegotiationStrategy::Lookup,
                _ => NegotiationStrategy::Filtering,
            },
            _ => NegotiationStrategy::Filtering,
        };
        match test.input {
            NegotiateTestInput::NoDefault(requested, available) => {
                let requested: Vec<LanguageIdentifier> =
                    dbg!(requested.iter().map(|v| v.try_into().unwrap()).collect());
                let available: Vec<LanguageIdentifier> =
                    dbg!(available.iter().map(|v| v.try_into().unwrap()).collect());
                let output: Vec<LanguageIdentifier> =
                    test.output.iter().map(|v| v.try_into().unwrap()).collect();
                let output2: Vec<&LanguageIdentifier> = output.iter().map(|t| t.as_ref()).collect();
                assert_eq!(
                    negotiate_languages(&requested, &available, None, strategy),
                    output2,
                    "Test in {} failed",
                    path
                );
            }
            NegotiateTestInput::Default(requested, available, default) => {
                let requested: Vec<LanguageIdentifier> =
                    dbg!(requested.iter().map(|v| v.try_into().unwrap()).collect());
                let available: Vec<LanguageIdentifier> =
                    dbg!(available.iter().map(|v| v.try_into().unwrap()).collect());
                let output: Vec<LanguageIdentifier> =
                    test.output.iter().map(|v| v.try_into().unwrap()).collect();
                let output2: Vec<&LanguageIdentifier> = output.iter().map(|t| t.as_ref()).collect();
                assert_eq!(
                    dbg!(negotiate_languages(
                        &requested,
                        &available,
                        default.try_into().ok().as_ref(),
                        strategy
                    )),
                    output2,
                    "Test in {} failed",
                    path
                );
            }
        }
    }
}

#[test]
fn negotiate_filtering() {
    let paths = fs::read_dir("./tests/fixtures/negotiate/filtering").unwrap();

    for path in paths {
        let p = path.unwrap().path().to_str().unwrap().to_owned();
        if p.contains("available") {
            test_negotiate_fixtures(p.as_str());
        }
    }
}

#[test]
fn negotiate_matching() {
    let paths = fs::read_dir("./tests/fixtures/negotiate/matching").unwrap();

    for path in paths {
        let p = path.unwrap().path().to_str().unwrap().to_owned();
        test_negotiate_fixtures(p.as_str());
    }
}

#[test]
fn negotiate_lookup() {
    let paths = fs::read_dir("./tests/fixtures/negotiate/lookup").unwrap();

    for path in paths {
        let p = path.unwrap().path().to_str().unwrap().to_owned();
        test_negotiate_fixtures(p.as_str());
    }
}

#[test]
fn accepted_languages() {
    let file = File::open("./tests/fixtures/accepted_languages.json").unwrap();
    let tests: Vec<AcceptedLanguagesTestSet> = serde_json::from_reader(file).unwrap();

    for test in tests {
        let locales = parse_accepted_languages(test.input.as_str());
        let output: Vec<LanguageIdentifier> =
            test.output.iter().map(|v| v.try_into().unwrap()).collect();
        assert_eq!(output, locales);
    }
}
