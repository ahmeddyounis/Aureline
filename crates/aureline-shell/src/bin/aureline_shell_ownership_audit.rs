//! Headless inspector for the desktop-entry ownership audit packet.
//!
//! The bin reads the same protected fixture consumed by the shell
//! module, the install crate's fixture tests, and the support-export
//! wrapper. It exists so reviewers and support tooling can inspect the
//! audit without standing up the live shell.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_ownership_audit -- packet
//! cargo run -q -p aureline-shell --bin aureline_shell_ownership_audit -- rows
//! cargo run -q -p aureline-shell --bin aureline_shell_ownership_audit -- surface
//! cargo run -q -p aureline-shell --bin aureline_shell_ownership_audit -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_ownership_audit -- validate
//! ```

use aureline_shell::ownership_audit::{
    load_seeded_ownership_audit_packet, seeded_ownership_audit_support_export,
    seeded_ownership_audit_surface_projection, validate_seeded_ownership_audit_packet,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("packet") | None => {
            let packet = load_seeded_ownership_audit_packet()?;
            print_json(&packet)?;
        }
        Some("rows") => {
            let packet = load_seeded_ownership_audit_packet()?;
            print_json(&packet.rows)?;
        }
        Some("surface") => {
            let surface = seeded_ownership_audit_surface_projection()?;
            print_json(&surface)?;
        }
        Some("support-export") => {
            let export = seeded_ownership_audit_support_export()?;
            print_json(&export)?;
        }
        Some("validate") => {
            let report = validate_seeded_ownership_audit_packet()?;
            if report.passed {
                println!("ok");
            } else {
                for finding in &report.findings {
                    eprintln!(
                        "error: {check} ({ref_id}): {message}",
                        check = finding.check_id,
                        ref_id = finding.ref_id,
                        message = finding.message,
                    );
                }
                std::process::exit(3);
            }
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(value)?;
    println!("{json}");
    Ok(())
}
