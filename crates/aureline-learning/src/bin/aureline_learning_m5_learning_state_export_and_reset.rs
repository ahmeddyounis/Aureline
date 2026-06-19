//! Headless emitter and validator for the learning-state portability manifest.
//!
//! Emits the seeded manifest as JSON, validates it against the learning-state
//! portability invariants, and can refresh the on-disk fixture so it stays in
//! sync with the Rust types.
//!
//! ## Subcommands
//!
//! ```sh
//! # Print the full manifest as JSON.
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_state_export_and_reset -- manifest
//!
//! # Print a plaintext export/reset summary.
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_state_export_and_reset -- summary
//!
//! # Validate the seeded manifest (exit 0 = ok, exit 2 = failures).
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_state_export_and_reset -- validate
//!
//! # Emit the on-disk fixture.
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_state_export_and_reset \
//!   -- emit-fixture fixtures/help/m5/learning-state-export-and-reset/m5_learning_state_export_and_reset.json
//! ```

use std::path::PathBuf;

use aureline_learning::{
    seeded_m5_learning_state_export_and_reset, validate_m5_learning_state_export_and_reset,
    M5LearningStatePortabilityManifest, QualificationVerdict,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let manifest = seeded_m5_learning_state_export_and_reset();

    match args.first().map(String::as_str) {
        None | Some("manifest") => {
            println!("{}", serde_json::to_string_pretty(&manifest)?);
            Ok(())
        }
        Some("summary") => {
            print_summary(&manifest);
            Ok(())
        }
        Some("validate") => match validate_m5_learning_state_export_and_reset(&manifest) {
            Ok(()) => {
                println!("ok — all learning-state portability invariants pass");
                Ok(())
            }
            Err(errors) => {
                for e in &errors {
                    eprintln!("FAIL {e}");
                }
                Err(format!("{} validation error(s)", errors.len()).into())
            }
        },
        Some("emit-fixture") => {
            let path: PathBuf = args
                .get(1)
                .ok_or("emit-fixture requires a target path argument")?
                .into();
            let json = serde_json::to_string_pretty(&manifest)?;
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&path, format!("{json}\n"))?;
            println!("wrote {}", path.display());
            Ok(())
        }
        Some(unknown) => Err(format!("unknown subcommand: {unknown}").into()),
    }
}

fn print_summary(manifest: &M5LearningStatePortabilityManifest) {
    println!(
        "M5 learning-state portability manifest: {}",
        manifest.manifest_id
    );
    println!("Generated: {}", manifest.generated_at);
    println!("Overall verdict: {}", manifest.overall_verdict.as_str());
    if !manifest.overall_narrowing_reasons.is_empty() {
        println!("Narrowing reasons:");
        for r in &manifest.overall_narrowing_reasons {
            println!("  - {r}");
        }
    }

    println!("\nExport bundles ({}):", manifest.export_bundles.len());
    for bundle in &manifest.export_bundles {
        let escape = if bundle.source_language.presented_localized {
            format!(
                "{}→{} escape",
                bundle.source_language.source_locale, bundle.source_language.presented_locale
            )
        } else {
            "source language".to_string()
        };
        println!(
            "  {} → {} [family={}, state={}, target={}, freshness={}, lang={}]",
            bundle.bundle_id,
            bundle.verdict.as_str(),
            bundle.family.as_str(),
            bundle.state_kind.as_str(),
            bundle.target_kind.as_str(),
            bundle.cached_pack.freshness.as_str(),
            escape,
        );
        for reason in &bundle.narrowing_reasons {
            println!("      ! {reason}");
        }
    }

    println!("\nReset plans ({}):", manifest.reset_plans.len());
    for plan in &manifest.reset_plans {
        println!(
            "  {} → {} [clears={}, protects={}]",
            plan.plan_id,
            plan.verdict.as_str(),
            plan.target_state_kinds.len(),
            plan.protected_classes.len(),
        );
        for reason in &plan.narrowing_reasons {
            println!("      ! {reason}");
        }
    }

    if manifest.overall_verdict == QualificationVerdict::QualifiedStable {
        println!(
            "\n✓ Every export and reset is provenance-preserving, redacted, reversible, and fenced."
        );
    } else {
        println!(
            "\n⚠ Overall: {} — one or more bundles narrowed below Stable (disclosed).",
            manifest.overall_verdict.as_str()
        );
    }
}
