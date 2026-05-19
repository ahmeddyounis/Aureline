//! Headless minter for the command-forms catalog.
//!
//! Emits the same record consumed by the live shell parameter-form host,
//! the CLI inspect surface, the AI tool envelope, automation-recipe step
//! editors, request / run / debug / template / repair workspaces, and the
//! markdown report under
//! `docs/ux/m3/command_parameter_form_beta.md`. This bin is the only
//! mint-from-truth path for the JSON fixtures checked in under
//! `fixtures/ux/m3/parameter_forms_and_invocation_review/`.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_command_forms -- catalog
//! cargo run -q -p aureline-shell --bin aureline_shell_command_forms -- report-md
//! cargo run -q -p aureline-shell --bin aureline_shell_command_forms -- validate
//! ```

use aureline_shell::command_forms::{
    render_catalog_markdown, seeded_command_forms_catalog, validate_command_forms_catalog,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let catalog = seeded_command_forms_catalog();

    match args.first().map(String::as_str) {
        Some("catalog") | None => {
            let json = serde_json::to_string_pretty(&catalog)?;
            println!("{json}");
        }
        Some("report-md") => {
            print!("{}", render_catalog_markdown(&catalog));
        }
        Some("validate") => match validate_command_forms_catalog(&catalog) {
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
