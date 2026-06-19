//! Headless emitter and validator for the guided-exercise-rail manifest.
//!
//! Emits the seeded manifest as JSON, validates it against the guided-exercise
//! rail invariants, and can refresh the on-disk fixture so it stays in sync with
//! the Rust types.
//!
//! ## Subcommands
//!
//! ```sh
//! # Print the full manifest as JSON.
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_guided_exercise_rails -- manifest
//!
//! # Print a plaintext rail summary.
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_guided_exercise_rails -- summary
//!
//! # Validate the seeded manifest (exit 0 = ok, exit 2 = failures).
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_guided_exercise_rails -- validate
//!
//! # Emit the on-disk fixture.
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_guided_exercise_rails \
//!   -- emit-fixture fixtures/help/m5/guided-exercise-rails/m5_guided_exercise_rails.json
//! ```

use std::path::PathBuf;

use aureline_learning::{
    seeded_m5_guided_exercise_rails, validate_m5_guided_exercise_rails,
    M5GuidedExerciseRailManifest, QualificationVerdict,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let manifest = seeded_m5_guided_exercise_rails();

    match args.first().map(String::as_str) {
        None | Some("manifest") => {
            println!("{}", serde_json::to_string_pretty(&manifest)?);
            Ok(())
        }
        Some("summary") => {
            print_summary(&manifest);
            Ok(())
        }
        Some("validate") => match validate_m5_guided_exercise_rails(&manifest) {
            Ok(()) => {
                println!("ok — all guided-exercise-rail invariants pass");
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

fn print_summary(manifest: &M5GuidedExerciseRailManifest) {
    println!("M5 guided-exercise-rail manifest: {}", manifest.manifest_id);
    println!("Generated: {}", manifest.generated_at);
    println!("Overall verdict: {}", manifest.overall_verdict.as_str());
    if !manifest.overall_narrowing_reasons.is_empty() {
        println!("Narrowing reasons:");
        for r in &manifest.overall_narrowing_reasons {
            println!("  - {r}");
        }
    }

    println!("\nRails ({}):", manifest.rails.len());
    for rail in &manifest.rails {
        let apply_steps = rail
            .steps
            .iter()
            .filter(|s| s.step_kind.is_apply_capable())
            .count();
        let sandbox_steps = rail
            .steps
            .iter()
            .filter(|s| {
                s.mutation_target == aureline_learning::MutationTarget::SandboxedLocalReversible
            })
            .count();
        println!(
            "  {} → {} [freshness={}, steps={}, apply={}, sandboxed={}, sandbox_available={}]",
            rail.rail_id,
            rail.verdict.as_str(),
            rail.freshness_state.as_str(),
            rail.steps.len(),
            apply_steps,
            sandbox_steps,
            rail.sandbox_preference.sandbox_available,
        );
    }

    if manifest.overall_verdict == QualificationVerdict::QualifiedStable {
        println!("\n✓ Every M5 family ships a Stable, command-backed guided-exercise rail.");
    } else {
        println!(
            "\n⚠ Overall: {} — one or more rails narrowed below Stable (cached/local-only, disclosed).",
            manifest.overall_verdict.as_str()
        );
    }
}
