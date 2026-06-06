//! Headless inspector for the beta locale-pack contract.
//!
//! The binary emits deterministic JSON records consumed by fixtures, docs,
//! support export review, and surface parity checks.

use aureline_i18n::{
    seeded_dense_i18n_conformance_corpus, seeded_dense_i18n_conformance_review_packet,
    seeded_locale_pack_beta_contract, seeded_locale_pack_help_about_projection,
    seeded_locale_pack_settings_projection, seeded_locale_pack_support_export,
    seeded_locale_pack_support_projection, seeded_stable_locale_lifecycle_parity_packet,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let contract = seeded_locale_pack_beta_contract();
    match args.first().map(String::as_str) {
        Some("manifest") | None => print_json(&contract)?,
        Some("settings") => print_json(&seeded_locale_pack_settings_projection())?,
        Some("help-about") => print_json(&seeded_locale_pack_help_about_projection())?,
        Some("support-projection") => print_json(&seeded_locale_pack_support_projection())?,
        Some("support-export") => print_json(&seeded_locale_pack_support_export())?,
        Some("dense-corpus") => print_json(&seeded_dense_i18n_conformance_corpus())?,
        Some("dense-review") => print_json(&seeded_dense_i18n_conformance_review_packet())?,
        Some("stable-lifecycle") => print_json(&seeded_stable_locale_lifecycle_parity_packet())?,
        Some("stable-lifecycle-validate") => {
            match seeded_stable_locale_lifecycle_parity_packet().validate() {
                Ok(()) => println!("ok"),
                Err(findings) => {
                    for finding in findings {
                        eprintln!("{}: {}", finding.row_ref, finding.message);
                    }
                    std::process::exit(3);
                }
            }
        }
        Some("dense-validate") => match seeded_dense_i18n_conformance_corpus().validate() {
            Ok(()) => println!("ok"),
            Err(findings) => {
                for finding in findings {
                    eprintln!("{}: {}", finding.row_ref, finding.message);
                }
                std::process::exit(3);
            }
        },
        Some("validate") => match contract.validate() {
            Ok(()) => println!("ok"),
            Err(findings) => {
                for finding in findings {
                    eprintln!("{}: {}", finding.row_ref, finding.message);
                }
                std::process::exit(3);
            }
        },
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(value)?;
    println!("{json}");
    Ok(())
}
