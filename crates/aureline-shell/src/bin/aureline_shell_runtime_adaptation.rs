//! Headless inspector for the beta runtime-adaptation projection.
//!
//! The bin emits the same beta records consumed by the live shell, by
//! the support-export wrapper, and by the integration test that
//! replays the checked-in fixtures.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_runtime_adaptation -- page
//! cargo run -q -p aureline-shell --bin aureline_shell_runtime_adaptation -- posture
//! cargo run -q -p aureline-shell --bin aureline_shell_runtime_adaptation -- suspend
//! cargo run -q -p aureline-shell --bin aureline_shell_runtime_adaptation -- monitor
//! cargo run -q -p aureline-shell --bin aureline_shell_runtime_adaptation -- foreground
//! cargo run -q -p aureline-shell --bin aureline_shell_runtime_adaptation -- matrix
//! cargo run -q -p aureline-shell --bin aureline_shell_runtime_adaptation -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_runtime_adaptation -- validate
//! ```

use aureline_shell::runtime_adaptation::{
    seeded_runtime_adaptation_page, validate_runtime_adaptation_page,
    RuntimeAdaptationSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_runtime_adaptation_page();

    match args.first().map(String::as_str) {
        Some("page") | None => {
            print_json(&page)?;
        }
        Some("posture") => {
            print_json(&page.power_posture_rows)?;
        }
        Some("suspend") => {
            print_json(&page.suspend_resume_rows)?;
        }
        Some("monitor") => {
            print_json(&page.monitor_continuity_rows)?;
        }
        Some("foreground") => {
            print_json(&page.foreground_protection_rows)?;
        }
        Some("matrix") => {
            print_json(&page.desktop_matrix_rows)?;
        }
        Some("support-export") => {
            let export = RuntimeAdaptationSupportExport::from_page(
                "support-export:runtime-adaptation:001",
                page,
            );
            print_json(&export)?;
        }
        Some("validate") => match validate_runtime_adaptation_page(&page) {
            Ok(()) => {
                println!("ok");
            }
            Err(errors) => {
                for err in &errors {
                    eprintln!("error: {err}");
                }
                std::process::exit(3);
            }
        },
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
