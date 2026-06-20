//! Emits the seeded M5 message-id registry and its continuity artifacts.
//!
//! ```sh
//! cargo run -q -p aureline-i18n --example dump_m5_message_registry -- registry
//! cargo run -q -p aureline-i18n --example dump_m5_message_registry -- baseline
//! cargo run -q -p aureline-i18n --example dump_m5_message_registry -- continuity
//! cargo run -q -p aureline-i18n --example dump_m5_message_registry -- render es-MX
//! cargo run -q -p aureline-i18n --example dump_m5_message_registry -- summary
//! ```

use aureline_i18n::{seeded_m5_message_id_baseline, seeded_m5_message_registry};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let registry = seeded_m5_message_registry();
    let baseline = seeded_m5_message_id_baseline();
    registry
        .validate()
        .map_err(|findings| format!("seeded registry failed validation: {findings:?}"))?;
    baseline
        .validate()
        .map_err(|findings| format!("seeded baseline failed validation: {findings:?}"))?;

    match args.first().map(String::as_str) {
        Some("registry") | None => print_json(&registry)?,
        Some("baseline") => print_json(&baseline)?,
        Some("continuity") => print_json(&registry.continuity_against(&baseline))?,
        Some("summary") => print_json(&registry.summary)?,
        Some("render") => {
            let locale = args.get(1).map(String::as_str).unwrap_or("en-US");
            print_json(&registry.render(locale))?;
        }
        Some(other) => return Err(format!("unknown selector: {other}").into()),
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
