//! Emits the seeded M5 translated-help pack, per-locale render, and parity report.
//!
//! ```sh
//! cargo run -q -p aureline-i18n --example dump_m5_translated_help_packs -- pack
//! cargo run -q -p aureline-i18n --example dump_m5_translated_help_packs -- parity
//! cargo run -q -p aureline-i18n --example dump_m5_translated_help_packs -- render ar-SA
//! ```

use aureline_i18n::{
    seeded_m5_translated_help_pack, seeded_m5_translated_help_parity_report,
    seeded_m5_translated_help_render,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let pack = seeded_m5_translated_help_pack();
    pack.validate()
        .map_err(|findings| format!("seeded pack failed validation: {findings:?}"))?;
    let parity = seeded_m5_translated_help_parity_report();
    parity
        .validate()
        .map_err(|findings| format!("seeded parity report failed validation: {findings:?}"))?;

    match args.first().map(String::as_str) {
        Some("pack") | None => print_json(&pack)?,
        Some("parity") => print_json(&parity)?,
        Some("render") => {
            let locale = args.get(1).map(String::as_str).unwrap_or("en-US");
            print_json(&seeded_m5_translated_help_render(locale))?;
        }
        Some(other) => return Err(format!("unknown selector: {other}").into()),
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
