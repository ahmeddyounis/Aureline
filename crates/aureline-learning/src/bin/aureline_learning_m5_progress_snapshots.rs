//! Headless emitter and validator for the learning-progress manifest.
//!
//! Emits the seeded manifest as JSON, validates it against the learning-progress
//! invariants, and can refresh the on-disk fixture so it stays in sync with the
//! Rust types.
//!
//! ## Subcommands
//!
//! ```sh
//! # Print the full manifest as JSON.
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_progress_snapshots -- manifest
//!
//! # Print a plaintext progress summary.
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_progress_snapshots -- summary
//!
//! # Validate the seeded manifest (exit 0 = ok, exit 2 = failures).
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_progress_snapshots -- validate
//!
//! # Emit the on-disk fixture.
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_progress_snapshots \
//!   -- emit-fixture fixtures/help/m5/learning-progress/m5_learning_progress_snapshots.json
//! ```

use std::path::PathBuf;

use aureline_learning::{
    seeded_m5_learning_progress_snapshots, validate_m5_learning_progress_snapshots,
    M5LearningProgressManifest, QualificationVerdict,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let manifest = seeded_m5_learning_progress_snapshots();

    match args.first().map(String::as_str) {
        None | Some("manifest") => {
            println!("{}", serde_json::to_string_pretty(&manifest)?);
            Ok(())
        }
        Some("summary") => {
            print_summary(&manifest);
            Ok(())
        }
        Some("validate") => match validate_m5_learning_progress_snapshots(&manifest) {
            Ok(()) => {
                println!("ok — all learning-progress invariants pass");
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

fn print_summary(manifest: &M5LearningProgressManifest) {
    println!("M5 learning-progress manifest: {}", manifest.manifest_id);
    println!("Generated: {}", manifest.generated_at);
    println!("Overall verdict: {}", manifest.overall_verdict.as_str());
    if !manifest.overall_narrowing_reasons.is_empty() {
        println!("Narrowing reasons:");
        for r in &manifest.overall_narrowing_reasons {
            println!("  - {r}");
        }
    }

    println!("\nSnapshots ({}):", manifest.snapshots.len());
    for snapshot in &manifest.snapshots {
        let resume = snapshot
            .resume_point
            .as_ref()
            .map(|r| r.step_ref.as_str())
            .unwrap_or("—");
        println!(
            "  {} → {} [family={}, flow={}, disclosure={}, sync={}, completed={}/{}, dismissed={}, resume_at={}]",
            snapshot.snapshot_id,
            snapshot.verdict.as_str(),
            snapshot.family.as_str(),
            snapshot.flow_kind.as_str(),
            snapshot.disclosure_state.as_str(),
            snapshot.sync_policy.as_str(),
            snapshot.completed_step_count(),
            snapshot.steps.len(),
            snapshot.dismissed_step_count(),
            resume,
        );
        for reason in &snapshot.narrowing_reasons {
            println!("      ! {reason}");
        }
    }

    println!("\nDigests ({}):", manifest.digests.len());
    for digest in &manifest.digests {
        println!(
            "  {} → {} [covers={}, actions={}]",
            digest.digest_id,
            digest.verdict.as_str(),
            digest.covered_snapshot_refs.len(),
            digest.actions.len(),
        );
        for reason in &digest.narrowing_reasons {
            println!("      ! {reason}");
        }
    }

    if manifest.overall_verdict == QualificationVerdict::QualifiedStable {
        println!("\n✓ Every progress snapshot is Stable, user-owned, and durably recoverable.");
    } else {
        println!(
            "\n⚠ Overall: {} — one or more snapshots narrowed below Stable (disclosed).",
            manifest.overall_verdict.as_str()
        );
    }
}
