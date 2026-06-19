//! Headless emitter and validator for the learning-mode-profile manifest.
//!
//! Emits the seeded manifest as JSON, validates it against the learning-mode
//! profile invariants, and can refresh the on-disk fixture so it stays in sync
//! with the Rust types.
//!
//! ## Subcommands
//!
//! ```sh
//! # Print the full manifest as JSON.
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_mode_profiles -- manifest
//!
//! # Print a plaintext profile summary.
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_mode_profiles -- summary
//!
//! # Validate the seeded manifest (exit 0 = ok, exit 2 = failures).
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_mode_profiles -- validate
//!
//! # Emit the on-disk fixture.
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_mode_profiles \
//!   -- emit-fixture fixtures/help/m5/learning-mode-profiles/m5_learning_mode_profiles.json
//! ```

use std::path::PathBuf;

use aureline_learning::{
    seeded_m5_learning_mode_profiles, validate_m5_learning_mode_profiles,
    M5LearningModeProfileManifest, QualificationVerdict,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let manifest = seeded_m5_learning_mode_profiles();

    match args.first().map(String::as_str) {
        None | Some("manifest") => {
            println!("{}", serde_json::to_string_pretty(&manifest)?);
            Ok(())
        }
        Some("summary") => {
            print_summary(&manifest);
            Ok(())
        }
        Some("validate") => match validate_m5_learning_mode_profiles(&manifest) {
            Ok(()) => {
                println!("ok — all learning-mode-profile invariants pass");
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

fn print_summary(manifest: &M5LearningModeProfileManifest) {
    println!(
        "M5 learning-mode-profile manifest: {}",
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

    println!("\nProfiles ({}):", manifest.profiles.len());
    for profile in &manifest.profiles {
        println!(
            "  {} → {} [preset={}, scope={}, tips={}, jargon={}, ai={}, guardrail={}, sync={}]",
            profile.profile_id,
            profile.verdict.as_str(),
            profile.preset.as_str(),
            profile.scope_binding.scope.as_str(),
            profile.tip_intensity.as_str(),
            profile.jargon_level.as_str(),
            profile.ai_explanation_posture.as_str(),
            profile.mutation_guardrail.as_str(),
            profile.sync_posture.as_str(),
        );
        for reason in &profile.narrowing_reasons {
            println!("      ! {reason}");
        }
    }

    if manifest.overall_verdict == QualificationVerdict::QualifiedStable {
        println!("\n✓ Every learning-mode profile is Stable, user-owned, and reversible.");
    } else {
        println!(
            "\n⚠ Overall: {} — one or more profiles narrowed below Stable (disclosed).",
            manifest.overall_verdict.as_str()
        );
    }
}
