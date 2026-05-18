//! Headless minter for the command-reference catalog.
//!
//! The bin emits the same record consumed by the live shell
//! command-detail surface, the markdown parity report under
//! `artifacts/ux/m3/command_reference_parity_report.md`, and the
//! beta contract doc under
//! `docs/ux/m3/command_reference_beta_contract.md`. It is the only
//! mint-from-truth path for the JSON fixtures checked in under
//! `fixtures/ux/m3/command_reference_and_discoverability/`.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_command_reference -- catalog
//! cargo run -q -p aureline-shell --bin aureline_shell_command_reference -- report-md
//! cargo run -q -p aureline-shell --bin aureline_shell_command_reference -- validate
//! ```

use aureline_shell::command_reference::{
    render_catalog_markdown, seeded_command_reference_catalog, validate_command_reference_catalog,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let catalog = seeded_command_reference_catalog();

    match args.first().map(String::as_str) {
        Some("catalog") | None => {
            let json = serde_json::to_string_pretty(&catalog)?;
            println!("{json}");
        }
        Some("report-md") => {
            print!("{}", render_catalog_markdown(&catalog));
        }
        Some("validate") => match validate_command_reference_catalog(&catalog) {
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
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}
