//! Emits the seeded locale-pack compatibility report and core pack artifacts.
//!
//! ```sh
//! cargo run -q -p aureline-i18n --example dump_locale_pack_compatibility -- report
//! cargo run -q -p aureline-i18n --example dump_locale_pack_compatibility -- artifact locale-pack:core:es-mx
//! cargo run -q -p aureline-i18n --example dump_locale_pack_compatibility -- artifacts
//! ```
//!
//! Regenerate the canonical fixtures and checked-in core artifacts with:
//!
//! ```sh
//! cargo run -q -p aureline-i18n --example dump_locale_pack_compatibility -- report \
//!   > fixtures/i18n/pack-skew-and-signature/compatibility_report.json
//! cargo run -q -p aureline-i18n --example dump_locale_pack_compatibility -- artifact locale-pack:core:source:en-us \
//!   > locale-packs/core/en-US/pack.json
//! ```

use aureline_i18n::{
    seeded_core_locale_pack_artifacts, seeded_locale_pack_compatibility_report,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let report = seeded_locale_pack_compatibility_report();
    report
        .validate()
        .map_err(|findings| format!("seeded compatibility report failed validation: {findings:?}"))?;

    match args.first().map(String::as_str) {
        Some("report") | None => print_json(&report)?,
        Some("artifacts") => print_json(&seeded_core_locale_pack_artifacts())?,
        Some("artifact") => {
            let pack_id = args
                .get(1)
                .ok_or("usage: artifact <pack_id>")?
                .as_str();
            let artifact = seeded_core_locale_pack_artifacts()
                .into_iter()
                .find(|artifact| artifact.pack_id == pack_id)
                .ok_or_else(|| format!("unknown core pack id: {pack_id}"))?;
            artifact
                .validate()
                .map_err(|findings| format!("artifact failed validation: {findings:?}"))?;
            print_json(&artifact)?;
        }
        Some("validate") => match report.validate() {
            Ok(()) => println!("ok"),
            Err(findings) => {
                for finding in findings {
                    eprintln!("{}: {}", finding.row_ref, finding.message);
                }
                std::process::exit(3);
            }
        },
        Some(other) => return Err(format!("unknown selector: {other}").into()),
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
