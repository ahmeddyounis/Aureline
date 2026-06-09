//! Headless emitter for the M5 feature scorecard and compatibility-packet template register.
//!
//! Mints the on-disk fixtures under
//! `fixtures/release/m5/publish_feature_scorecard_and_compatibility_packet_templates_for_every_m5_family/`
//! so the release center, support export, and diagnostics surfaces all quote the same record
//! renders as the in-code corpus.
//!
//! Subcommands:
//!
//! ```sh
//! # Print the full register as JSON.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_publish_feature_scorecard_and_compatibility_packet_templates_for_every_m5_family -- register
//!
//! # Print the support-export projection as JSON.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_publish_feature_scorecard_and_compatibility_packet_templates_for_every_m5_family -- support-export
//!
//! # Refresh the on-disk fixtures.
//! cargo run -q -p aureline-release \
//!   --bin aureline_release_publish_feature_scorecard_and_compatibility_packet_templates_for_every_m5_family -- emit-fixtures \
//!   fixtures/release/m5/publish_feature_scorecard_and_compatibility_packet_templates_for_every_m5_family
//! ```

use std::path::PathBuf;

use aureline_release::publish_feature_scorecard_and_compatibility_packet_templates_for_every_m5_family::
    current_m5_template_register;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        None | Some("register") => {
            let reg = current_m5_template_register()?;
            println!("{}", serde_json::to_string_pretty(&reg)?);
            Ok(())
        }
        Some("support-export") => {
            let reg = current_m5_template_register()?;
            let projection = reg.support_export_projection();
            println!("{}", serde_json::to_string_pretty(&projection)?);
            Ok(())
        }
        Some("emit-fixtures") => {
            let dir = args
                .get(1)
                .ok_or("emit-fixtures requires a target directory argument")?;
            emit_fixtures(dir)
        }
        Some("validate") => {
            let reg = current_m5_template_register()?;
            let violations = reg.validate();
            if violations.is_empty() {
                println!("validate: clean (no violations)");
            } else {
                println!("validate: {} violation(s)", violations.len());
                for v in &violations {
                    println!("- {}", v);
                }
                std::process::exit(1);
            }
            Ok(())
        }
        Some(cmd) => Err(format!("unknown subcommand: {cmd}").into()),
    }
}

fn emit_fixtures(dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let base = PathBuf::from(dir);
    std::fs::create_dir_all(&base)?;

    let reg = current_m5_template_register()?;
    let violations = reg.validate();
    if !violations.is_empty() {
        return Err(format!("register has {} violation(s)", violations.len()).into());
    }

    std::fs::write(
        base.join("register.json"),
        serde_json::to_string_pretty(&reg)?,
    )?;

    let projection = reg.support_export_projection();
    std::fs::write(
        base.join("support_export_projection.json"),
        serde_json::to_string_pretty(&projection)?,
    )?;

    println!("emitted {} fixtures to {}", 2, base.display());
    Ok(())
}
