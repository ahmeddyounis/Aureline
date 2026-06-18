//! Headless inspector for the M5 imported-theme mapping & rollback report.
//!
//! The bin emits the same packet records consumed by the live migration center,
//! by the support-export wrapper, by the docs page, by the published report
//! artifact, and by the integration that replays the checked-in fixtures.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_theme_import_reports -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_theme_import_reports -- rows
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_theme_import_reports -- outcomes
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_theme_import_reports -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_theme_import_reports -- validate
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_theme_import_reports -- compact
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_theme_import_reports -- markdown
//! ```

use aureline_shell::theme_import_reports::{
    seeded_theme_import_report, validate_theme_import_report, ThemeImportSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let report = seeded_theme_import_report();

    match args.first().map(String::as_str) {
        Some("report") | None => {
            print_json(&report)?;
        }
        Some("rows") => {
            print_json(&report.rows)?;
        }
        Some("outcomes") => {
            print_json(&report.outcome_summary)?;
        }
        Some("support-export") => {
            let export = ThemeImportSupportExport::from_report(
                "support-export:m5-theme-import-reports:001",
                report,
            );
            print_json(&export)?;
        }
        Some("validate") => match validate_theme_import_report(&report) {
            Ok(()) => {
                println!("ok");
            }
            Err(errors) => {
                for err in &errors {
                    eprintln!(
                        "error: {}",
                        serde_json::to_string(err).unwrap_or_else(|_| format!("{err:?}"))
                    );
                }
                std::process::exit(3);
            }
        },
        Some("compact") => {
            for line in report.compact_lines() {
                println!("{line}");
            }
        }
        Some("markdown") => {
            print!("{}", report.render_markdown());
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
