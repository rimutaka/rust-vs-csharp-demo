/// C# vs Rust comparison demo
///
/// 1. Shadowing and overloading (`use error`, `let individual`)
/// 2. Linitng, compiler messages
/// 3. Nullability vs Error / Option (`let individual`)
/// 4. Code formatting
/// 5. Traits (`Display`)
/// 6. Enums with types (`enum Individual`)
/// 7. Targets:
///     * `rustc --print target-list`
///     * `rustup target add x86_64-unknown-linux-musl`
///     * `cargo build --target x86_64-unknown-linux-musl`
///     * `cargo build --target x86_64-unknown-linux-gnu`
/// 8. Offline builds: `cargo vendor`
use serde::Deserialize;

// use anyhow::Error;
// use std::error::Error as StdError;
// type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, Deserialize)]
enum Suffixes {
    Jr,
    Snr,
    PhD,
    // MD
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct IndividualDetails {
    pub given_name: String,
    pub middle_name: Option<String>,
    pub family_name: String,
    pub age: u32,
    pub suffix: Suffixes,
}

// #[derive(Debug, Deserialize)]
// enum Individual {
//     Parsed(IndividualDetails),
//     Simple(&'static str),
// }

// impl std::fmt::Display for Individual {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match &self {
//             Individual::Parsed(v) => {
//                 write!(f, "Our Parsed individual: {} {}", v.given_name, v.family_name)
//             }
//             Individual::Simple(v) => write!(f, "Our Simple individual: {}", v),
//         }
//     }
// }

const INDIVIDUAL_JSON_OR_STRING: &str = r#"{
    "given_name": "Jim",
    "family_name": "Bolger",
    "age": 0,
    "suffix":"PhD"
  }"#;

// const INDIVIDUAL_JSON_OR_STRING: &str = "Jim Bolger, PhD";

fn main() {
    println!("Hello, world!");

    let individual = serde_json::from_str::<IndividualDetails>(INDIVIDUAL_JSON_OR_STRING);
    // let individual = serde_json::from_str::<IndividualDetails>(INDIVIDUAL_JSON_OR_STRING).unwrap();
    // let individual = serde_json::from_str::<IndividualDetails>(INDIVIDUAL_JSON_OR_STRING).expect("JSON to struct err:");
    // let individual = serde_json::from_str::<IndividualDetails>(INDIVIDUAL_JSON_OR_STRING)?;
    // let individual = match serde_json::from_str::<IndividualDetails>(INDIVIDUAL_JSON_OR_STRING) {
    //     Ok(v) => Individual::Parsed(v),
    //     Err(e) => {
    //         eprintln!("JSON to struct err: {}", e);
    //         Individual::Simple(INDIVIDUAL_JSON_OR_STRING)
    //     }
    // };

    println!("IndividualDetails struct: {:?}", individual);
    // println!("Individual enum: {}", individual);

    // if let Individual::Simple(individual_as_str) = individual {
    //     println!("Our individual: {}", individual_as_str);
    // }

    // if let Individual::Parsed(individual_as_struct) = individual {
    //     match individual_as_struct.suffix {
    //         Suffixes::Jr | Suffixes::Snr => {
    //             println!("From a big family")
    //         }
    //         Suffixes::Phd => println!("Really clever"),
    //         _ => println!("Whatever!"),
    //     }
    // }
}
