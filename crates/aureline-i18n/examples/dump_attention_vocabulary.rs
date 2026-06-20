//! Emits the governed attention/lifecycle vocabulary glossary and audit.
//!
//! ```sh
//! cargo run -q -p aureline-i18n --example dump_attention_vocabulary -- glossary
//! cargo run -q -p aureline-i18n --example dump_attention_vocabulary -- parity
//! cargo run -q -p aureline-i18n --example dump_attention_vocabulary -- drift
//! ```

use aureline_i18n::{
    seeded_attention_vocabulary_drift_scenarios, seeded_attention_vocabulary_glossary,
    seeded_attention_vocabulary_parity_report,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let glossary = seeded_attention_vocabulary_glossary();
    glossary
        .validate()
        .map_err(|findings| format!("seeded glossary failed validation: {findings:?}"))?;

    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("glossary") | None => print_json(&glossary)?,
        Some("parity") => print_json(&seeded_attention_vocabulary_parity_report())?,
        Some("drift") => print_json(&seeded_attention_vocabulary_drift_scenarios())?,
        Some(other) => return Err(format!("unknown selector: {other}").into()),
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
