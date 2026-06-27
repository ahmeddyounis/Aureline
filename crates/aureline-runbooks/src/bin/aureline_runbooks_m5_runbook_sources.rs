//! Headless emitter for the M5 runbook source register.
//!
//! The bin is the only mint-from-truth path for the source-register support export and Markdown
//! proof checked in under `artifacts/release/m5-runbook-proof/`, the published inventory at
//! `artifacts/runbooks/m5-runbook-source-register.json`, the stale-mirror drill fixture, and the
//! per-source descriptor fixtures under `fixtures/runbooks/m5-source-descriptors/`. The docs/help
//! runbook browser, the incident workspace, operator dashboards, and support exports all read this
//! one register, so every runbook source shows the same provenance class, authority posture,
//! freshness, signer summary, and version wherever it is rendered or exported.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_sources -- register
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_sources -- markdown
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_sources -- fixture-stale-mirror
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_sources -- source <id>
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_sources -- sources
//! cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_sources -- validate
//! ```

use aureline_runbooks::m5_runbook_sources::{
    seeded_m5_runbook_source_register, seeded_m5_runbook_source_register_stale_mirror_narrowed,
    seeded_runbook_sources, M5RunbookSourceRegister,
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
            let register = seeded_m5_runbook_source_register();
            assert_valid(&register)?;
            println!("{}", register.export_safe_json());
        }
        Some("markdown") => {
            print!(
                "{}",
                seeded_m5_runbook_source_register().render_markdown_summary()
            );
        }
        Some("fixture-stale-mirror") => {
            let register = seeded_m5_runbook_source_register_stale_mirror_narrowed();
            assert_valid(&register)?;
            println!("{}", register.export_safe_json());
        }
        Some("source") => {
            let id = args.get(1).map(String::as_str).unwrap_or("");
            let source = seeded_runbook_sources()
                .into_iter()
                .find(|s| s.source_id == id)
                .ok_or_else(|| format!("unknown source id: {id}"))?;
            let violations = source.validate();
            if !violations.is_empty() {
                let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
                return Err(format!("source {id} failed validation: {}", tokens.join(",")).into());
            }
            println!(
                "{}",
                serde_json::to_string_pretty(&source).expect("source serializes")
            );
        }
        Some("sources") => {
            for source in seeded_runbook_sources() {
                let violations = source.validate();
                if !violations.is_empty() {
                    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
                    return Err(format!(
                        "source {} failed validation: {}",
                        source.source_id,
                        tokens.join(",")
                    )
                    .into());
                }
                println!("{}", source.source_id);
            }
        }
        Some("validate") => {
            for register in [
                seeded_m5_runbook_source_register(),
                seeded_m5_runbook_source_register_stale_mirror_narrowed(),
            ] {
                assert_valid(&register)?;
            }
            for source in seeded_runbook_sources() {
                let violations = source.validate();
                if !violations.is_empty() {
                    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
                    return Err(format!(
                        "source {} failed validation: {}",
                        source.source_id,
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

fn assert_valid(register: &M5RunbookSourceRegister) -> Result<(), Box<dyn std::error::Error>> {
    let violations = register.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("source register failed validation: {}", tokens.join(",")).into())
    }
}
