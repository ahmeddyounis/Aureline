//! Headless emitter for the AI copy guardrail catalog.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/content/m5-ai-copy-proof/` and the localized / offline-mirror fixtures
//! under `fixtures/content/m5-ai-copy-guardrails/`. Prompt composer, patch review,
//! notebook help, docs/help, and provider/account surfaces resolve controlled AI
//! wording through this catalog, and route candidate copy through its lint, so a
//! literal overclaiming string never becomes the source of truth.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_ai_copy_guardrails -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_ai_copy_guardrails -- markdown
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_ai_copy_guardrails -- fixture-localized
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_ai_copy_guardrails -- fixture-offline-mirror
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_ai_copy_guardrails -- lint "<candidate copy>"
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_ai_copy_guardrails -- validate
//! ```

use aureline_shell::content::ai_copy_guardrails::{
    seeded_ai_copy_guardrail_catalog, seeded_ai_copy_guardrail_catalog_localized,
    seeded_ai_copy_guardrail_catalog_offline_mirror, AiCopyGuardrailCatalog, AiCopySurface,
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
        Some("support-export") | None => {
            let catalog = seeded_ai_copy_guardrail_catalog();
            assert_valid(&catalog)?;
            println!("{}", catalog.export_safe_json());
        }
        Some("markdown") => {
            print!(
                "{}",
                seeded_ai_copy_guardrail_catalog().render_markdown_summary()
            );
        }
        Some("fixture-localized") => {
            let catalog = seeded_ai_copy_guardrail_catalog_localized();
            assert_valid(&catalog)?;
            println!("{}", catalog.export_safe_json());
        }
        Some("fixture-offline-mirror") => {
            let catalog = seeded_ai_copy_guardrail_catalog_offline_mirror();
            assert_valid(&catalog)?;
            println!("{}", catalog.export_safe_json());
        }
        Some("lint") => {
            let candidate = args.get(1).map(String::as_str).unwrap_or_default();
            let catalog = seeded_ai_copy_guardrail_catalog();
            let mut any = false;
            for surface in AiCopySurface::ALL {
                for finding in catalog.lint(candidate, surface) {
                    any = true;
                    println!(
                        "{}\t{}\t{}\t{}\t{}",
                        surface.as_str(),
                        finding.class.as_str(),
                        finding.phrase_id,
                        finding.matched_pattern,
                        finding.approved_replacement_term_ids.join(",")
                    );
                }
            }
            if !any {
                println!("clean");
            } else {
                std::process::exit(1);
            }
        }
        Some("validate") => {
            for catalog in [
                seeded_ai_copy_guardrail_catalog(),
                seeded_ai_copy_guardrail_catalog_localized(),
                seeded_ai_copy_guardrail_catalog_offline_mirror(),
            ] {
                assert_valid(&catalog)?;
            }
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(catalog: &AiCopyGuardrailCatalog) -> Result<(), Box<dyn std::error::Error>> {
    let violations = catalog.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("catalog failed validation: {}", tokens.join(",")).into())
    }
}
