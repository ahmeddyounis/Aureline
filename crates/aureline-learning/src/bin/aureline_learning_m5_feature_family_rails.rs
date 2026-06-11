//! Headless emitter and validator for the M5 feature-family learning manifest.
//!
//! Emits the seeded manifest as JSON, validates it against the M5 learnability
//! invariants, and can refresh the on-disk fixture so it stays in sync with the
//! Rust types.
//!
//! ## Subcommands
//!
//! ```sh
//! # Print the full manifest as JSON.
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_feature_family_rails -- manifest
//!
//! # Print a plaintext learnability summary.
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_feature_family_rails -- summary
//!
//! # Validate the seeded manifest (exit 0 = ok, exit 2 = failures).
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_feature_family_rails -- validate
//!
//! # Emit the on-disk fixture.
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_feature_family_rails \
//!   -- emit-fixture fixtures/ux/m5/guided-tours/m5_feature_family_learning_manifest.json
//! ```

use std::path::PathBuf;

use aureline_learning::{
    seeded_m5_feature_family_learning_manifest, validate_m5_feature_family_learning,
    M5FeatureFamilyLearningManifest, QualificationVerdict,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let manifest = seeded_m5_feature_family_learning_manifest();

    match args.first().map(String::as_str) {
        None | Some("manifest") => {
            println!("{}", serde_json::to_string_pretty(&manifest)?);
            Ok(())
        }
        Some("summary") => {
            print_summary(&manifest);
            Ok(())
        }
        Some("validate") => match validate_m5_feature_family_learning(&manifest) {
            Ok(()) => {
                println!("ok — all M5 learning invariants pass");
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

fn print_summary(manifest: &M5FeatureFamilyLearningManifest) {
    println!(
        "M5 feature-family learning manifest: {}",
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

    println!("\nFamily bundles ({}):", manifest.family_bundles.len());
    for b in &manifest.family_bundles {
        println!(
            "  {} → {} [claimed={}, command_backed={}, help_cards={}, mirror_parity={}]",
            b.family.as_str(),
            b.verdict.as_str(),
            b.claimed,
            b.in_product_command_backed_path,
            b.contextual_help_cards.len(),
            b.mirror_parity.freshness_label,
        );
        for r in &b.narrowing_reasons {
            println!("      ⚠ {r}");
        }
    }

    println!("\nContract refs:");
    for (k, v) in &manifest.contract_refs {
        println!("  {k}: {v}");
    }

    if manifest.overall_verdict == QualificationVerdict::QualifiedStable {
        println!("\n✓ All claimed M5 families qualify Stable.");
    } else {
        println!(
            "\n⚠ Overall: {} — one or more families narrowed below Stable.",
            manifest.overall_verdict.as_str()
        );
    }
}
