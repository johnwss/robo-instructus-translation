//! Generate missing translation files use `features "fill"`
//!
//! env GOOGLE_TRANSLATE_API_KEY=abc cargo run --bin fill --features fill -- --google-translate
//!
//! cargo run --bin fill --features fill
#![cfg(feature = "fill")]

use lazy_static::lazy_static;
use rustc_hash::{FxHashMap, FxHashSet};
use std::env;
use std::io::Write;

// REPLACE: FxHashMap<&str, FxHashMap<&str, &str>>: { "ru" -> { "yes" -> "да", .. }, .. }
include!("../../target/generated/translations.rs");

fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "robo_instructus=debug");
    }
    env_logger::init();

    let machine_translate = env::args().any(|a| a == "--google-translate");

    if machine_translate {
        robo_instructus_translation::realtime::external_init();
    }

    let all_en: FxHashSet<_> = REPLACE.values().flat_map(FxHashMap::keys).collect();

    // run all keys through all known languages to produce missing tranlations
    for (lang, translations) in REPLACE.iter() {
        for en in &all_en {
            if translations.get(*en).is_none() {
                if machine_translate {
                    robo_instructus_translation::realtime::google_translate(lang, en).unwrap();
                } else {
                    let mut file = std::fs::OpenOptions::new()
                        .append(true)
                        .open(&format!("en-replace.{}.pairs", lang))
                        .unwrap();
                    writeln!(file, "{}\n{}\n", en, en).unwrap();
                }
            }
        }
    }
}
