use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;

use unic_locale::{Locale, ExtensionType};
use fluent_locale::negotiate::negotiate_languages;
use fluent_locale::negotiate::NegotiationStrategy;
use fluent_locale::parse_accepted_languages;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct LocaleTestInputData {
    string: String,
    options: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct LocaleTestOutputObject {
    language: Option<String>,
    script: Option<String>,
    region: Option<String>,
    variants: Option<Vec<String>>,
    extensions: Option<HashMap<String, HashMap<String, String>>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum LocaleTestOutput {
    String(String),
    Object(LocaleTestOutputObject),
}

#[derive(Serialize, Deserialize)]
struct LocaleTestSet {
    input: LocaleTestInputData,
    output: LocaleTestOutput,
}

fn read_locale_testsets<P: AsRef<Path>>(path: P) -> Result<Vec<LocaleTestSet>, Box<Error>> {
    let file = File::open(path)?;
    let sets = serde_json::from_reader(file)?;
    Ok(sets)
}

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

fn test_locale_fixtures(path: &str) {
    let tests = read_locale_testsets(path).unwrap();

    for test in tests {
        let s = test.input.string;

        let loc;
        if let Some(opts) = test.input.options {
            let borrowed: HashMap<&str, &str> =
                opts.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
            loc = Locale::from_str_with_options(&s, borrowed).unwrap();
        } else {
            loc = Locale::from_str(&s).unwrap();
        }

        match test.output {
            LocaleTestOutput::Object(o) => {
                let mut ref_locale = Locale::new();
                if let Some(language) = o.language {
                    ref_locale.set_language(Some(language.as_str())).unwrap();
                }
                if let Some(script) = o.script {
                    ref_locale.set_script(Some(script.as_str())).unwrap();
                }
                if let Some(region) = o.region {
                    ref_locale.set_region(Some(region.as_str())).unwrap();
                }
                if let Some(variants) = o.variants {
                    ref_locale.set_variants(
                        &variants.iter().map(String::as_str).collect::<Vec<&str>>()).unwrap();
                }
                if let Some(extensions) = o.extensions {
                    for (ext_name, values) in extensions {
                        let ext = match ext_name.as_str() {
                            "unicode" => ExtensionType::Unicode,
                            _ => unimplemented!()
                        };
                        for (key, val) in values {
                            ref_locale.set_extension(ext, &key, &val).unwrap();
                        }
                    }
                }
                assert_eq!(loc, ref_locale);
            }
            LocaleTestOutput::String(s) => {
                assert_eq!(loc.to_string(), s);
            }
        }
    }
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
            NegotiateTestInput::NoDefault(r, a) => {
                // One with &str
                let requested: Vec<&str> = r.iter().map(|v| v.as_str()).collect();
                let available: Vec<&str> = a.iter().map(|v| v.as_str()).collect();
                assert_eq!(
                    negotiate_languages(&requested, &available, None, &strategy),
                    test.output,
                    "Test in {} failed",
                    path
                );
            }
            NegotiateTestInput::Default(requested, available, default) => {
                // One with String
                assert_eq!(
                    negotiate_languages(&requested, &available, Some(default.as_str()), &strategy),
                    test.output,
                    "Test in {} failed",
                    path
                );
            }
        }
    }
}

#[test]
fn parse() {
    test_locale_fixtures("./tests/fixtures/locale/parsing.json");
}

#[test]
fn parse_ext() {
    test_locale_fixtures("./tests/fixtures/locale/parsing-ext.json");
}

#[test]
fn serialize() {
    test_locale_fixtures("./tests/fixtures/locale/serialize-options.json");
}

#[test]
fn options() {
    test_locale_fixtures("./tests/fixtures/locale/options.json");
}

#[test]
fn options_ext() {
    test_locale_fixtures("./tests/fixtures/locale/options-ext.json");
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
        assert_eq!(test.output, locales);
    }
}

#[test]
fn test_locale_parsing_error() {
    let loc = Locale::from_str("verybroken-tag");
    assert_eq!(loc.is_err(), true);

    if let Err(err) = loc {
        assert_eq!(format!("{}", err), "Language Identifier Parser Error");
    }
}
