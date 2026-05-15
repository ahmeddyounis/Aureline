//! Headless inspector for the beta workspace-management projection.
//!
//! The bin emits the same beta records consumed by the live shell, by
//! the support-export wrapper, and by the integration test that
//! replays the checked-in fixtures.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_windows -- page
//! cargo run -q -p aureline-shell --bin aureline_shell_windows -- split
//! cargo run -q -p aureline-shell --bin aureline_shell_windows -- detach
//! cargo run -q -p aureline-shell --bin aureline_shell_windows -- move
//! cargo run -q -p aureline-shell --bin aureline_shell_windows -- restore
//! cargo run -q -p aureline-shell --bin aureline_shell_windows -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_windows -- validate
//! ```

use aureline_shell::windows::{
    seeded_windows_beta_page, validate_windows_beta_page, WindowsBetaSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_windows_beta_page();

    match args.first().map(String::as_str) {
        Some("page") | None => {
            print_json(&page)?;
        }
        Some("split") => {
            print_json(&page.split_intents)?;
        }
        Some("detach") => {
            print_json(&page.detach_intents)?;
        }
        Some("move") => {
            print_json(&page.move_intents)?;
        }
        Some("restore") => {
            print_json(&page.restore_outcomes)?;
        }
        Some("support-export") => {
            let export =
                WindowsBetaSupportExport::from_page("support-export:windows-beta:001", page);
            print_json(&export)?;
        }
        Some("validate") => match validate_windows_beta_page(&page) {
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
