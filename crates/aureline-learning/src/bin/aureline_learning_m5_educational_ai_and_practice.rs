//! Headless emitter and validator for the educational-AI and practice manifest.
//!
//! Emits the seeded manifest as JSON, validates it against the educational-AI and
//! practice invariants, and can refresh the on-disk fixture so it stays in sync
//! with the Rust types.
//!
//! ## Subcommands
//!
//! ```sh
//! # Print the full manifest as JSON.
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_educational_ai_and_practice -- manifest
//!
//! # Print a plaintext summary.
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_educational_ai_and_practice -- summary
//!
//! # Validate the seeded manifest (exit 0 = ok, exit 2 = failures).
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_educational_ai_and_practice -- validate
//!
//! # Emit the on-disk fixture.
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_educational_ai_and_practice \
//!   -- emit-fixture fixtures/help/m5/educational-ai-and-practice/m5_educational_ai_and_practice.json
//! ```

use std::path::PathBuf;

use aureline_learning::{
    seeded_m5_educational_ai_and_practice, validate_m5_educational_ai_and_practice,
    M5EducationalAiAndPracticeManifest, QualificationVerdict,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let manifest = seeded_m5_educational_ai_and_practice();

    match args.first().map(String::as_str) {
        None | Some("manifest") => {
            println!("{}", serde_json::to_string_pretty(&manifest)?);
            Ok(())
        }
        Some("summary") => {
            print_summary(&manifest);
            Ok(())
        }
        Some("validate") => match validate_m5_educational_ai_and_practice(&manifest) {
            Ok(()) => {
                println!("ok — all educational-AI and practice invariants pass");
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

fn print_summary(manifest: &M5EducationalAiAndPracticeManifest) {
    println!(
        "M5 educational-AI and practice manifest: {}",
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

    println!("\nEducational panels ({}):", manifest.panels.len());
    for panel in &manifest.panels {
        println!(
            "  {} → {} [family={}, kind={}, scope={}, citations={}, open_actions={}, explain_apply={}, offline={}]",
            panel.panel_id,
            panel.verdict.as_str(),
            panel.family.as_str(),
            panel.surface_kind.as_str(),
            panel.truth_source_scope.as_str(),
            panel.citations.len(),
            panel.open_resource_actions.len(),
            panel.explain_apply_class.as_str(),
            panel.offline_parity.as_str(),
        );
        for reason in &panel.narrowing_reasons {
            println!("      ! {reason}");
        }
    }

    println!(
        "\nPractice indicators ({}):",
        manifest.practice_indicators.len()
    );
    for indicator in &manifest.practice_indicators {
        println!(
            "  {} → {} [family={}, state={}, reset={}, mutates_live={}]",
            indicator.indicator_id,
            indicator.verdict.as_str(),
            indicator.family.as_str(),
            indicator.surface_state.as_str(),
            indicator.reset_behavior.as_str(),
            indicator.mutates_live_state,
        );
        for reason in &indicator.narrowing_reasons {
            println!("      ! {reason}");
        }
    }

    if manifest.overall_verdict == QualificationVerdict::QualifiedStable {
        println!("\n✓ Every panel is cited and scoped, and every practice space is distinct from live state.");
    } else {
        println!(
            "\n⚠ Overall: {} — one or more records narrowed below Stable (disclosed).",
            manifest.overall_verdict.as_str()
        );
    }
}
