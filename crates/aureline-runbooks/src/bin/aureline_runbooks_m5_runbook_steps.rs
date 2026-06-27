//! Headless emitter for the M5 runbook executable step library.
//!
//! The bin is the only mint-from-truth path for the step-library support export and Markdown proof
//! checked in under `artifacts/release/m5-runbook-proof/`, the published inventory at
//! `artifacts/runbooks/m5-runbook-step-library.json`, the companion follow-view fixture, and the
//! per-step fixtures under `fixtures/runbooks/m5-step-library/`. The desktop runbook/incident UI,
//! companion follow views, and support exports all read this one library, so every executable step
//! shows the same class, target-selector scope, approval requirement, execution mode, and expected
//! evidence wherever it is previewed, executed, followed, or exported, and no step mints a hidden
//! privileged mutate channel.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_steps -- library
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_steps -- markdown
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_steps -- fixture-companion-scoped
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_steps -- step <id>
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_steps -- steps
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_steps -- validate
//! ```

use aureline_runbooks::m5_runbook_steps::{
    seeded_executable_steps, seeded_m5_runbook_step_library,
    seeded_m5_runbook_step_library_companion_scoped, M5RunbookStepLibrary,
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
        Some("library") | None => {
            let library = seeded_m5_runbook_step_library();
            assert_valid(&library)?;
            println!("{}", library.export_safe_json());
        }
        Some("markdown") => {
            print!(
                "{}",
                seeded_m5_runbook_step_library().render_markdown_summary()
            );
        }
        Some("fixture-companion-scoped") => {
            let library = seeded_m5_runbook_step_library_companion_scoped();
            assert_valid(&library)?;
            println!("{}", library.export_safe_json());
        }
        Some("step") => {
            let id = args.get(1).map(String::as_str).unwrap_or("");
            let step = seeded_executable_steps()
                .into_iter()
                .find(|s| s.step_id == id)
                .ok_or_else(|| format!("unknown step id: {id}"))?;
            let violations = step.validate();
            if !violations.is_empty() {
                let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
                return Err(format!("step {id} failed validation: {}", tokens.join(",")).into());
            }
            println!(
                "{}",
                serde_json::to_string_pretty(&step).expect("step serializes")
            );
        }
        Some("steps") => {
            for step in seeded_executable_steps() {
                let violations = step.validate();
                if !violations.is_empty() {
                    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
                    return Err(format!(
                        "step {} failed validation: {}",
                        step.step_id,
                        tokens.join(",")
                    )
                    .into());
                }
                println!("{}", step.step_id);
            }
        }
        Some("validate") => {
            for library in [
                seeded_m5_runbook_step_library(),
                seeded_m5_runbook_step_library_companion_scoped(),
            ] {
                assert_valid(&library)?;
            }
            for step in seeded_executable_steps() {
                let violations = step.validate();
                if !violations.is_empty() {
                    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
                    return Err(format!(
                        "step {} failed validation: {}",
                        step.step_id,
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

fn assert_valid(library: &M5RunbookStepLibrary) -> Result<(), Box<dyn std::error::Error>> {
    let violations = library.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("step library failed validation: {}", tokens.join(",")).into())
    }
}
