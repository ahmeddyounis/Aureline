//! Headless inspector for shared document-identity disclosures.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_document_identity_truth -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_document_identity_truth -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_document_identity_truth -- compact
//! cargo run -q -p aureline-shell --bin aureline_shell_document_identity_truth -- validate
//! ```

use aureline_shell::document_identity::seeded_document_identity_report;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let report = seeded_document_identity_report();

    match args.first().map(String::as_str) {
        Some("report") | None => print_json(&report)?,
        Some("support-export") => print_json(&report.support_export())?,
        Some("compact") => {
            for line in report.compact_lines() {
                println!("{line}");
            }
        }
        Some("validate") => match report.validate() {
            Ok(()) => println!("ok"),
            Err(findings) => {
                for finding in findings {
                    eprintln!(
                        "{}",
                        serde_json::to_string(&finding)
                            .unwrap_or_else(|_| "document identity finding".to_owned())
                    );
                }
                std::process::exit(3);
            }
        },
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
