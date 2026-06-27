//! Headless emitter for the M5 runbook companion-scoped surface register.
//!
//! The bin is the only mint-from-truth path for the companion-register support export and Markdown
//! proof checked in under `artifacts/release/m5-runbook-proof/`, the published inventory at
//! `artifacts/runbooks/m5-runbook-companion-register.json`, and the per-surface fixtures under
//! `fixtures/runbooks/m5-companion-surfaces/`. The companion app, the desktop incident workspace
//! that receives a handoff, and support exports all read this one register, so a companion's
//! authority over a runbook step reads identically wherever it is rendered or exported: follow and
//! acknowledge are available within scope, a companion-allowed approval reuses the same desktop
//! approval/audit refs, and a blocked privileged mutate degrades to a clear desktop handoff.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_companion -- register
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_companion -- markdown
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_companion -- surface <step-id>
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_companion -- surfaces
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_companion -- validate
//! ```

use aureline_runbooks::m5_runbook_companion::{
    seeded_companion_surfaces, seeded_m5_runbook_companion_register, M5RunbookCompanionRegister,
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
        Some("register") | None => {
            let register = seeded_m5_runbook_companion_register();
            assert_valid(&register)?;
            println!("{}", register.export_safe_json());
        }
        Some("markdown") => {
            print!(
                "{}",
                seeded_m5_runbook_companion_register().render_markdown_summary()
            );
        }
        Some("surface") => {
            let id = args.get(1).map(String::as_str).unwrap_or("");
            let surface = seeded_companion_surfaces()
                .into_iter()
                .find(|s| s.step_id == id)
                .ok_or_else(|| format!("unknown step id: {id}"))?;
            let violations = surface.validate();
            if !violations.is_empty() {
                let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
                return Err(format!("surface {id} failed validation: {}", tokens.join(",")).into());
            }
            println!(
                "{}",
                serde_json::to_string_pretty(&surface).expect("surface serializes")
            );
        }
        Some("surfaces") => {
            for surface in seeded_companion_surfaces() {
                let violations = surface.validate();
                if !violations.is_empty() {
                    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
                    return Err(format!(
                        "surface {} failed validation: {}",
                        surface.step_id,
                        tokens.join(",")
                    )
                    .into());
                }
                println!("{}", surface.step_id);
            }
        }
        Some("validate") => {
            let register = seeded_m5_runbook_companion_register();
            assert_valid(&register)?;
            for surface in seeded_companion_surfaces() {
                let violations = surface.validate();
                if !violations.is_empty() {
                    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
                    return Err(format!(
                        "surface {} failed validation: {}",
                        surface.step_id,
                        tokens.join(",")
                    )
                    .into());
                }
            }
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(register: &M5RunbookCompanionRegister) -> Result<(), Box<dyn std::error::Error>> {
    let violations = register.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("companion register failed validation: {}", tokens.join(",")).into())
    }
}
