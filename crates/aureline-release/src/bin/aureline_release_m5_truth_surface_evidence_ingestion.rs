//! Inspect the M5 truth-surface evidence-ingestion register.
//!
//! Subcommands:
//!
//! - (default) / `register` — print the full register as pretty JSON;
//! - `validate` — run `validate()` and exit non-zero on any violation;
//! - `surface <name>` — print the rows for one truth surface
//!   (`help_about`, `service_health`, `release_center`, `support_export`,
//!   `public_truth_pack`).

use std::process::ExitCode;

use aureline_release::m5_truth_surface_evidence_ingestion::{
    current_m5_truth_surface_ingestion, TruthSurface,
};

fn main() -> ExitCode {
    let register = match current_m5_truth_surface_ingestion() {
        Ok(register) => register,
        Err(err) => {
            eprintln!("failed to parse embedded register: {err}");
            return ExitCode::FAILURE;
        }
    };

    let args: Vec<String> = std::env::args().skip(1).collect();
    let command = args.first().map(String::as_str).unwrap_or("register");

    match command {
        "register" => {
            print_json(&register);
            ExitCode::SUCCESS
        }
        "validate" => {
            let violations = register.validate();
            if violations.is_empty() {
                println!("m5 truth-surface ingestion register: clean");
                ExitCode::SUCCESS
            } else {
                for violation in &violations {
                    eprintln!("VIOLATION: {violation:?}");
                }
                ExitCode::FAILURE
            }
        }
        "surface" => {
            let Some(name) = args.get(1) else {
                eprintln!("usage: surface <help_about|service_health|release_center|support_export|public_truth_pack>");
                return ExitCode::FAILURE;
            };
            let surface = match parse_surface(name) {
                Some(surface) => surface,
                None => {
                    eprintln!("unknown surface: {name}");
                    return ExitCode::FAILURE;
                }
            };
            let rows: Vec<_> = register.rows_for_surface(surface).collect();
            print_json(&rows);
            ExitCode::SUCCESS
        }
        other => {
            eprintln!("unknown command: {other}");
            ExitCode::FAILURE
        }
    }
}

fn parse_surface(name: &str) -> Option<TruthSurface> {
    match name {
        "help_about" => Some(TruthSurface::HelpAbout),
        "service_health" => Some(TruthSurface::ServiceHealth),
        "release_center" => Some(TruthSurface::ReleaseCenter),
        "support_export" => Some(TruthSurface::SupportExport),
        "public_truth_pack" => Some(TruthSurface::PublicTruthPack),
        _ => None,
    }
}

fn print_json<T: serde::Serialize>(value: &T) {
    match serde_json::to_string_pretty(value) {
        Ok(json) => println!("{json}"),
        Err(err) => eprintln!("failed to serialize: {err}"),
    }
}
