extern crate fluent_locale;
extern crate serde_json;

use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

use self::fluent_locale::locale::Locale;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct TestInputData {
    string: String,
    options: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TestOutput {
    language: Option<String>,
    script: Option<String>,
    region: Option<String>,
    variants: Option<Vec<String>>,
    extensions: Option<HashMap<String, HashMap<String, String>>>,
}

#[derive(Serialize, Deserialize)]
struct TestSet {
    input: TestInputData,
    output: TestOutput,
}

fn read_testsets<P: AsRef<Path>>(path: P) -> Result<Vec<TestSet>, Box<Error>> {
    let file = File::open(path)?;
    let sets = serde_json::from_reader(file)?;
    Ok(sets)
}

#[test]
fn parse() {
    let tests = read_testsets("./tests/fixtures/locale/parsing.json").unwrap();

    for test in tests {
        let s = test.input.string;

        let loc = Locale::new(&s, None).unwrap();

        assert_eq!(loc.language, test.output.language);
        assert_eq!(loc.script, test.output.script);
        assert_eq!(loc.region, test.output.region);
        assert_eq!(loc.variants, test.output.variants);
        assert_eq!(loc.extensions, test.output.extensions);
    }
}

#[test]
fn parse_ext() {
    let tests = read_testsets("./tests/fixtures/locale/parsing-ext.json").unwrap();

    for test in tests {
        let s = test.input.string;

        let loc = Locale::new(&s, None).unwrap();

        assert_eq!(loc.language, test.output.language);
        assert_eq!(loc.script, test.output.script);
        assert_eq!(loc.region, test.output.region);
        assert_eq!(loc.variants, test.output.variants);
        assert_eq!(loc.extensions, test.output.extensions);
    }
}

// #[test]
// fn options() {
//     let tests = read_testsets("./tests/fixtures/locale/options.json").unwrap();

//     for test in tests {
//         let s = test.input.string;

//         let loc = Locale::new(&s, test.input.options).unwrap();

//         assert_eq!(loc.language, test.output.language);
//         assert_eq!(loc.script, test.output.script);
//         assert_eq!(loc.region, test.output.region);
//         assert_eq!(loc.variants, test.output.variants);
//         assert_eq!(loc.extensions, test.output.extensions);
//     }
// }

// #[test]
// fn options_ext() {
//     let tests = read_testsets("./tests/fixtures/locale/options-ext.json").unwrap();

//     for test in tests {
//         let s = test.input.string;

//         let loc = Locale::new(&s, None).unwrap();

//         assert_eq!(loc.language, test.output.language);
//         assert_eq!(loc.script, test.output.script);
//         assert_eq!(loc.region, test.output.region);
//         assert_eq!(loc.variants, test.output.variants);
//         assert_eq!(loc.extensions, test.output.extensions);
//     }
// }
