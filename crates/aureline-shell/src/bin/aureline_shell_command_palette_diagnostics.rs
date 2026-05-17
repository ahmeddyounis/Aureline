//! Headless inspector for beta command-palette diagnostics.
//!
//! The binary emits the deterministic diagnostics pack, its redacted support
//! export, and the palette parity examples artifact consumed by docs and
//! fixture tests.
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_command_palette_diagnostics -- pack
//! cargo run -q -p aureline-shell --bin aureline_shell_command_palette_diagnostics -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_command_palette_diagnostics -- parity-examples
//! cargo run -q -p aureline-shell --bin aureline_shell_command_palette_diagnostics -- validate
//! ```

use aureline_shell::palette::{
    seeded_beta_command_palette_diagnostics_pack,
    seeded_beta_command_palette_diagnostics_support_export,
    seeded_beta_palette_parity_examples_artifact, validate_beta_command_palette_diagnostics_pack,
    validate_beta_command_palette_diagnostics_support_export,
    validate_beta_palette_parity_examples_artifact,
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
        Some("pack") | None => print_json(&seeded_beta_command_palette_diagnostics_pack())?,
        Some("support-export") => {
            print_json(&seeded_beta_command_palette_diagnostics_support_export())?
        }
        Some("parity-examples") => print_json(&seeded_beta_palette_parity_examples_artifact())?,
        Some("validate") => validate_seeded_records()?,
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn validate_seeded_records() -> Result<(), Box<dyn std::error::Error>> {
    let pack = seeded_beta_command_palette_diagnostics_pack();
    let support_export = seeded_beta_command_palette_diagnostics_support_export();
    let parity_examples = seeded_beta_palette_parity_examples_artifact();

    match (
        validate_beta_command_palette_diagnostics_pack(&pack),
        validate_beta_command_palette_diagnostics_support_export(&support_export),
        validate_beta_palette_parity_examples_artifact(&parity_examples),
    ) {
        (Ok(()), Ok(()), Ok(())) => {
            println!("ok");
            Ok(())
        }
        (pack_result, support_result, parity_result) => {
            if let Err(errors) = pack_result {
                for err in errors {
                    eprintln!("pack error: {err}");
                }
            }
            if let Err(errors) = support_result {
                for err in errors {
                    eprintln!("support-export error: {err}");
                }
            }
            if let Err(errors) = parity_result {
                for err in errors {
                    eprintln!("parity-examples error: {err}");
                }
            }
            std::process::exit(3);
        }
    }
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(value)?;
    println!("{json}");
    Ok(())
}
