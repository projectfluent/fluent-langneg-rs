extern crate fluent_locale;

use std::collections::HashMap;
use fluent_locale::Locale;

fn main() {
    let locale = Locale::from("en-US");

    println!("======");
    println!("Locale: {}", locale);
    println!("-----");
    println!("language: {:?}", locale.language);
    println!("script: {:?}", locale.script);
    println!("region: {:?}", locale.region);
    println!("======\n\n");



    let mut locale = Locale::from("de-DE");
    locale.region = Some("AT".to_owned());

    println!("======");
    println!("Locale: {}", locale);
    println!("======\n\n");



    let mut options = HashMap::new();
    options.insert("hour-cycle", "h12");

    let locale = Locale::new("it-IT", Some(options)).unwrap();

    println!("======");
    println!("Locale: {}", locale);
    println!("======\n\n");
}
