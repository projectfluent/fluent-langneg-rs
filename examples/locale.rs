extern crate fluent_locale;

use std::collections::BTreeMap;
use fluent_locale::Locale;

fn main() {
    let locale = Locale::from("en-US");

    println!("======");
    println!("Locale: {}", locale);
    println!("-----");
    println!("language: {:?}", locale.get_language());
    println!("script: {:?}", locale.get_script());
    println!("region: {:?}", locale.get_region());
    println!("======\n\n");



    let mut locale = Locale::from("de-DE");
    locale.set_region("AT").unwrap();

    println!("======");
    println!("Locale: {}", locale);
    println!("======\n\n");



    let mut options = BTreeMap::new();
    options.insert("hour-cycle", "h12");

    let locale = Locale::new("it-IT", Some(options)).unwrap();

    println!("======");
    println!("Locale: {}", locale);
    println!("======\n\n");
}
